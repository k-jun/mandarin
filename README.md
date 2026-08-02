# MANDARIN

Mandarin is a simple tool to manage GMail's Filter as a TOML file.

## Install

### Prebuilt binary (no cargo required)

Download the binary for your platform from [GitHub Releases](https://github.com/k-jun/mandarin/releases/latest).

```sh
# macOS (Apple Silicon)
curl -fsSL https://github.com/k-jun/mandarin/releases/latest/download/mandarin-aarch64-apple-darwin.tar.gz | tar xz
mv mandarin  ~/.local/bin/

# macOS (Intel)
curl -fsSL https://github.com/k-jun/mandarin/releases/latest/download/mandarin-x86_64-apple-darwin.tar.gz | tar xz
mv mandarin  ~/.local/bin/

# Linux (x86_64)
curl -fsSL https://github.com/k-jun/mandarin/releases/latest/download/mandarin-x86_64-unknown-linux-musl.tar.gz | tar xz
mv mandarin  ~/.local/bin/

# Linux (arm64)
curl -fsSL https://github.com/k-jun/mandarin/releases/latest/download/mandarin-aarch64-unknown-linux-musl.tar.gz | tar xz
mv mandarin  ~/.local/bin/
```

### With cargo

```sh
cargo install --git https://github.com/k-jun/mandarin
```

## Usage

```sh
# Generate a sample config at ~/.mandarin/config.toml
mandarin init

# Convert the config into Gmail's filter XML (import it from Gmail settings)
mandarin run > ~/Desktop/mandarin.xml
```

## Release (for maintainers)

Push a `v*` tag and GitHub Actions builds and attaches binaries to the release.

```sh
git tag v0.1.0 && git push origin v0.1.0
```
