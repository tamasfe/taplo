# Binary Releases

We pre-compile each release for all major operating systems, these releases can be found on [GitHub](https://github.com/tamasfe/taplo/releases).

## Variations

Taplo offers features — such as the language server — that might not be useful for most use-cases. For this reason we build multiple variations that differ in terms of features.

### Default Build

The default build with commonly-used features.

#### Download

- [Linux (x86_64)](https://github.com/tamasfe/taplo/releases/latest/download/taplo-linux-x86_64.gz)
- [macOS (x86_64)](https://github.com/tamasfe/taplo/releases/latest/download/taplo-darwin-x86_64.gz)
- Windows coming soon...

### Full Build

The full build contains the following additional features:

- Language Server
- An interface for [toml-test](https://github.com/BurntSushi/toml-test)

#### Download

- [Linux (x86_64)](https://github.com/tamasfe/taplo/releases/latest/download/taplo-full-linux-x86_64.gz)
- [macOS (x86_64)](https://github.com/tamasfe/taplo/releases/latest/download/taplo-full-darwin-x86_64.gz)
- Windows coming soon...

## Example

```bash
curl -fsSL https://github.com/tamasfe/taplo/releases/latest/download/taplo-full-linux-x86_64.gz \
  | gzip -d - | install -m 755 /dev/stdin /usr/local/bin/taplo
```