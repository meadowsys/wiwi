# Contributing

file!

## Cargo.toml structure

- `package`
  - `name`
  - `description`
  - `version = { workspace = true }`
  - `edition = { workspace = true }`
  - `rust-version = { workspace = true }`
  - `authors = { workspace = true }`
  - `repository = { workspace = true }`
  - `license = { workspace = true }`
  - `categories = [..]`
  - `keywords = [..]`
- `lib` (usually not applicable)
- `dependencies`
- `dev-dependencies`
- `features`
- `lints`
  - `workspace = true`
- `package.metadata.docs.rs`
  - `rustdoc-args = ["--cfg", "docsrs"]`
  - `features = [..]`
  - `targets = [..]`
