# Slack API in Rust

[![ci](https://github.com/Gompei/slack-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/Gompei/slack-rust/actions/workflows/ci.yml)

This is a Slack library for Rust that I'm working on, inspired by [slack-go/slack](https://github.com/slack-go/slack).

## Support Status

- [ ] Web API
- [ ] Events API
- [ ] Socket Mode

## Other Reference Repository

- [slack-rs/slack-rs](https://github.com/slack-rs/slack-rs)
- [Pctg-x8/slack-socket-mode-client](https://github.com/Pctg-x8/slack-socket-mode-client)

## linux build

```bash
$rustup target add x86_64-unknown-linux-musl
$brew install filosottile/musl-cross/musl-cross
$mkdir .cargo
$echo '[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"' > .cargo/config
ln -s /usr/local/bin/x86_64-linux-musl-gcc /usr/local/bin/musl-gcc
cargo build --release --target x86_64-unknown-linux-musl
```