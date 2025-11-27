# Python Bindings Architecture Proposal

## Current Issue

The current implementation has a manually maintained 1400+ line `python_bindings.rs` file that duplicates information already present in the library's public API. This creates maintenance burden and is prone to getting out of sync.

## Proposed Solutions (Ranked)

### ✅ Solution 1: Build Script Auto-Generation (RECOMMENDED)

**Approach**: Use a `build.rs` script to parse `lib.rs` and automatically generate `python_bindings.rs` at build time.

**Pros**:
- ✅ Single source of truth (lib.rs exports)
- ✅ Zero maintenance burden
- ✅ Preserves zero-dependency principle
- ✅ Clean separation of concerns
- ✅ Functions automatically in sync with library

**Cons**:
- ❌ Slightly more complex build setup
- ❌ Requires syn/quote dependencies for build script

**Implementation**:

```rust
// build.rs
use quote::quote;
use syn::{parse_file, Item, ItemUse};

fn main() {
    // Only run when python feature is enabled
    #[cfg(feature = "python")]
    {
        let lib_content = std::fs::read_to_string("src/lib.rs").unwrap();
        let syntax = parse_file(&lib_content).unwrap();
        
        let mut bindings = Vec::new();
        
        for item in syntax.items {
            if let Item::Use(ItemUse { tree, .. }) = item {
                // Parse use statements and generate wrappers
                // Extract function names and signatures
                bindings.push(generate_wrapper(tree));
            }
        }
        
        // Generate python_bindings.rs
        let output = quote! {
            use pyo3::prelude::*;
            use crate::*;
            
            #(#bindings)*
            
            #[pymodule]
            fn _redstr(m: &Bound<'_, PyModule>) -> PyResult<()> {
                // Auto-register all functions
                Ok(())
            }
        };
        
        std::fs::write("src/python_bindings.rs", output.to_string()).unwrap();
    }
}
```

**Effort**: ~4 hours
**Maintenance**: Near zero

---

### ✅ Solution 2: Macro-Based Generation

**Approach**: Use declarative macros to generate bindings from a list.

**Pros**:
- ✅ Much less boilerplate than current approach
- ✅ Easier to maintain than current solution
- ✅ No build script complexity

**Cons**:
- ❌ Still requires listing functions (but much shorter)
- ❌ Manual updates needed when adding functions

**Implementation**:

```rust
// Define once
export_to_python! {
    // Case transformations
    alternate_case(text: &str) -> String,
    case_swap(text: &str) -> String,
    leetspeak(text: &str) -> String,
    // ... etc (one line per function)
}

// Macro generates all wrappers automatically
macro_rules! export_to_python {
    ($($name:ident($($arg:ident: $arg_ty:ty),*) -> $ret:ty),* $(,)?) => {
        $(
            #[pyfunction(name = stringify!($name))]
            fn [<py_ $name>]($($arg: $arg_ty),*) -> $ret {
                $name($($arg),*)
            }
        )*
        
        #[pymodule]
        fn _redstr(m: &Bound<'_, PyModule>) -> PyResult<()> {
            $(
                m.add_function(wrap_pyfunction!([<py_ $name>], m)?)?;
            )*
            Ok(())
        }
    };
}
```

**Effort**: ~2 hours
**Maintenance**: Low (add one line per new function)

---

### ❌ Solution 3: Direct Export with #[pyfunction]

**Approach**: Add `#[pyfunction]` directly to source functions.

**Pros**:
- ✅ Zero duplication
- ✅ Impossible to get out of sync

**Cons**:
- ❌ **BREAKS zero-dependency principle** (PyO3 always required)
- ❌ Mixes concerns (Python in core library)
- ❌ Makes library harder to understand
- ❌ **NOT RECOMMENDED for redstr**

---

### ⚠️ Solution 4: uniffi (Mozilla)

**Approach**: Use uniffi to generate bindings from UDL files.

**Pros**:
- ✅ Multi-language support (Python, Kotlin, Swift, Ruby)
- ✅ Industry standard (used by Mozilla)
- ✅ Well maintained

**Cons**:
- ❌ Requires complete rewrite of API
- ❌ Changes function signatures (async by default)
- ❌ Adds significant dependency
- ❌ Not compatible with current architecture

---

## Recommendation

**Implement Solution 1 (Build Script Auto-Generation) or Solution 2 (Macro-Based)**

### Why Solution 1 is Best:
1. **Zero maintenance** - Functions are automatically exported
2. **Single source of truth** - lib.rs is the only place to update
3. **Preserves principles** - Zero-dependency for core library
4. **Future-proof** - Adding functions requires no Python code changes

### Why Solution 2 is Good Enough:
1. **Simpler** - No build script complexity
2. **Still much better** - Reduces 1400 lines to ~60 lines
3. **Maintainable** - Clear what's exported
4. **Quick to implement** - Can be done in 2 hours

## Current State vs Proposed

### Current (python_bindings.rs):
- 1400+ lines of code
- Every function manually wrapped
- Full docstrings duplicated
- High maintenance burden
- Easy to forget updating

### With Solution 1 (Build Script):
- ~100 lines in build.rs
- python_bindings.rs auto-generated
- Zero maintenance
- Impossible to forget

### With Solution 2 (Macros):
- ~60 lines of function list
- ~50 lines of macro code
- Low maintenance
- Easy to update

## Migration Path

### Phase 1 (Immediate - 2 hours):
1. Implement Solution 2 (macro-based)
2. Replace current python_bindings.rs
3. Verify all tests pass
4. Document approach

### Phase 2 (Optional - 4 hours):
1. Implement Solution 1 (build script)
2. Auto-generate from lib.rs
3. Update documentation
4. Remove macro-based code

## Example: Macro-Based Implementation

```rust
// src/python_bindings.rs - MUCH shorter version

use pyo3::prelude::*;
use crate::*;

// All functions in one concise list
py_export! {
    // Case (7 functions)
    alternate_case, case_swap, inverse_case, randomize_capitalization,
    to_camel_case, to_kebab_case, to_snake_case,
    
    // Encoding (6 functions)
    base64_encode, hex_encode, hex_encode_mixed, html_entity_encode,
    mixed_encoding, url_encode,
    
    // ... etc - one line per category
}

// Special cases handled separately
#[pyfunction(name = "random_user_agent")]
fn py_random_user_agent() -> String { random_user_agent() }

#[pyfunction(name = "ssti_framework_variation")]
fn py_ssti_framework_variation(template: &str, framework: &str) -> String {
    ssti_framework_variation(template, framework)
}
```

Total: ~100 lines instead of 1400!

## Decision

**Recommend implementing Solution 2 now** (can take 2 hours) with **optional upgrade to Solution 1 later** if we want zero maintenance.

Both are significantly better than the current approach and maintain redstr's architectural principles.
