# peerpipe

An auto-discoverable, encrypted network pipe for P2P data transfer

## Example

```bash
machine-a $ echo "Hello" | peerpipe send
```

```bash
machine-b $ peerpipe recv
Hello
```

In the example above, any `machine-b` on the network could run that command and receive the data from `machine-a`.  If you'd like the receiver to authenticate themselves you can request they provide a secret:

```bash
machine-a $ echo "Hello" | peerpipe send -s supersecret
```

The receiver must provide a matching secret in order to receive any data:

```bash
machine-b $ peerpipe recv -s supersecret
```

## Install

```bash
git clone https://github.com/scttnlsn/peerpipe
cd peerpipe
cargo build --release
sudo cp target/release/peerpipe /usr/bin/peerpipe # or somewhere else in your PATH
```
