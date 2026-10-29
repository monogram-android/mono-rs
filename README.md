# Monogram MTProto

A Rust library for talking to Telegram as a regular user account over MTProto.

It signs in, keeps a session on disk, receives updates, sends messages and moves media. You call a handle-based API; the library takes care of encryption, connections, retries and recovery. The API is designed for use from Rust applications or language-specific adapter crates.

## Features

- **Fully encrypted sessions.** The session file is encrypted with AES-256-GCM using a 32-byte key you provide. A wrong key or a damaged file never overwrites a good session
- **Perfect forward secrecy.** Extra connections use temporary keys that exist only in memory, so a leaked session file can't decrypt old traffic
- **Fast media downloads.** Parallel lanes, up to eight parts in flight, resume after a crash, cancel from any thread
- **Full CDN support.** CDN redirects are followed automatically and every part is hash-checked against the master datacenter. Anything that fails verification falls back to the origin
- **Smart updates.** You never track `pts`, `qts` or `seq`. The library orders packets, fills gaps and gives you durable batches that survive a crash until you acknowledge them
- **Works on bad networks.** SOCKS5, HTTP, HTTPS and MTProxy (simple, `dd` and `ee` secrets), plus datacenter port rotation

## Quick start

You need Rust 1.98 or newer

```sh
git clone https://github.com/monogram-android/mono-rs
cd mono-rs

cargo xtask test-layers
cargo test --workspace --all-features
```

The tests use local fixtures and sockets, so no Telegram credentials are needed. The default build targets TL layer 232. To build every supported layer (223, 229, 230, 231, 232), run `cargo build-layers`

Using the client looks like this:

```rust
// Sketch of the flow. USAGE.md has the full recipes with the real function names.
let handle = create_encrypted_client(api_id, api_hash, session_path, key32)?;
connect(handle)?;

if !is_authorized(handle)? {
    let sent = send_auth_code(handle, phone)?;
    sign_in(handle, phone, sent.phone_code_hash, code)?;
}

start_updates(handle)?;
let chats = get_chats(handle)?;
```

You need your own `api_id` and `api_hash` from [my.telegram.org](https://my.telegram.org). [USAGE](docs/USAGE.md#credentials) explains how to get them

## Things to know first

- Every call blocks the calling thread. Run calls on a worker thread
- The library signs in existing accounts only. Create the account in an official Telegram app first
- One session file belongs to one handle in one process
- The repository builds for host and Android. Other targets need their own build
- The test suite doesn't cover live Telegram behavior, and the library hasn't had an independent security review

## What's in the repository

`mono-proto-client` is the crate you start with. The rest are the layers beneath it, usable on their own

| Crate                                                                 | Purpose                                                    |
|-----------------------------------------------------------------------|------------------------------------------------------------|
| `mono-proto-client`                                                   | The client: handles, sessions, updates, operations         |
| `mono-proto-runtime`                                                  | Connections, authentication, RPC and recovery              |
| `mono-proto-messages`, `mono-proto-media`, `mono-proto-updates`        | Messages and dialogs, media and CDN, update mapping        |
| `mono-proto-storage`, `mono-proto-types`, `mono-proto-observability`  | Session storage and journal, shared types, timing counters |
| `mono-proto`, `mono-proto-codec`, `mono-proto-crypto`                 | Generated TL declarations, TL codec, cryptography          |
| `mono-proto-session`, `mono-proto-engine`                             | Protocol session state, encrypted RPC engine               |
| `mono-proto-transport`, `mono-proto-socket`                           | Framing and traits, TCP, proxies and MTProxy               |
| `mono-proto-testkit`                                                  | Test utilities                                             |

`xtask` holds the development tooling behind the `cargo` aliases

## Documentation

| Document                                      | Read it to                                                              |
|-----------------------------------------------|-------------------------------------------------------------------------|
| [CLIENT](docs/CLIENT.md)                      | Set up a standalone build and see how the client fits together          |
| [USAGE](docs/USAGE.md)                        | Sign in, handle updates, send messages, download media, handle errors   |
| [ARCHITECTURE](docs/ARCHITECTURE.md)          | Understand the design and its invariants before changing the internals  |
| [RUNTIME](docs/RUNTIME.md)                    | Learn how the protocol runtime works                                    |
| [THIRD_PARTY_NOTICES](docs/THIRD_PARTY_NOTICES.md) | See schema snapshots and third-party licenses                      |

Everything here follows Telegram's public specification. The full list of Telegram pages we rely on is at the end of [USAGE](docs/USAGE.md#telegram-documentation-map)

## License

[GPL-3.0-only](LICENSE). Third-party components keep their own licenses and notices.

This project is not affiliated with Telegram. If you build an app on it, follow the [Telegram API Terms of Service](https://core.telegram.org/api/terms)
