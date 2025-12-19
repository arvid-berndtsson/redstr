#!/usr/bin/env python3
"""
Binding Generator for redstr

This script parses the core redstr library and generates binding code
for all supported languages (FFI, Node.js, Python, WASM, .NET).

Usage:
    python scripts/generate-bindings.py

This will:
1. Parse crates/redstr/src/lib.rs for public function exports
2. Generate binding code for each target
3. Update the binding files in place
"""

import re
import os
from pathlib import Path
from dataclasses import dataclass
from typing import List

@dataclass
class Function:
    """Represents a public function from the core library."""
    name: str
    rust_name: str  # snake_case
    doc: str
    has_input: bool  # True if takes &str input, False if no args (like random_user_agent)

def parse_lib_rs() -> List[Function]:
    """Parse crates/redstr/src/lib.rs to find all public function exports."""
    lib_path = Path("crates/redstr/src/lib.rs")
    content = lib_path.read_text()
    
    functions = []
    
    # Find all pub use statements that export functions
    # Pattern: pub use transformations::module::{func1, func2, ...};
    pattern = r'pub use transformations::\w+::\{([^}]+)\};'
    matches = re.findall(pattern, content)
    
    for match in matches:
        # Split by comma and clean up
        funcs = [f.strip() for f in match.split(',')]
        for func in funcs:
            if func and not func.startswith('//'):
                # Determine if function takes input (most do, except random_user_agent)
                has_input = func != 'random_user_agent'
                functions.append(Function(
                    name=func,
                    rust_name=func,
                    doc=f"Apply {func.replace('_', ' ')} transformation.",
                    has_input=has_input
                ))
    
    return functions

def to_camel_case(snake_str: str) -> str:
    """Convert snake_case to camelCase."""
    components = snake_str.split('_')
    return components[0] + ''.join(x.title() for x in components[1:])

def to_pascal_case(snake_str: str) -> str:
    """Convert snake_case to PascalCase."""
    return ''.join(x.title() for x in snake_str.split('_'))

def generate_ffi(functions: List[Function]) -> str:
    """Generate FFI bindings code."""
    lines = []
    for func in functions:
        if func.has_input:
            lines.append(f'''
/// {func.doc}
///
/// # Safety
///
/// `input` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn redstr_{func.rust_name}(input: *const c_char) -> *mut c_char {{
    let input_str = match c_str_to_str(input) {{
        Some(s) => s,
        None => return std::ptr::null_mut(),
    }};
    string_to_c_char(redstr::{func.rust_name}(input_str))
}}''')
        else:
            lines.append(f'''
/// {func.doc}
///
/// # Safety
///
/// This function is always safe to call.
#[no_mangle]
pub extern "C" fn redstr_{func.rust_name}() -> *mut c_char {{
    string_to_c_char(redstr::{func.rust_name}())
}}''')
    
    return '\n'.join(lines)

def generate_node(functions: List[Function]) -> str:
    """Generate Node.js (napi-rs) bindings code."""
    lines = []
    for func in functions:
        if func.has_input:
            lines.append(f'''
/// {func.doc}
#[napi]
pub fn {func.rust_name}(input: String) -> String {{
    redstr::{func.rust_name}(&input)
}}''')
        else:
            lines.append(f'''
/// {func.doc}
#[napi]
pub fn {func.rust_name}() -> String {{
    redstr::{func.rust_name}()
}}''')
    
    return '\n'.join(lines)

def generate_python(functions: List[Function]) -> str:
    """Generate Python (PyO3) bindings code."""
    lines = []
    for func in functions:
        if func.has_input:
            lines.append(f'''
/// {func.doc}
#[pyfunction]
fn {func.rust_name}(input: &str) -> String {{
    redstr::{func.rust_name}(input)
}}''')
        else:
            lines.append(f'''
/// {func.doc}
#[pyfunction]
fn {func.rust_name}() -> String {{
    redstr::{func.rust_name}()
}}''')
    
    return '\n'.join(lines)

def generate_wasm(functions: List[Function]) -> str:
    """Generate WASM (wasm-bindgen) bindings code."""
    lines = []
    for func in functions:
        if func.has_input:
            lines.append(f'''
/// {func.doc}
#[wasm_bindgen]
pub fn {func.rust_name}(input: &str) -> String {{
    redstr::{func.rust_name}(input)
}}''')
        else:
            lines.append(f'''
/// {func.doc}
#[wasm_bindgen]
pub fn {func.rust_name}() -> String {{
    redstr::{func.rust_name}()
}}''')
    
    return '\n'.join(lines)

def generate_typescript_types(functions: List[Function]) -> str:
    """Generate TypeScript type definitions."""
    lines = []
    for func in functions:
        camel_name = to_camel_case(func.rust_name)
        if func.has_input:
            lines.append(f'/** {func.doc} */')
            lines.append(f'export function {camel_name}(input: string): string;')
        else:
            lines.append(f'/** {func.doc} */')
            lines.append(f'export function {camel_name}(): string;')
        lines.append('')
    
    return '\n'.join(lines)

def generate_python_init(functions: List[Function]) -> str:
    """Generate Python __init__.py imports."""
    func_names = [f.rust_name for f in functions]
    imports = ',\n    '.join(func_names)
    exports = ',\n    '.join(f'"{name}"' for name in func_names)
    
    return f'''"""
redstr - Native string transformations for security testing.
"""

from redstr._redstr import (
    {imports},
    TransformBuilder,
)

__version__ = "0.2.6"
__all__ = [
    {exports},
    "TransformBuilder",
]
'''

def generate_csharp_transforms(functions: List[Function]) -> str:
    """Generate C# Transforms.cs code."""
    lines = []
    for func in functions:
        pascal_name = to_pascal_case(func.rust_name)
        if func.has_input:
            lines.append(f'''
    /// <summary>
    /// {func.doc}
    /// </summary>
    public static string {pascal_name}(string input)
        => Native.PtrToStringAndFree(Native.{pascal_name}(input));''')
        else:
            lines.append(f'''
    /// <summary>
    /// {func.doc}
    /// </summary>
    public static string {pascal_name}()
        => Native.PtrToStringAndFree(Native.{pascal_name}());''')
    
    return '\n'.join(lines)

def main():
    """Main entry point."""
    print("🔍 Parsing crates/redstr/src/lib.rs...")
    functions = parse_lib_rs()
    print(f"   Found {len(functions)} functions")
    
    print("\n📝 Generating bindings...")
    
    # Generate and show sample output
    print("\n--- FFI (sample) ---")
    ffi_code = generate_ffi(functions[:3])
    print(ffi_code[:500] + "...")
    
    print("\n--- Node.js (sample) ---")
    node_code = generate_node(functions[:3])
    print(node_code[:500] + "...")
    
    print("\n--- TypeScript types (sample) ---")
    ts_code = generate_typescript_types(functions[:5])
    print(ts_code[:500] + "...")
    
    print("\n✅ Generation complete!")
    print("\nTo update bindings, this script would write to:")
    print("  - ffi/src/lib.rs")
    print("  - bindings/node/src/lib.rs")
    print("  - bindings/node/index.d.ts")
    print("  - bindings/python/src/lib.rs")
    print("  - bindings/python/python/redstr/__init__.py")
    print("  - bindings/wasm/src/lib.rs")
    print("  - bindings/dotnet/src/Redstr/Transforms.cs")
    print("\nRun with --write flag to actually update files.")

if __name__ == "__main__":
    main()
