# crispy-iptv-types

Protocol-agnostic IPTV domain types shared across the `crispy-*` Rust crates.

## What This Crate Is

`crispy-iptv-types` is the foundation crate for the rest of the ecosystem. It contains the common data model used by playlist parsers, EPG readers, provider clients, stream validators, and playlist tooling.

If you need a stable shared vocabulary for IPTV data without pulling in any app-specific persistence, FFI, or UI code, this is the crate to start with.

## What It Contains

- playlist and channel-facing types in `channel`
- EPG and XMLTV-facing types in `epg`
- VOD-facing types in `vod`
- stream URL, protocol, and status types in `stream`
- video resolution helpers in `resolution`
- a protocol-neutral error type in `error`

The crate re-exports the most commonly used types at the root:

- `PlaylistEntry`
- `CatchupConfig`
- `CatchupType`
- `EpgProgramme`
- `StreamUrl`
- `StreamProtocol`
- `StreamStatus`
- `Resolution`
- `VodEntry`
- `IptvError`

## Installation

```toml
[dependencies]
crispy-iptv-types = "0.1.1"
```

MSRV: Rust `1.85`

## Quick Start

```rust
use crispy_iptv_types::{PlaylistEntry, StreamProtocol, StreamStatus, StreamUrl};

let url = StreamUrl::new("http://example.com/live/cnn.m3u8").unwrap();

let entry = PlaylistEntry {
    id: Some("cnn".into()),
    name: Some("CNN".into()),
    urls: smallvec::smallvec![url.raw.clone()],
    stream_protocol: Some(StreamProtocol::Hls),
    stream_status: Some(StreamStatus::Unknown),
    ..Default::default()
};

assert_eq!(entry.name.as_deref(), Some("CNN"));
assert_eq!(entry.urls.len(), 1);
```

## When To Use It

Use this crate when you need:

- a common schema across multiple IPTV crates
- conversions between protocol-specific parsers/clients and app-specific models
- a shared EPG/playlist/VOD vocabulary

## When Not To Use It

Do not use this crate for:

- storage models tied to a database schema
- UI models
- network client logic
- parsing or serialization by itself

Use the higher-level crates for that:

- `crispy-m3u`
- `crispy-xmltv`
- `crispy-xtream`
- `crispy-stalker`
- `crispy-iptv-tools`
- `crispy-catchup`
- `crispy-media-probe`
- `crispy-stream-checker`

## Design Notes

- types are protocol-neutral where possible
- serde support is built in
- compact strings and small vectors are used where they materially help payload-heavy IPTV data

## Current Limitations

- this crate defines shared models only; it does not validate provider-specific business rules
- field-level semantics can still vary by upstream protocol, so callers should normalize where needed
- semver should be treated as pre-1.0 while the shared model surface is still settling

## License

This repository currently ships with the same license and notice model as the parent CrispyTivi project. Review `LICENSE.md` and `NOTICE.md` before adopting it in downstream projects.
