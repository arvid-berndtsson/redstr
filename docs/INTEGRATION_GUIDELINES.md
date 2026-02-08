# Integration Guidelines for redstr

This document provides guidance on integrating redstr with security tools, creating extensions, and organizing integration code.

## 🎯 Overview

redstr is designed to be integrated into security testing tools and frameworks. This guide helps you decide where to build your integration and how to structure it.

## 📦 Where Should Integrations Live?

### Use a Separate Repository When:

✅ **The integration is a standalone tool or add-on** that has its own build process, dependencies, and lifecycle
   - **Examples:** OWASP ZAP add-ons, Burp Suite extensions, browser extensions
   - **Why:** These tools have their own plugin ecosystems, packaging requirements, and distribution channels

✅ **The integration requires dependencies that conflict with redstr's zero-dependency principle**
   - **Examples:** Java-based Burp Suite extensions, Python-based tools
   - **Why:** redstr core maintains zero dependencies (std only). External integrations can use whatever they need.

✅ **The integration targets a specific tool's marketplace or distribution system**
   - **Examples:** Burp Suite BApp Store, OWASP ZAP Marketplace, VS Code Marketplace
   - **Why:** These platforms often require specific repository structures and build configurations

✅ **The integration has a different release cycle than redstr**
   - **Examples:** Tool-specific plugins that need frequent updates independent of redstr releases
   - **Why:** Separate repos allow independent versioning and release schedules

✅ **The integration is language-specific and can't be written in Rust**
   - **Examples:** Java-based Burp extensions, Python scripts for tool X
   - **Why:** Separate repos allow using the appropriate language and tooling

### Use the redstr Repository When:

✅ **Creating usage examples or integration documentation**
   - **Location:** `examples/` directory
   - **Examples:** Code samples showing how to use redstr in various contexts

✅ **Adding library features that enable integrations**
   - **Location:** `src/` directory
   - **Examples:** New transformation functions, builder methods, or serialization support

✅ **Contributing integration documentation**
   - **Location:** `docs/` directory  
   - **Examples:** Integration guides, API documentation, best practices

## 🔧 Integration Patterns

### Pattern 1: Standalone Add-on/Extension (Separate Repo)

**Best for:** OWASP ZAP, Burp Suite, browser extensions

**Repository Structure:**
```
your-tool-redstr-integration/
├── src/                    # Extension source code
├── lib/                    # Include redstr as dependency
├── manifest.json          # Tool-specific manifest
├── build.gradle           # Tool-specific build
├── README.md              # Installation & usage
└── Cargo.toml             # If using Rust, reference redstr
```

**How to include redstr:**
```toml
[dependencies]
redstr = "0.1"
```

**Examples:**
- `redstr-burp-extension` - Separate repo for Burp Suite integration
- `redstr-zap-addon` - Separate repo for OWASP ZAP integration
- `redstr-caido-plugin` - Separate repo for Caido integration

### Pattern 2: Library Integration (This Repo)

**Best for:** Code examples, library enhancements, documentation

**Location in redstr repo:**
- Examples: `examples/your_integration_example.rs`
- Documentation: `docs/your_integration_guide.md`
- Library code: `crates/redstr/src/transformations/your_module.rs`

**Example:**
```rust
// examples/burp_usage.rs
use redstr::{url_encode, xss_tag_variations};

fn main() {
    let payload = "<script>alert(1)</script>";
    let encoded = url_encode(&xss_tag_variations(payload));
    println!("{}", encoded);
}
```

### Pattern 3: HTTP API Bridge (Either Location)

**Best for:** Tools that need HTTP/REST access to redstr

**Options:**
1. **Separate microservice repo** - If it needs its own deployment
2. **Example in redstr repo** - If it's a reference implementation

**Example structure (separate repo):**
```
redstr-api-server/
├── src/
│   └── main.rs           # HTTP server
├── Dockerfile
└── Cargo.toml
    └── dependencies: redstr = "0.1"
```

## 🌟 OWASP ZAP Add-on Specific Guidance

### Should I create a separate repo for an OWASP ZAP add-on?

**Yes, absolutely.** OWASP ZAP add-ons should be in separate repositories because:

1. **ZAP Marketplace Requirements** - ZAP add-ons need specific structure and packaging for the ZAP Marketplace
2. **Java/Kotlin Build System** - ZAP add-ons are typically Java-based with Gradle/Maven builds
3. **Independent Release Cycle** - ZAP add-ons are released independently through the ZAP Marketplace
4. **Different Versioning** - Add-on versions should follow ZAP conventions, not redstr's

### Recommended Structure for ZAP Add-on:

```
redstr-zap-addon/
├── src/
│   └── main/
│       └── java/org/zaproxy/addon/redstr/
│           ├── RedstrExtension.java
│           └── transformations/
│               └── RedstrTransformations.java
├── lib/
│   └── redstr-jni.so      # JNI bindings to Rust
├── build.gradle.kts       # Gradle build
├── CHANGELOG.md
├── README.md
└── ZapAddOn.xml           # ZAP add-on manifest
```

### Integration Approach for ZAP:

**Option A: JNI Bindings (Recommended for performance)**
- Create Rust C-compatible library
- Use JNI to call from Java
- Package native library with add-on

**Option B: HTTP Bridge**
- Run redstr as microservice
- ZAP add-on calls HTTP endpoints
- Good for distributed deployments

**Option C: CLI Wrapper**
- ZAP add-on shells out to redstr CLI
- Simplest but slowest approach

### Steps to Create Your ZAP Add-on:

1. **Create separate repository:** `redstr-zap-addon` or similar name
2. **Add redstr dependency:**
   - For Rust FFI: Use `cargo` to build shared library
   - For HTTP: Use `reqwest` or similar to call redstr API
3. **Follow ZAP guidelines:** https://www.zaproxy.org/docs/developer/
4. **Reference redstr in your README:**
   ```markdown
   This add-on uses [redstr](https://github.com/arvid-berndtsson/redstr)
   for string transformations.
   ```
5. **Submit to ZAP Marketplace** when ready

## 🔗 Linking Back to redstr

Regardless of where your integration lives, please:

1. **Link to redstr** in your README:
   ```markdown
   Built with [redstr](https://github.com/arvid-berndtsson/redstr) - 
   Red team string transformation library
   ```

2. **Mention redstr** in your tool's description/manifest

3. **Let us know!** Open an issue or PR to add your integration to our list:
   - Add to `README.md` under integrations section
   - We'll feature it in our documentation

## 🤝 Community Integrations

### Tell Us About Your Integration

We love seeing redstr used in creative ways! Please let us know by:

1. **Opening an issue** with label `integration-showcase`
2. **Creating a PR** to add your integration to our list
3. **Sharing in discussions** on GitHub

### Integration Showcase

We maintain a list of community integrations in the README. Submit yours!

**Current integrations:**
- (List will be maintained in README.md)

## 📚 Resources

### For Integration Developers

- **API Documentation:** https://docs.rs/redstr
- **Examples Directory:** `/examples` in this repo
- **Community Chat:** (Coming soon)
- **Integration Support:** Open an issue with `integration-help` label

### Tool-Specific Resources

- **Burp Suite:** https://portswigger.net/burp/extender
- **OWASP ZAP:** https://www.zaproxy.org/docs/developer/
- **Caido:** https://docs.caido.io/ (when available)

## ❓ Still Unsure?

If you're not sure where your integration should live, open an issue with:
- Description of what you want to build
- Target tool/platform
- Technical requirements
- Expected lifecycle

We'll help you decide the best approach!

## 📝 Summary

**Quick Decision Tree:**

```
Are you building an add-on for a specific tool (Burp, ZAP, etc.)?
├─ YES → Separate repository
└─ NO → Continue...

Does it require non-Rust dependencies?
├─ YES → Separate repository  
└─ NO → Continue...

Is it just example code or documentation?
├─ YES → This repository (examples/ or docs/)
└─ NO → Separate repository

Does it need independent releases?
├─ YES → Separate repository
└─ NO → This repository (if it fits)
```

---

**Questions?** Open an issue or discussion on GitHub!
