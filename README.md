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
- `EpgChannel`
- `EpgProgramme`
- `StreamUrl`
- `StreamProtocol`
- `StreamFormat`
- `StreamStatus`
- `Resolution`
- `VodEntry`
- `VodType`
- `VodCategory`
- `IptvError`

## Installation

```toml
[dependencies]
crispy-iptv-types = "0.1.1"
```

MSRV: Rust `1.85`

## Quick Start

```rust
use crispy_iptv_types::{PlaylistEntry, StreamFormat, StreamProtocol, StreamUrl};

let url = StreamUrl::classify("http://example.com/live/cnn.m3u8");

let mut entry = PlaylistEntry {
    name: Some("CNN".into()),
    tvg_id: Some("cnn.us".into()),
    ..Default::default()
};
entry.set_primary_url(url.url.clone());

assert_eq!(url.protocol, StreamProtocol::Http);
assert_eq!(url.format, StreamFormat::Hls);
assert_eq!(entry.primary_url(), Some("http://example.com/live/cnn.m3u8"));
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
- `StreamUrl::classify` is best-effort classification; `StreamUrl::try_parse` performs strict URL validation
- `StreamUrl` preserves transport (`StreamProtocol`) separately from stream format (`StreamFormat`)
- `PlaylistEntry` uses `urls[0]` as the canonical primary URL, with `primary_url()` and `set_primary_url()` helpers
- `EpgChannel` keeps singular `icon` / `url` compatibility fields, but `icons` / `urls` are canonical; `normalize_legacy_fields()` now reconciles conflicts by mirroring the plural primary entry back into the singular field
- `EpgPerson` now preserves ordered XMLTV mixed credit content through `EpgPersonContent::{Text, Image, Url}`, so image-only, url-only, and interleaved credit nodes no longer lose ordering
- `EpgReview.review_type` is XMLTV-aligned and required via the constrained `EpgReviewType::{Text, Url}` enum
- `EpgProgramme` models a broader XMLTV subset including language, country, subtitles, rating icons, prior-showing metadata, and explicit length units

## Current Limitations

- this crate defines shared models only; it does not validate provider-specific business rules
- field-level semantics can still vary by upstream protocol, so callers should normalize where needed
- `EpgPerson` is intentionally XMLTV-centric now; downstream adapters that previously treated credits as a flat `name` plus side arrays should map from ordered `content`
- semver should be treated as pre-1.0 while the shared model surface is still settling

## License

This repository currently ships with the same license and notice model as the parent CrispyTivi project. Review `LICENSE.md` and `NOTICE.md` before adopting it in downstream projects.
