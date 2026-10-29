# Client setup

`mono-proto-client` is the crate most people start with. It gives you a blocking, handle-based API for sign-in, dialogs, messages, live updates, media transfers, stickers, instant view, wallpapers and proxies.

Use the Rust crates directly in a native application. If you need Kotlin, Swift or another language, wrap the client in an adapter (for example with UniFFI) inside your own application. The adapter isn't part of this repository, and the Kotlin examples in these docs only show what one could look like.

- [USAGE](USAGE.md): how to call the library, with recipes
- [ARCHITECTURE](ARCHITECTURE.md): how it works inside

## Quick start

Add the crate to your project:

```toml
[dependencies]
mono-proto-client = { path = "../mono-rs/crates/mono-proto-client" }
```

Then connect, sign in and load your chats:

```rust
let handle = mono_proto_client::create_client(api_id, api_hash, session_path);
mono_proto_client::connect(handle)?;

if !mono_proto_client::is_authorized(handle)? {
    let sent = mono_proto_client::send_auth_code(handle, phone)?;
    // Supply the code selected by your auth flow.
    mono_proto_client::sign_in(handle, phone, sent.phone_code_hash, code)?;
}

mono_proto_client::start_updates(handle)?;
let chats = mono_proto_client::get_chats(handle)?;
let history = mono_proto_client::get_history(handle, chats[0].id, 50, 0, 0, 0)?;
```

The same flow through a generated Kotlin adapter:

```kotlin
val handle: ULong = createEncryptedClient(apiId, apiHash, sessionPath, key32)
connect(handle)

if (!isAuthorized(handle)) {
    val sent = sendAuthCode(handle, phone)
    // show a prompt that matches sent.codeType, then:
    signIn(handle, phone, sent.phoneCodeHash, code)
}

startUpdates(handle)
val chats = getChats(handle)
```

Rust uses snake_case names and `u64` handles, and generated bindings follow the target language's conventions. Every call blocks the calling thread, so run calls on a worker thread. The full walkthrough, with 2FA, updates, downloads and error handling, is in [USAGE: A complete session](USAGE.md#a-complete-session).

## Where to go next

| You want to                              | Read                                                                     |
|------------------------------------------|--------------------------------------------------------------------------|
| Log a user in and handle every code type | [Signing in](USAGE.md#signing-in)                                        |
| Receive messages reliably                | [Live updates](USAGE.md#live-updates)                                    |
| Download media fast and resume it        | [Downloads](USAGE.md#downloads)                                          |
| Pick the right priority for a call       | [Scheduling](USAGE.md#scheduling)                                        |
| Use a proxy                              | [Proxies and transport](USAGE.md#proxies-and-transport)                  |
| Understand an error                      | [Errors](USAGE.md#errors)                                                |
| Use it from Swift, Python or Ruby        | [Using it from other languages](USAGE.md#using-it-from-other-languages) |
| Understand the update pipeline           | [ARCHITECTURE: Live updates](ARCHITECTURE.md#live-updates)               |
| Understand the CDN and download path     | [ARCHITECTURE: Downloads](ARCHITECTURE.md#downloads)                     |
| Upgrade the TL layer                     | [ARCHITECTURE: TL layers and schema](ARCHITECTURE.md#tl-layers-and-schema) |

## Credentials

1. Create the Telegram account in an official app. The library signs in existing accounts only.
2. Get an API ID and hash at [my.telegram.org/apps](https://my.telegram.org/apps). One phone number gets one API ID.
3. Pass them to `create_client` or `create_encrypted_client`, and keep them out of version control.

Sample API IDs from Telegram's open-source clients are rate-limited (`API_ID_PUBLISHED_FLOOD`) and only good for testing.

Your app has to follow the [API Terms of Service](https://core.telegram.org/api/terms). In short: it must say that it uses the Telegram API and is part of the Telegram ecosystem, its title must not include "Telegram" unless preceded by "Unofficial", and it must not use the official logo.

> **Warning:** my.telegram.org can refuse to create an API ID from a VPN, proxy, datacenter or Tor address, usually with a generic form error. There's no client-side workaround, so retry from a regular connection. See [API ID troubleshooting](USAGE.md#troubleshooting).

## Build

You need Rust 1.98 or newer and Cargo.

```sh
git clone https://github.com/monogram-android/mono-rs.git
cd mono-rs

cargo build --workspace --release
cargo test --workspace
cargo xtask test-layers
```

The tests use local sockets and deterministic protocol fixtures. They need no device, no account and no connection to Telegram.

### Building for Android

Add the Android targets and `cargo-ndk`, and install an NDK (CI uses r28c):

```sh
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android
cargo install cargo-ndk --locked
```

The Gradle tasks and the binding generation that wrap this belong to the application you embed the library in. Rust consumers should depend directly on the `mono-proto-*` crate they need.

## Troubleshooting

| Symptom                           | Check                                                         | Details                                                              |
|-----------------------------------|---------------------------------------------------------------|----------------------------------------------------------------------|
| Login fails with a generic error  | `api_id`/`api_hash`, sample IDs, IP restrictions              | [API ID troubleshooting](USAGE.md#troubleshooting)                   |
| "No account" for a number         | The account must already exist                                | [Sign-in only](USAGE.md#sign-in-only)                                |
| No SMS arrives                    | `codeType`; the code may be in an official app                | [When no code arrives](USAGE.md#when-no-code-arrives)                |
| Code is rejected                  | Expiry, resend, the `phoneCodeHash` you pass                  | [Submit the code and the password](USAGE.md#submit-the-code-and-the-password) |
| Session dies right after login    | Another client uses the same authorization                    | [Common errors](USAGE.md#common-errors)                              |
| Download fails                    | Media not indexed, `FILE_REFERENCE`, wrong dispatch class     | [Download errors](USAGE.md#download-errors)                          |
| Downloads are slow                | Dispatch class, lanes, parts, chunk size                      | [Tuning](USAGE.md#tuning)                                            |
| Updates seem to stall             | Drain loop running, batches acknowledged                      | [Live updates](USAGE.md#live-updates)                                |
| Unfamiliar error                  | Look it up in the official error database                     | [Errors](USAGE.md#errors)                                            |

## Documentation

| Document                                 | Use it for                                                                          |
|------------------------------------------|-------------------------------------------------------------------------------------|
| [README](../README.md)                   | What the library is, its features and how to build it                               |
| [USAGE](USAGE.md)                        | API calls, recipes, tuning, errors, other languages, and the Telegram documentation map |
| [ARCHITECTURE](ARCHITECTURE.md)          | Design decisions, scheduling, RPC recovery, updates, downloads and CDN, sessions, security |
| [RUNTIME](RUNTIME.md)                    | The low-level protocol crates: codec, crypto, transport, session, engine            |
| [THIRD_PARTY_NOTICES](THIRD_PARTY_NOTICES.md) | Schema snapshots and third-party licenses                                      |

## License

The client crate is GPL-3.0-only. Applications that embed it can have their own license terms. Vendored components keep their own licenses; see the repository [LICENSE](../LICENSE) and the [third-party notices](THIRD_PARTY_NOTICES.md).
