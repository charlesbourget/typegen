# TypeGen

Quickly generate Rust struct from OpenApi specification.
This will generate only the types located in the `components.schema` section
of the OpenApi document.

Under the hood this project uses:

- [progenitor](https://github.com/oxidecomputer/progenitor)
- [openapiv3](https://github.com/glademiller/openapiv3)

## Usage

### In `build.rs`

Add the following to your `build.rs`:

```rust
fn main() {
    let openapi_spec = "openapi.yml";

    let generation_settings = GenerationSettings::default();
    let content = typegen::generate_code(openapi_spec, &generation_settings).unwrap();

    let mut out_file = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).to_path_buf();
    out_file.push("typegen.rs");

    std::fs::write(out_file, content).unwrap();
}
```

Then include the generated file:

```rust
include!(concat!(env!("OUT_DIR"), "/typegen.rs"));
```

### Using the CLI

```bash
typegen openapi.yml output.rs
```
