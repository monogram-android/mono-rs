# Protocol building blocks

The low-level MTProto crates of the workspace: codec, crypto, transport, session and engine, plus a test kit. They don't depend on any async runtime, so you can build your own client on top of them. The ready-made client, `mono-proto-client`, is built from these pieces.

## Crates

| Crate                  | What it provides                                                                                                                                                                                       |
|------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `mono-proto`           | Generated Telegram TL declarations                                                                                                                                                                     |
| `mono-proto-codec`     | Bounded TL primitives, bare and boxed extension traits, flags, dependent lengths, generic vectors, recursion handling, constructor errors                                                              |
| `mono-proto-crypto`    | SHA-1 and SHA-256, AES-256-IGE and CTR, MTProto 2.0 message envelopes and KDFs, quick-ack tokens, RSA fingerprints and RSA_PAD, bounded `pq` factorization, DH validation and derivation, salts, OS randomness |
| `mono-proto-transport` | Abridged, intermediate, padded-intermediate, full and HTTP framing; quick acknowledgements; direct and MTProxy client obfuscation; a synchronous runtime-adapter boundary                              |
| `mono-proto-session`   | Auth key, salt, IDs, sequences, bounded replay history, acknowledgements, clock repair, migration, reconnect state, persistence traits                                                                  |
| `mono-proto-engine`    | Typed method adapters, encrypted and plain envelopes, raw containers, bounded `gzip_packed`, acknowledgements, correlation, retries, RPC error advice, authorization transitions                       |
| `mono-proto-testkit`   | Fake connections, deterministic time and randomness, malformed fixtures, Telegram's official authorization vectors, Telegram Desktop behavior regressions                                              |

See each crate's `Cargo.toml` for its exact dependencies.

Every crate is GPL-3.0-only and forbids unsafe Rust. Public failures use typed errors, and no core crate depends on Tokio, async-std, `anyhow` or `eyre`.

Each crate root is a small documented facade. The implementation is organized by responsibility instead of piling up in `lib.rs`:

```text
codec/      decoder · encoder · error · traits · value
crypto/     symmetric · message · rsa · dh · error
transport/  common · framing · obfuscated · http · connection
session/    state · receive · ports · error
engine/     wire · rpc · rpc_error · authorization · error
testkit/    time · connection · fixtures
```

Unit and property tests live in per-crate `tests.rs` modules. Cross-crate vectors derived from Telegram's sources are integration tests under `mono-proto-testkit/tests`.

## What's not here

These crates don't implement high-level updates, file transfers, users and chats, a storage policy, the Secret Chat state machine or an MTProxy server. That work lives in the crates above them, starting with [`mono-proto-client`](CLIENT.md).

## Wire behavior and limits

**TL encoding.** Numbers are little-endian. Strings and byte arrays use canonical 1- or 4-byte length prefixes and zero padding to a 4-byte boundary. Decoding rejects redundant length forms, invalid padding, trailing input, excessive recursion, and oversized aggregate input, byte lengths and vector counts. Generated layer types dispatch their own constructor IDs through the codec traits, which is what lets adapters support several pinned layers.

**Transports.** Decoders are incremental: `Incomplete` reports the buffer size it needs, and `Packet::consumed` preserves a following packet. The full transport checks sequence and CRC before advancing receive state. Padded-intermediate deliberately returns the payload plus transport padding, because only the authenticated encrypted envelope can tell where a message ends. Obfuscated streams keep their AES-CTR state across partial reads and writes.

**Incoming encrypted messages** are authenticated before any plaintext field is returned. They're then checked for auth-key ID, session ID, message-ID parity and time, body limits, padding limits and replay. Gzip expansion is bounded before generated TL dispatch.

## Examples

Encode and decode a bare value:

```rust
use mono_proto_codec::{from_bytes, to_bytes, Limits};

let wire = to_bytes(&42_i64)?;
let value: i64 = from_bytes(&wire, Limits::default())?;
assert_eq!(value, 42);
```

Open session state with randomness you control:

```rust
use mono_proto_session::{OsRandom, Snapshot};

let mut state = Snapshot::new(2, &mut OsRandom)?;
state.set_auth_key(vec![7; 256])?;
```

### Sending an RPC

1. Implement `mono_proto_engine::Method` for a wrapper around the generated request you want to send. Its `encode_request` and `decode_response` call that layer's codec.
2. Pass the wrapper to `Engine::invoke`.
3. Drain `next_outbound`, add cryptographically random MTProto padding with `seal_outbound`, and apply the transport framing you chose.
4. A correlated `rpc_result` comes back through `receive_result`, and `take_response` decodes it.

Retries keep the typed handle and allocate fresh wire IDs.

### Plugging in a socket

A runtime adapter only has to implement the byte connection:

```rust
use mono_proto_transport::{Connection, Error};

struct MySocket;
impl Connection for MySocket {
    fn send(&mut self, packet: &[u8]) -> Result<(), Error> { /* write_all */ Ok(()) }
    fn receive(&mut self, output: &mut [u8]) -> Result<usize, Error> { /* read */ Ok(0) }
    fn close(&mut self) -> Result<(), Error> { Ok(()) }
}
```

Socket ownership, DNS, TLS, HTTP transfer decoding, timers, cancellation and task synchronization belong in separate runtime adapter packages (for example a Tokio one). `mono-proto-socket` is the adapter this workspace ships.

## Security and state assumptions

- Authenticate server RSA keys against a trusted key set you can update, and apply the crypto crate's DH checks before deriving an authorization key.
- `rsa_pad` uses the operating system's CSPRNG. Deterministic injection exists only for reproducible tests.
- Encrypt persisted auth keys and session state, and replace them atomically in your storage implementation.
- A restored snapshot is validated before the engine accepts it.

## Compatibility and test provenance

The credential-free integration tests reproduce Telegram's published [authorization-key sample](https://core.telegram.org/mtproto/samples-auth_key): plain envelopes, `resPQ`, `pq` factorization, temporary AES, the encrypted server DH answer, safe-prime and public-value checks, and the final nonce hash. Transport and service-message behavior follows Telegram's [transport](https://core.telegram.org/mtproto/mtproto-transports) and [service message](https://core.telegram.org/mtproto/service_messages) documents.

Tests that need live Telegram credentials are excluded by default. Runtime adapters, a trusted-key distribution policy and live DC interoperability remain the integrator's job, not part of the protocol core.

## Verification

```sh
just check
just package

# or run both stages exactly as CI does
just ci
```

The `justfile` exposes each routine command on its own. `cargo xtask ci` handles the dependency-aware orchestration CI uses, and `cargo xtask package` verifies dependency-root archives and builds dependent archives in publish order.

Dependent tarballs use `--no-verify` until their new workspace dependencies are published, and `xtask` supplies local registry patches only while it creates those archives. Release verification then goes in dependency order: codec, crypto and session first, then transport, engine and testkit.

## License

GPL-3.0-only; see [LICENSE](../LICENSE). Third-party components keep their own notices and licenses, listed in [THIRD_PARTY_NOTICES](THIRD_PARTY_NOTICES.md).
