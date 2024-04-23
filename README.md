# Haskell Plugin

[![ci](https://github.com/fluentci-io/haskell-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/fluentci-io/haskell-plugin/actions/workflows/ci.yml)

This plugin sets up your CI/CD pipeline with a specific version of [haskell](https://www.haskell.org/).

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm haskell setup
```

## Functions

| Name   | Description                                |
| ------ | ------------------------------------------ |
| setup  | Installs a specific version of haskell.    |
| build  | Compile targets within the project.        |
| test   | Run test-suites.                           |
| install | Install packages.                         |
| check   |Check the package for common mistakes.     |
| sdist   | Generate a source distribution file (.tar.gz). |
| upload  | Uploads source packages or documentation to Hackage. |
| report  | Upload build reports to a remote server. | 

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.1.9"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call("https://pkg.fluentci.io/haskell@v0.1.0?wasm=1", "setup", vec!["latest"])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: haskell
    args: |
      setup
- name: Show ghc, cabal version
  run: |
    type ghc
    type cabal
    ghc --version
    cabal --version
```
