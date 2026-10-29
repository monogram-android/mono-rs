# Monogram MTProto architecture

How the library is organized, and why. This page is for maintainers, reviewers and anyone embedding the library. If you only want to call it, start with [USAGE](USAGE.md). For setup, see [CLIENT](CLIENT.md).

- [Overview](#overview)
- [Workspace ownership](#workspace-ownership)
- [End-to-end flows](#end-to-end-flows)
- [Source map](#source-map)
- [Design decisions](#design-decisions)
- [Handles, state and concurrency](#handles-state-and-concurrency)
- [Adapter boundary](#adapter-boundary)
- [Threads](#threads)
- [Scheduling](#scheduling)
- [RPC path](#rpc-path)
- [Live updates](#live-updates)
- [Downloads](#downloads)
- [Media index](#media-index)
- [Uploads](#uploads)
- [Auth and sessions](#auth-and-sessions)
- [Network and proxies](#network-and-proxies)
- [Security model](#security-model)
- [Limits and backpressure](#limits-and-backpressure)
- [Failure matrix](#failure-matrix)
- [TL layers and schema](#tl-layers-and-schema)
- [Maintenance](#maintenance)
- [Extending the library](#extending-the-library)
- [Testing](#testing)
- [Invariants](#invariants)
- [References](#references)

## Overview

```
application (Rust, or any language through an adapter)
    |
    v
mono-proto-client       handles, lanes, scheduling, updates, media
    |
    v
mono-proto-runtime      authentication, RPC, connection and recovery
    |
    v
mono-proto-*            TL types, codec, crypto, session, storage
    |  framed bytes
    v
mono-proto-transport    framing and connection traits
    |
    v
mono-proto-socket       TCP and proxy connections
    |
    v
Telegram datacenters
```

The library runs inside one process and is addressed through integer handles. Calls block at the edge of the library. The host decides which thread runs them, and attaches a request token and a dispatch class. The library owns all socket, retry and session state.

Three ideas shape most of the design:

1. **Protocol work in Rust, scheduling in the host.** The boundary stays small and synchronous. Priorities and cancellation are passed in through thread state.
2. **Never lose or duplicate work.** Updates are journaled and acknowledged, downloads are staged and resumable, sends are matched by random id, and mutations are never replayed blindly.
3. **Fail closed.** Unknown constructors, unverified CDN data, wrong keys and mixed schemas stop the operation instead of guessing.

## Workspace ownership

Each crate owns one layer, and consumers depend on the crate that owns what they need. There is no compatibility facade crate.

| Crate                      | Owns                                                                   |
|----------------------------|------------------------------------------------------------------------|
| `mono-proto-client`        | The stateful handle-based client, session lifecycle, operation orchestration, durable update flow |
| `mono-proto-runtime`       | Connection, authentication, RPC, scheduling and transfer policy        |
| `mono-proto`               | Generated Telegram protocol declarations                               |
| `mono-proto-codec`         | TL encoding and decoding                                               |
| `mono-proto-crypto`        | MTProto cryptographic primitives                                       |
| `mono-proto-session`       | Protocol session state                                                 |
| `mono-proto-engine`        | Encrypted RPC and protocol engine                                      |
| `mono-proto-transport`     | Runtime-independent framing and connection traits                      |
| `mono-proto-socket`        | TCP connections, proxies and MTProxy                                   |
| `mono-proto-types`         | Shared data models                                                     |
| `mono-proto-storage`       | Session persistence, encryption and the update journal                 |
| `mono-proto-messages`      | Message, dialog and upload operations and their mappings               |
| `mono-proto-media`         | Media metadata, downloads and CDN                                      |
| `mono-proto-updates`       | Update recovery and mapping                                            |
| `mono-proto-observability` | Opt-in timing and counters                                             |
| `mono-proto-testkit`       | Protocol test utilities                                                |

An application may keep its own binding declarations and platform adapter, but it should not carry a second MTProto runtime or client.

## End-to-end flows

**Login**

1. The host calls `sendAuthCode` with the phone number.
2. The auth path creates or restores the auth key and sends `auth.sendCode`.
3. `AuthCodeSent` returns the phone hash, `codeType` and length.
4. `signIn` sends `auth.signIn`. The server may answer that a password is needed.
5. `checkPassword` completes SRP when required.
6. The host calls `startUpdates` and begins draining.

**Update delivery**

1. The update lane receives packets, and the reorder buffer resolves their sequence.
2. Durable events enter the journal together with the cursor they advanced.
3. The host drains a batch, commits it, then acknowledges it.
4. A crash before the acknowledgement replays the batch.

**Download**

1. The host sets a media dispatch class and calls a download function.
2. A media lane binds a temporary key if needed, and the transfer is admitted per datacenter.
3. `upload.getFile` parts fill the profile's window.
4. Parts are written to the staging file, and the ranges sidecar is updated.
5. CDN parts are decrypted and hash-verified, or the download falls back to the origin.
6. The staging file is published by rename under the session lease.

**Send**

1. The host supplies a fresh random id and calls a send function.
2. The request goes out on the main lane (for media, the upload lanes go first).
3. The call returns the message, or fails with an error that says whether delivery is uncertain.
4. Independently, `UpdateMessageId:<randomId>:<messageId>` arrives as a normal update, so even a lost response gets resolved.

## Source map

Paths are relative to the workspace root.

| Path                                           | Responsibility                                      |
|------------------------------------------------|-----------------------------------------------------|
| `crates/mono-proto-types/src/`                 | Shared data models and message entities             |
| `crates/mono-proto-messages/src/messages/`     | Message operations and mappings                     |
| `crates/mono-proto-messages/src/dialogs/`      | Dialogs, history, folders and forum topics          |
| `crates/mono-proto-messages/src/upload_rpc.rs` | File parts and outgoing media                       |
| `crates/mono-proto-media/src/`                 | Media locations, metadata, downloads and CDN        |
| `crates/mono-proto-runtime/src/`               | Authentication, RPC, scheduling and transfer policy |
| `crates/mono-proto-storage/src/`               | Session persistence, encryption and update journal  |
| `crates/mono-proto-observability/src/`         | Runtime timing and counters                         |
| `crates/mono-proto-socket/`                    | TCP and proxy connections                           |
| `crates/mono-proto-client/src/construct.rs`    | Client construction and registry initialization     |
| `crates/mono-proto-client/src/auth.rs`         | Connection setup and authentication flows           |
| `crates/mono-proto-client/src/dispatch.rs`     | Handle-based operations and RPC orchestration       |
| `crates/mono-proto-client/src/updates.rs`      | Update subscription and client update draining      |

Platform adapters belong to the applications that embed the library and are not part of this map.

## Design decisions

| Problem                                   | Solution                                                                 | Cost                                         | Main source                         |
|-------------------------------------------|--------------------------------------------------------------------------|----------------------------------------------|-------------------------------------|
| The TL schema changes every layer         | Generated schema and codec in the `mono-proto` workspace crates          | Layer is pinned                              | `Cargo.toml`                        |
| Hosts have different async models         | The client API is synchronous; hosts wrap calls in their own concurrency | Host-specific scheduling                     | `mono-proto-client/src/`            |
| One slow request blocks unrelated work    | Main, read, media and upload lanes with priority gates                   | More sockets and temporary keys              | `scheduler.rs`, `lanes.rs`          |
| Lanes can overwrite each other's state    | Clone `ClientState`, run the call, merge only changed entries            | Field-diff discipline                        | `client/mod.rs`                     |
| Updates must survive crashes              | Durable journal with explicit acknowledgement                            | Consumers must be idempotent                 | `update_journal.rs`                 |
| Downloads must survive process death      | Staging file, missing-range sidecar, identity sidecar                    | Disk use and cleanup                         | `media/download.rs`                 |
| CDN data can be corrupt                   | SHA-256 check against master-DC hashes, origin fallback                  | Slower fallback                              | `media/cdn.rs`                      |
| One authorization allows one main session | Extra sessions use bound temporary keys and `tmp_sessions`               | Extra auth work; fallback to main            | `auth/bind_temp.rs`, `scheduler.rs` |
| Blocking and censorship                   | SOCKS5, HTTP, HTTPS, MTProxy, DoH `help.configSimple`                    | Proxy compatibility                          | `mono-proto-transport`              |
| Session files hold sensitive data         | AES-256-GCM with a caller-supplied key                                   | Device-bound backups; caller manages the key | `session_crypto.rs`                 |

The trade-offs behind those choices:

- **Generated protocol code over hand-written TL.** Telegram declarations are never edited by hand, so a layer bump is a deliberate, tested change.
- **Blocking calls over a callback reactor.** The boundary stays small and easy to bind. Scheduling and cancellation belong to the host.
- **Separate lanes over one socket.** No head-of-line blocking, and it matches Telegram's parallel-session rules, at the cost of sockets and key management.
- **At-least-once updates over exactly-once.** A batch can be replayed, so the application has to be idempotent. The alternative, losing a batch in a crash, is worse.
- **Staged media over writing the final path.** A failed download never destroys the previous file and can resume, but staging files need cleanup.
- **Origin fallback over trusting the CDN.** Correctness over edge bandwidth.
- **Caller-owned session key over built-in key storage.** Every platform has its own secure storage, and a library can't pick one for all of them.

## Handles, state and concurrency

`create_client` returns a `u64` that indexes a process-global `CLIENTS` map. Every other function looks the handle up. A handle owns one session file and one authorization, and a second handle or process on the same authorization can cause `AUTH_KEY_DUPLICATED`.

A `Client` contains:

- one `main` lane and gate, carrying ordinary RPCs and the update connection
- two extra read lanes, used only when the server allows parallel sessions and a temporary key is bound
- up to eight media lanes, opened as needed up to the configured count
- four upload lanes
- a connection control object, a transfer policy, and shared `ClientData` (peers, update cursors, media index, journal, snapshots, caches)

Most stateful calls go through `with_client_mut`. It clones a `ClientState` from `ClientData` and the main-lane snapshot, runs the operation, and merges back only what changed. That stops a lane that spent time on the network from overwriting peer or media entries another lane wrote in the meantime. The merge is field-diffed on purpose: assigning the whole state back would bring the lost-update race back.

If the closure changes the auth key, session id or DC, the lane's transport is dropped instead of returned to the pool. A socket bound to an old session must not be reused after an auth restart, bad-message recovery or migration.

`session_dead` is set when the server revokes the authorization or the auth key is duplicated. It's saved with the session, so later calls fail fast with the stored reason.

## Adapter boundary

The library exposes Rust APIs directly. A host may add a synchronous adapter, for example a UniFFI layer for Android, but that adapter lives outside this workspace. It owns generated language bindings, DTO declarations and platform-specific error mapping. It has to delegate protocol and client behavior to `mono-proto-client` instead of duplicating it.

## Threads

There's no hidden async runtime. Work runs on the caller's thread and on a few named native threads:

- **Caller thread.** A direct Rust caller invokes the client synchronously. An adapter may add its own worker and callback boundary.
- **Media preparation.** A second media lane may be prepared on a short-lived worker while the first lane starts downloading.
- **`mtproto-maintenance`.** Closes idle file transports and flushes idle acknowledgement work.
- **`mtproto-persist`.** Coalesces session writes, retries with bounded backoff, and marks persistence as exhausted after repeated failures.
- **`mtproto-transfer-config`.** Refreshes server transfer limits (`help.getAppConfig` queue settings) without blocking a user call.

Thread-local state is the dispatch class (`set_dispatch_class`), the bound request-control token (`bind_request_control`) and the transfer policy used by media and scheduler code.

A handle is created once, used from several threads, and destroyed once. `destroyClient` removes it from the registry, flushes the session and closes registered sockets. Later calls fail with `UnknownClient`. Don't recreate a handle for the same authorization in the same process.

## Scheduling

The lane families and dispatch classes are listed in [USAGE: Scheduling](USAGE.md#scheduling). This section covers how they work.

**Why lanes exist.** A single MTProto connection serializes everything on it. A 50 MB download would delay a history request behind it, and a stalled upload would delay everything else. Lanes put each kind of traffic on its own socket, so work only queues behind its own kind.

**Extra read lanes** need both the server's `config.tmp_sessions` allowance and a successfully bound temporary key. If either is missing, reads run on the main lane. Media lanes open on demand, and upload lanes are fixed. See [Parallel sessions](https://core.telegram.org/api/datacenter#parallel-sessions).

**LaneGate** admits the best waiter, FIFO within a priority. Waiters above priority 2 move up one step every 5 seconds, down to priority 1. Interactive writes stay at priority 0. Cancellation is checked while a request is queued, so a cancelled waiter leaves the queue without ever holding a lane.

**Aging** answers starvation. Without it, steady interactive traffic would keep background work waiting forever. With it, a background request gains a step every five seconds until it competes with interactive media. The floor at 1 keeps interactive writes (0) strictly ahead.

**Profiles from classes.** Classes 3, 5, 6 and 7 also set a download profile. Classes 5, 6 and 7 share interactive-media priority but differ in chunk size and parts in flight. `promoteRequestControl` changes queue priority for waiting read requests, not a download profile.

**Admission** is per datacenter and per transfer size. The defaults are five small and two large transfers (20 MB and up). `help.getAppConfig` can supply other queue limits, but large transfers stay capped at two either way. While a DC is recovering from a flood wait and still has active transfers, new admissions wait, and they resume once the active count reaches zero.

## RPC path

### Request lifecycle

The API encodes a TL request and calls `invoke_encoded` or a batch variant. On a fresh socket the first request is wrapped in `invokeWithLayer(initConnection(...))`, and later requests go out bare. Non-update lanes use `invokeWithoutUpdates` so the connection doesn't subscribe to updates.

`rpc/invoke.rs` runs attempts under a supervisor:

1. Take a parked transport or dial an endpoint. Endpoints rotate DC IPs and ports, 443 first.
2. Ask the engine for the next outbound message.
3. Frame it, padded to a multiple of 16 bytes.
4. Send, read, and feed packets to the engine until the correlated response arrives.

A transport is parked only if the exchange ended on a clean frame boundary. A timed-out or half-read transport is closed, because the next request on it could be answered with leftover bytes.

Specs: [MTProto](https://core.telegram.org/mtproto), [Invoking methods](https://core.telegram.org/api/invoking).

### Recovery

- `bad_server_salt`: cache the salt windows and resend.
- `bad_msg_notification`: timing and sequence codes trigger session-id recovery. Codes 19 and 64 abort the request.
- DC migration: `PHONE_MIGRATE_X`, `NETWORK_MIGRATE_X` and `FILE_MIGRATE_X` re-route the call. `FILE_MIGRATE_X` changes only the media DC, and media on a foreign DC uses `auth.exportAuthorization` and a temporary key.
- Flood wait: a per-(DC, class) deadline parks transfers. Windows shrink, then recover one step at a time after sustained success.
- Uncertain delivery: reads and idempotent part uploads may be replayed. Mutations that may have reached the server come back as errors.
- Silent requests: overdue requests are probed with `msgs_state_req`. A status showing the server never received the message becomes a timeout, and a mutation is not resent.

Gzip containers are accepted in both directions with an 8 MiB inflation limit, so a small packet can't expand into unbounded memory. Unknown inbound constructors fail with the constructor id and are never skipped. Specs: [Errors](https://core.telegram.org/api/errors), [Service messages](https://core.telegram.org/mtproto/service_messages).

### Timeouts and keepalive

The default RPC budget is 20 seconds. Media windows get 45 seconds and extend while parts keep arriving. A socket idle for 19 seconds counts as stale for keepalive.

### Error shapes

Server failures reach the host as `MtprotoError::Message("RPC <code>: <TYPE>")`. A session the library knows is dead is reported as `session invalidated: <REASON>`, deliberately not shaped like a 401, so the real cause stays visible in logs. Transport failures arrive as text such as `RPC timeout`, `connection error`, `msg_key` or `bad_msg_notification`. Classifying these strings is left to the host.

## Live updates

### What Telegram requires

Telegram pushes events over the last active connection and numbers them. Three independent sequence families exist, and a client has to keep all of them consistent (see [Working with Updates](https://core.telegram.org/api/updates)):

| Sequence | Scope                                                                                                         |
|----------|---------------------------------------------------------------------------------------------------------------|
| `seq`    | The top-level update packets (`updates`, `updatesCombined`)                                                   |
| `pts`    | One per message box: the common box (private chats and basic groups), plus one for each channel or supergroup |
| `qts`    | A secondary box for secret chats and certain bot events                                                       |

Each event carries the counter value after it, and `pts` updates also carry how many events they contain (`pts_count`). The rule for a box with local state `local_pts`:

| Check                          | Meaning             | Action                       |
|--------------------------------|---------------------|------------------------------|
| `local_pts + pts_count == pts` | In order            | Apply, set `local_pts = pts` |
| `local_pts + pts_count > pts`  | Already applied     | Ignore                       |
| `local_pts + pts_count < pts`  | Updates were missed | Fill the gap                 |

`qts` works the same way with a count of one. `seq` has one special case: a `seq_start` of 0 means the update is unordered and applies immediately.

Gaps are filled with `updates.getDifference` (common and secret state) or `updates.getChannelDifference` (one channel). A `differenceSlice` means more is waiting, so the call repeats with the intermediate state, and a channel difference without the `final` flag repeats the same way. A difference can also come back "too long" when the client's state is older than the server keeps. Then the client has to fetch fresh state and refill old history with explicit message fetches.

Telegram lists the situations that need manual recovery: a gap in `seq`, `pts` or `qts` (it suggests allowing a short time for reordered packets to fill the hole first), a "new session created" service notification, data that can't be decoded, an incomplete short-form update, a long silence, and the server's own `updatesTooLong` and `updateChannelTooLong` hints. Updates received on an unencrypted connection must always be ignored.

Message ids form separate sequences too: one shared by private chats and basic groups, one per channel or supergroup, one per secret chat (identified by `random_id`), and separate ones for scheduled and quick-reply messages. The `random_id` on a send is stored by the server permanently and used to deduplicate, and `updateMessageID` ties it to the generated message id.

### What the library does

**Polling and the reorder buffer.** The host calls `startUpdates` once and then polls `drainUpdates`. The crate never calls the host for updates, and `drainUpdates` does nothing until the client is authorized and `startUpdates` has set the flag.

Inbound packets go into a bounded reorder window (256 packets or 8 MiB). Sequenced events wait for their predecessor, which absorbs reordering without asking the server for anything. A gap that can't be resolved triggers difference recovery.

**Journal and acknowledgement.**

1. Durable events are grouped into an `UpdateJournal` batch.
2. The journal is saved with its cursor and channel cursors, capped at 256 unacknowledged batches or 8 MiB.
3. While undelivered batches exist, `drainUpdates` returns `replayRequired`. The host reads `pendingUpdateBatches`, applies them and acknowledges.
4. Acknowledging removes the batch. Until then, a crash replays it.

Non-durable events (typing, presence, sync edges) are returned in `liveEvents` and never journaled. Replaying "user is typing" after a restart would be wrong.

**Cursors.** `pts`, `qts`, `seq` and `date` become durable only after `getDifference` applies a cursor. The head from `updates.getState` is an in-memory bootstrap value and isn't saved, so a crash can't claim work that was never applied. The journal stores each batch with the cursor it advanced, so replay and cursor stay consistent.

**Channels.** Channel `pts` is tracked per channel. Lagging channels enter a recovery queue processed at a measured rate, so catching up after a long offline period doesn't hammer the server or starve other traffic. `updateLazySyncConfig` keeps off-screen gaps durable without recovering them immediately, and exception chat ids stay eager.

**Vocabulary.** `UpdateEventDto` is a closed vocabulary: messages, edits, deletions, typing, presence and emoji status, read receipts, reactions, discussion reads, dialog patches, folder and chat changes, saved GIFs and `SyncState`. Unknown constructors become `Ignored { kind }`. A pending-send confirmation is `UpdateMessageId:<randomId>:<messageId>`.

### Why a journal

An in-memory queue loses events when the process dies between "received from the server" and "applied by the app". Telegram has already advanced its own state for the connection, and the client's saved cursor must not move past work the app never applied. The journal makes the hand-off explicit: the library stores the batch with its cursor, the host applies and commits, and only the host's acknowledgement releases the batch. That is the whole contract, and it's why the order "apply, commit, acknowledge" must never be swapped.

Spec: [Updates](https://core.telegram.org/api/updates).

## Downloads

### Telegram's file rules

Files are fetched in parts with `upload.getFile`. Telegram requires the offset and limit to be multiples of 4 KiB and a request not to cross a 1 MiB boundary, so every chunk size the library offers (128, 256, 512 KiB) divides 1 MiB. Files can live on a different datacenter than the account (`FILE_MIGRATE_X`), can be served from a CDN (`fileCdnRedirect`), and are addressed by a file reference that expires. Specs: [Files](https://core.telegram.org/api/files), [Optimization](https://core.telegram.org/api/optimisation).

### Profile selection

`download_message_media` picks a `DownloadProfile` in this order:

| Request                       | Profile                                |
|-------------------------------|----------------------------------------|
| `Thumb`                       | Thumb                                  |
| `Full` with an offset         | Playing                                |
| Media class 3, 5, 6 or 7 set  | Background, Ordinary, Visible or User  |
| `Display` with no profile     | Visible                                |
| Any other `Full`              | User                                   |

Chunk sizes and parts in flight per profile are in [USAGE: Pick a dispatch class](USAGE.md#pick-a-dispatch-class). Thumb is 128 KiB with one part, and Playing is 512 KiB with eight. `set_download_chunk_kib` can lower the chunk, and `set_download_concurrency` sets the pipeline limit each profile is capped by.

### The pipeline

A download keeps a window of parts in flight on one media lane, sized by the profile. As parts complete, new offsets are issued so the connection stays full instead of waiting a round trip per part. A second media lane may be prepared on a short-lived worker while the first one starts, which hides the cost of opening a socket and binding a temporary key. Part order doesn't matter for correctness, because each part is written at its own offset and recorded in the ranges file.

### Staging and publication

A network download writes to `<dest>.<pid>.<sequence>.part`. The missing ranges go to `...ranges.part` and the file identity (file id, access hash, DC, size, version) to `...identity.part`. The ranges file lets an interrupted transfer resume from its holes instead of throwing away complete bytes. On the next full download to the same destination, the largest staging file is chosen and its identity compared. A mismatch starts over, so bytes from an older version of a file can never be spliced into a new one.

A per-destination lock serializes downloads to the same path and is held for the lifetime of the staging file.

Publication checks the session lease again. The staging file is renamed onto the destination, the sidecars are removed, and orphaned sidecars are swept. Retryable errors and cancellation keep the staging file, and other errors drop it. A stale-staging sweep collects files older than two minutes and, if they exceed 32 MiB, deletes the oldest first until the directory is under the cap.

`.cancel` markers are checked between parts and before publication. The library never removes the marker.

Progress goes through one process-wide callback. Inline media (a `photoStrippedSize` or cached thumbnail) skips the network and is written directly.

### CDN

When `upload.getFile` returns `fileCdnRedirect`, the downloader:

1. fetches `help.getCdnConfig` and selects the public key for the CDN DC
2. builds a temporary auth key from that key
3. requests `upload.getCdnFile` parts
4. decrypts each part with AES-256-CTR, using a counter IV derived from the part offset
5. checks the part against the master-DC `file_hashes` before writing

A part the CDN doesn't have yet is re-uploaded on request (`REUPLOAD_NEEDED`), and after more than three reuploads of one part the download gives up on the CDN. A nonzero CDN offset is not resumed and falls back to the origin DC. Any CDN failure (a missing hash, a hash mismatch, bad ciphertext) also falls back to a plain origin download with the CDN disabled. The CDN is never needed for correctness, and the trust anchor is the master datacenter, not the CDN host. Spec: [CDN verification](https://core.telegram.org/cdn#verifying-files).

### File references

`FILE_REFERENCE_*` failures re-fetch the source constructor (message, sticker set, wallpaper or inline query) and retry once, even if the returned token looks unchanged. Thumbnail failures try the next getFile-sized thumbnail. Spec: [File references](https://core.telegram.org/api/file-references).

### Flood recovery

A flood wait parks the DC and class, halves the window, and then restores one in-flight slot at a time after sustained success. CDN and origin downloads share the same admission policy. Cutting fast and recovering slowly keeps a rate-limited datacenter from being hit again right after the wait ends.

## Media index

`MediaIndex` is an insertion-ordered map from `(chat_id, message_id)` to a `MediaRef`. An entry carries the location, size, kind, cache key, and optional thumb and display locations. Persisted limits:

- 500 message entries
- 200 zero-id document entries (stickers, GIFs, inline files, custom emoji)
- 32 instant-view entries
- unlimited peer avatars

Instant-view inline previews larger than 4 KiB are stripped before saving. Message and avatar inline bytes are saved as they are.

Peer avatars use `(peerId, 0)`. They're inserted by dialog indexing (`getChats`), profile loading and user lists. If a download targets `(peerId, 0)` and the entry is missing, the client refreshes the peer's profile or the dialog list before failing. A profile photo with a video variant is stored as `video_avatar`.

`peek_message_inline_thumb` and `peek_peer_inline_thumb` return an already expanded JPEG or `null`. They make no RPC and skip the main lane lock, but they briefly hold the client data mutex.

## Uploads

Uploads open the source file, read it in parts, and send `upload.saveFilePart` or `upload.saveBigFilePart` over the upload lanes. Part size is 32 KiB or 512 KiB. If the link round trip is one second or more, or the file is 32 KiB or smaller, one 32 KiB part is in flight at a time. Otherwise up to eight are.

Files over 10 MiB use the big-file method, and photos over 10 MiB are rejected. Albums are validated before sending: at most ten items, and photos or videos can't be mixed with documents.

Upload parts are idempotent, which is why a part that may not have arrived can be resent. The final send is a mutation and is not.

## Auth and sessions

**Auth-key creation** runs the standard DH exchange, validates the server's DH parameters, and uses the production or test RSA key. Specs: [MTProto description](https://core.telegram.org/mtproto/description), [auth-key test vectors](https://core.telegram.org/mtproto/samples-auth_key).

### Temporary keys and PFS

Extra read lanes and media lanes use short-lived keys bound with `auth.bindTempAuthKey`. The permanent key stays on the main session. The bind call uses the MTProto v1 encrypted envelope (`auth/bind_temp.rs`).

This gives forward secrecy because traffic on these lanes is encrypted under keys that expire and are never written to disk. Compromising the stored permanent key later doesn't decrypt what was sent under an expired temporary key. For foreign-DC media, the library first exports an authorization to the target DC and then binds a temporary key there. CDN downloads use their own temporary key built from the CDN public key.

Servers grant parallel sessions through `config.tmp_sessions`. Without it, the extra lanes simply aren't used and reads go to the main lane. Specs: [PFS](https://core.telegram.org/api/pfs), [Datacenters](https://core.telegram.org/api/datacenter).

### SRP

Two-factor sign-in lives in `auth/srp.rs` and is driven by `checkPassword`. `signIn` only reports `PasswordRequired`. The password is processed locally to answer the server's challenge and is never sent in the clear. Spec: [SRP](https://core.telegram.org/api/srp).

### Session file and encryption

`FileSessionStore` writes one JSON document with the snapshot, user id, peers, update cursors, channel recovery queue, media index, journal and imported foreign-DC keys. Writes are coalesced by an epoch counter and replaced atomically through a temporary file and rename, so a crash leaves either the old file or the new one, never half of each.

With `create_encrypted_client`, the document is encrypted with AES-256-GCM under a 32-byte key from the caller. GCM authenticates as well as encrypts, which is why a wrong key or a modified file fails the restore outright and leaves the stored session untouched. The library neither wraps nor stores the key. A plaintext session migrates to encrypted on the first encrypted restore.

Persistence stays off the hot path. The RPC result is merged in memory first, and the write happens later on `mtproto-persist`. If repeated writes fail, persistence is marked exhausted rather than blocking calls.

### Dead sessions

A session marked dead (revoked authorization or `AUTH_KEY_DUPLICATED`) keeps its reason across restarts. A new sign-in creates a fresh MTProto session, and the caller decides when to clear stored local state. Spec: [Errors, 406](https://core.telegram.org/api/errors#406-not-acceptable).

## Network and proxies

**DC discovery.** Endpoints come from static production and test lists in `rpc/dc.rs`, with 443 first and other ports rotated to get around blocks. The lists can be extended by the signed `help.configSimple` blob, fetched over DoH from `dc_txt_domain_name` TXT records. The blob is RSA-unwrapped, AES-CBC-decrypted and checksum-verified before its IPs are trusted. Specs: [Datacenters](https://core.telegram.org/api/datacenter), [Config](https://core.telegram.org/api/config).

**Proxies** live in `crates/mono-proto-socket`: SOCKS5, HTTP CONNECT, HTTPS CONNECT and MTProxy. CONNECT replies are validated strictly, and HTTPS uses `rustls-platform-verifier`. MTProxy supports simple, `dd` and `ee` secrets, and `ee` uses fake TLS authenticated by the proxy secret, which is not certificate-verified TLS. Specs: [Transports](https://core.telegram.org/mtproto/mtproto-transports), [Security guidelines](https://core.telegram.org/mtproto/security_guidelines).

**Transports.** The default is padded intermediate. The HTTP transport is available for networks that only pass HTTP.

## Security model

- **Telegram auth keys** are created and used inside Rust. They never cross the binding boundary and never appear in DTOs, logs or `Debug` output.
- **Session data key.** `createEncryptedClient` takes a raw 32-byte key from the embedder. Callers have to zero their copy and protect the key themselves. The Android app keeps a Keystore-wrapped copy.
- **Session file.** AES-256-GCM with an atomic replace. A copy on another device can't be restored when the wrapping key is device-bound.
- **Temporary keys** are bound to the permanent key, kept in process memory, and never saved as plaintext credentials.
- **CDN data** is verified against the master datacenter's hashes before it's written.
- **Logs and perf data** hold fixed labels, error types, timing spans and counters only. Message bodies, phone numbers, codes, API hashes, proxy secrets and session contents are never logged.
- **API credentials** are application credentials. The embedder decides how to store them.
- **Proxy secrets** are raw bytes. Fake TLS authenticates the proxy secret only.
- **Media cache.** Downloaded media can be private. Eviction and clearing belong to the application.
- **Not covered.** The default test suite makes no claims about live Telegram behavior, proxy interoperability or long soak runs, and the library has not had an independent security review.

## Limits and backpressure

Queues are bounded instead of buffered without limit:

| Stage                    | Bound                                                        |
|--------------------------|--------------------------------------------------------------|
| Lane waiting             | Priority gate, FIFO, 5-second aging                          |
| Download admission       | 5 small and 2 large per DC by default; large capped at 2     |
| Download window          | Chunk 128, 256 or 512 KiB; 1 to 8 parts in flight            |
| Media lanes              | 2 by default, 8 at most, opened as needed                    |
| Upload parts             | Up to 8 in flight, 32 or 512 KiB each                        |
| Update reorder buffer    | 256 packets or 8 MiB                                         |
| Update journal           | 256 unacknowledged batches or 8 MiB                          |
| Gzip inflation           | 8 MiB                                                        |
| Media index              | 500 messages, 200 zero-id documents, 32 instant-view entries |
| Instant-view inline data | 4 KiB per entry                                              |
| Staging cleanup          | Files older than 120 s; 32 MiB directory cap                 |

More lanes or parts mean more sockets, temporary keys, memory and server load. Raise them only for a measured workload, because the server can still answer with flood waits.

Performance telemetry is opt-in. `perf_set_enabled(true)` records fixed-name spans (API calls, first media chunk, persist, lane waits) and counters, and `perf_snapshot(reset)` returns them as JSON with per-operation counts, total and max microseconds, and a small sample ring.

## Failure matrix

| Situation                                    | What happens                                                              |
|----------------------------------------------|---------------------------------------------------------------------------|
| Packets arrive out of order                  | The reorder buffer holds them until the predecessor arrives               |
| A real gap in `pts`, `qts` or `seq`          | Difference recovery runs; the batch then enters the journal               |
| Process dies before acknowledgement          | The batch is replayed on the next `drainUpdates`                          |
| Process dies mid-download                    | Staging file and sidecars stay; the next call resumes                     |
| Staging file belongs to another file version | Identity mismatch; the download starts over                               |
| CDN returns a bad or unhashed part           | Fall back to the origin datacenter                                        |
| `FILE_REFERENCE_*`                           | Re-fetch the source and retry once                                        |
| `FILE_MIGRATE_X`                             | Re-route to the media DC; export authorization if needed                  |
| Flood wait on a datacenter                   | Transfers park; the window halves and recovers slowly                     |
| Server says `bad_server_salt`                | Cache the salt windows and resend                                         |
| Timing or sequence `bad_msg_notification`    | Session-id recovery; codes 19 and 64 abort                                |
| Request goes silent                          | `msgs_state_req` probe; a mutation is not resent                          |
| Reply may have been lost for a send          | Matched later by `UpdateMessageId` and the random id                      |
| Authorization revoked or key duplicated      | Session marked dead; calls fail fast; caller signs in again               |
| Wrong session key or damaged file            | Restore fails; the stored session is left unchanged                       |
| Session write fails repeatedly               | Persistence marked exhausted; calls are not blocked                       |
| Unknown inbound constructor                  | Fails with the constructor id; never skipped                              |
| Mixed TL layers in one build                 | Unknown constructors or decode failures; avoid by regenerating all layers |

## TL layers and schema

A TL layer is a frozen snapshot of constructor and method IDs, field layouts and types. Telegram changes the schema between layers, so constructors and signatures from different layers must never be mixed. Specs: [TL layers](https://core.telegram.org/api/layers), [TL schema](https://core.telegram.org/schema), [invokeWithLayer](https://core.telegram.org/method/invokeWithLayer).

### Pinned layer

This checkout is pinned to layer 232:

- `mono-proto` is `0.2.0+layer.232`.
- The default Cargo feature is `layer-232`, and `LATEST_API_LAYER` is 232.
- Generated modules for `layer-223`, `layer-229`, `layer-230`, `layer-231` and `layer-232` exist side by side.
- A platform adapter's `libraryVersion()` can report its adapter version and the selected runtime layer. The library layer is always available as `mono-proto::LATEST_API_LAYER`.

The runtime decodes and encodes with the generated constructor IDs of the selected layer, and `api_invoke` wraps the first request on a fresh socket in `invokeWithLayer(initConnection(...))`.

Older layers stay for comparison, migration and generator tests. The layer is chosen at build time. There's no call to switch it, and mixing modules would produce an inconsistent constructor table. A mixed build shows up as unknown constructors, `TYPE_CONSTRUCTOR_INVALID`, decode failures or requests the server rejects. Unknown inbound constructors fail closed with the constructor id in the error.

### Schema explorer

[schema.jppgr.am](https://schema.jppgr.am/) archives TL layers. It's useful for diagnosing wire mismatches and planning a bump: search constructors, methods and types, diff layers, look up errors and decode payloads.

- Raw pinned layer: [layers/232.tl](https://schema.jppgr.am/layers/232.tl)
- Layer 231 to 232 diff: [diff/231-232](https://schema.jppgr.am/diff/231-232)
- Changelog feed: [api/feed](https://schema.jppgr.am/api/feed)

It archives layers newer than this checkout. A constructor found there isn't usable until the schema is regenerated, so check the ID in `layers/232.tl` first.

### Upgrading a layer

A layer bump is a protocol migration, not a dependency bump.

1. Choose a schema snapshot listed in [THIRD_PARTY_NOTICES](THIRD_PARTY_NOTICES.md), verify its SHA-256, and keep the digest in the change record.
2. Add the new pinned snapshot and its digest to the schema manifest. Regenerate declarations with `cargo xtask generate-schemas`, which verifies the snapshot digest before parsing and emitting Rust; preserve the generator crates' MPL-2.0 notices and never edit generated declarations by hand.
3. Copy the generated layer modules into the `mono-proto` crate. Update its default and latest feature and the Cargo locks, and keep the older layers.
4. Compile the workspace and add explicit handling for every new or changed variant in the owning library modules. A host adapter regenerates its own bindings separately if its exported surface changes.
5. Run `cargo xtask check-schemas`, `cargo xtask test-layers`, the host, transport and wrapper tests, and check that `mono-proto::LATEST_API_LAYER` reports the new layer.
6. Make sure stored login identity and update cursors still restore. Never drop them to make a build pass.

## Maintenance

- **Host bindings.** Adapter repositories rebuild generated bindings and never edit them. A binding export change needs the adapter's regeneration and wrapper checks, and doesn't touch the protocol implementation here.
- **Layer generation.** Use the pinned snapshots, the generator checks and `cargo xtask test-layers` before changing generated declarations.
- **Session compatibility.** A change to the session format needs a forward-compatible restore path: handle the old document explicitly and test the migration. Encrypted sessions stay AES-256-GCM.
- **Android error catalog.** An Android app can classify errors with `TelegramErrorCatalog.kt`, generated from the official `errors.json` by `scripts/gen_telegram_errors.py`. It's app code, not part of the crate.
- **Versions.** Workspace crate versions live in the owning `crates/*/Cargo.toml` files, and the TL layer is `mono-proto::LATEST_API_LAYER`. Adapter library versions are maintained by the host integration.

## Extending the library

**Add an exported function.**

1. Implement the operation in the owning crate (`mono-proto-client`, `mono-proto-messages`, `mono-proto-media`, `mono-proto-runtime` or `mono-proto-updates`). Go through the client access helpers when it touches client state, and pick the lane family that matches the work.
2. If a host adapter exposes it, add only the adapter declaration and DTO conversion there. Don't move protocol behavior into the adapter.
3. Map errors through the public typed error model, and keep local dead-session reasons intact.
4. Add a focused unit test, and a wire test if it sends a new RPC.
5. Add the function to the [API index](USAGE.md#api-index) and document its dispatch class.

**Add an update event.**

1. Extend `UpdateEventDto` and map the TL update in the update path.
2. Decide whether it's durable (journaled) or live-only.
3. Take it out of `Ignored` only after the host handles the new variant, and regenerate bindings in the same change.

**Support a new TL variant after a layer bump.** Add explicit DTO handling for every changed variant that crosses the binding boundary, as in step 4 of [Upgrading a layer](#upgrading-a-layer).

**Change the journal or session shape.** Both are contracts: the journal with the host's acknowledgement protocol, and the session with every stored file already on users' devices. Change them with a migration and a test, never in place.

## Testing

Host tests need no device, account or live Telegram connection:

```sh
cargo test --workspace --offline
cargo xtask test-layers
```

Adapter repositories should also run their own binding regeneration and contract checks when the adapter surface changes.

| Layer               | What it checks                                                                  |
|---------------------|---------------------------------------------------------------------------------|
| Rust unit tests     | Scheduler, journal, DTO mapping, session crypto, media policy                   |
| RPC wire tests      | Encrypted local sockets, fragmented replies, correlation, retries, cancellation |
| Transport tests     | SOCKS5, HTTP, HTTPS and MTProxy handshakes; TLS and fake-TLS validation        |
| Media orchestration | Staging, hash verification, CDN fallback, publication lease                     |

Client tests feed crafted inbound packets into the deterministic engine to cover recovery paths. Transport tests use real local TCP servers, including fragmented writes. A host bridge can add its own tests for DTO mapping, error parsing, update delivery and session lifecycle.

The default suite doesn't cover live Telegram behavior, proxy interoperability, long soak runs or an independent security review. These stay manual: login code types and 2FA, reconnects and DC migration during a call, large downloads with resume and cancel, background updates after a process restart, and proxy or network changes.

After a change, run the narrowest relevant test plus the layer above it. Protocol, auth, media and session changes always need both host suites and the wrapper tests.

## Invariants

- The permanent auth key belongs to one handle in one process. Temporary lane keys are bound from it and never shared.
- Outgoing random ids are fresh per logical message, and the caller keeps pending sends until `UpdateMessageId` arrives.
- Cursors advance only through applied difference. Channel `pts` is separate state.
- A transport whose auth key, session id or DC changed is dropped, never pooled.
- Merge-back stays field-diffed. Assigning state wholesale brings back lost-update races between lanes.
- Telegram auth keys never cross the binding boundary. The session data key does, as 32 raw bytes passed to `create_encrypted_client`, and callers must zero their copy. Keys never appear in logs or `Debug` output.
- The update journal is a delivery contract with the host. Changing its shape means changing the acknowledgement protocol and the host wrapper.
- Generated bindings are rebuilt, never hand-edited.
- A download publishes only through the session lease, so a changed session can't publish an old transfer.
- CDN data is written only after hash verification.

## References

Telegram pages this document relies on. The full list lives in [USAGE: Telegram documentation map](USAGE.md#telegram-documentation-map).

| Topic                 | Page                                                                                                                     |
|-----------------------|--------------------------------------------------------------------------------------------------------------------------|
| MTProto overview      | [https://core.telegram.org/mtproto](https://core.telegram.org/mtproto)                                                   |
| MTProto description   | [https://core.telegram.org/mtproto/description](https://core.telegram.org/mtproto/description)                           |
| Transports            | [https://core.telegram.org/mtproto/mtproto-transports](https://core.telegram.org/mtproto/mtproto-transports)             |
| Service messages      | [https://core.telegram.org/mtproto/service_messages](https://core.telegram.org/mtproto/service_messages)                 |
| Security guidelines   | [https://core.telegram.org/mtproto/security_guidelines](https://core.telegram.org/mtproto/security_guidelines)           |
| Auth-key vectors      | [https://core.telegram.org/mtproto/samples-auth_key](https://core.telegram.org/mtproto/samples-auth_key)                 |
| Authorization         | [https://core.telegram.org/api/auth](https://core.telegram.org/api/auth)                                                 |
| Code types            | [https://core.telegram.org/api/auth#code-types](https://core.telegram.org/api/auth#code-types)                           |
| SRP                   | [https://core.telegram.org/api/srp](https://core.telegram.org/api/srp)                                                   |
| Errors                | [https://core.telegram.org/api/errors](https://core.telegram.org/api/errors)                                             |
| Errors JSON           | [https://core.telegram.org/api/errors.json](https://core.telegram.org/api/errors.json)                                   |
| Invoking              | [https://core.telegram.org/api/invoking](https://core.telegram.org/api/invoking)                                         |
| Layers                | [https://core.telegram.org/api/layers](https://core.telegram.org/api/layers)                                             |
| TL schema explorer    | [https://schema.jppgr.am/](https://schema.jppgr.am/)                                                                     |
| Pinned layer 232      | [https://schema.jppgr.am/layers/232.tl](https://schema.jppgr.am/layers/232.tl)                                           |
| Layer 231 to 232 diff | [https://schema.jppgr.am/diff/231-232](https://schema.jppgr.am/diff/231-232)                                             |
| Layer changelog feed  | [https://schema.jppgr.am/api/feed](https://schema.jppgr.am/api/feed)                                                     |
| Updates               | [https://core.telegram.org/api/updates](https://core.telegram.org/api/updates)                                           |
| Files                 | [https://core.telegram.org/api/files](https://core.telegram.org/api/files)                                               |
| File references       | [https://core.telegram.org/api/file-references](https://core.telegram.org/api/file-references)                           |
| CDN verification      | [https://core.telegram.org/cdn#verifying-files](https://core.telegram.org/cdn#verifying-files)                           |
| Datacenters           | [https://core.telegram.org/api/datacenter](https://core.telegram.org/api/datacenter)                                     |
| Parallel sessions     | [https://core.telegram.org/api/datacenter#parallel-sessions](https://core.telegram.org/api/datacenter#parallel-sessions) |
| PFS                   | [https://core.telegram.org/api/pfs](https://core.telegram.org/api/pfs)                                                   |
| Optimization          | [https://core.telegram.org/api/optimisation](https://core.telegram.org/api/optimisation)                                 |
| Config                | [https://core.telegram.org/api/config](https://core.telegram.org/api/config)                                             |
| Folders               | [https://core.telegram.org/api/folders](https://core.telegram.org/api/folders)                                           |
| Offsets               | [https://core.telegram.org/api/offsets](https://core.telegram.org/api/offsets)                                           |

**Protocol:** [MTProto index](https://core.telegram.org/mtproto), [TL schema](https://core.telegram.org/schema), [layers](https://core.telegram.org/api/layers), [invoking](https://core.telegram.org/api/invoking), [datacenters](https://core.telegram.org/api/datacenter), [PFS](https://core.telegram.org/api/pfs), [config](https://core.telegram.org/api/config).

**Data model:** [peers](https://core.telegram.org/api/peers), [minimum constructors](https://core.telegram.org/api/min), [entities](https://core.telegram.org/api/entities), [links](https://core.telegram.org/api/links), [channels](https://core.telegram.org/api/channel), [discussion](https://core.telegram.org/api/discussion), [forum](https://core.telegram.org/api/forum), [threads](https://core.telegram.org/api/threads).

**Media:** [files](https://core.telegram.org/api/files), [file references](https://core.telegram.org/api/file-references), [CDN](https://core.telegram.org/cdn#verifying-files), [optimization](https://core.telegram.org/api/optimisation), [stickers](https://core.telegram.org/api/stickers), [custom emoji](https://core.telegram.org/api/custom-emoji), [wallpapers](https://core.telegram.org/api/wallpapers).

**Methods:** [auth.bindTempAuthKey](https://core.telegram.org/method/auth.bindTempAuthKey), [upload.getFile](https://core.telegram.org/method/upload.getFile), [updates.getDifference](https://core.telegram.org/method/updates.getDifference), [help.getAppConfig](https://core.telegram.org/method/help.getAppConfig).
