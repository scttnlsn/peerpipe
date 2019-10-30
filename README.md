# peerpipe

An auto-discoverable network pipe for P2P data transfer

## Example

```bash
machine-a $ echo "Hello" | peerpipe send
```

```bash
machine-b $ peerpipe recv
Hello
```

## Install

```bash
git clone https://github.com/scttnlsn/peerpipe
cd peerpipe
cargo build --release
sudo cp target/release/peerpipe /usr/bin/peerpipe # or somewhere else in your PATH
```
