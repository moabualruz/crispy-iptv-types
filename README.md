# crispy-iptv-types

Protocol-agnostic IPTV domain types and shared vocabulary for the `crispy-*` Rust ecosystem.

## Status

Early-stage extracted library from CrispyTivi. Intended to become the foundational crate used by protocol parsers, API clients, and IPTV tooling.

## What This Crate Provides

- shared channel and playlist types
- shared EPG data structures
- shared VOD data structures
- shared stream URL and status types
- shared resolution model
- shared protocol-neutral error surface

## Why It Exists

Most IPTV crates need the same vocabulary:
- playlist entries
- stream URLs and protocols
- EPG programmes
- VOD metadata
- catchup configuration

This crate keeps that shared vocabulary in one place so parser/client/tool crates can compose cleanly without duplicating models.

## Installation

```toml
[dependencies]
crispy-iptv-types = "0.1"
```

## Quick Start

```rust
use crispy_iptv_types::{PlaylistEntry, StreamUrl};

let entry = PlaylistEntry {
    id: Some("cnn".into()),
    name: Some("CNN".into()),
    url: StreamUrl::new("http://example.com/live/cnn.m3u8").unwrap(),
    ..Default::default()
};

assert_eq!(entry.name.as_deref(), Some("CNN"));
```

## Core Modules

- `channel` — playlist and channel-facing structures
- `epg` — XMLTV/EPG-oriented structures
- `vod` — movie/series/episode oriented structures
- `stream` — stream URL, protocol, and liveness/status structures
- `resolution` — video resolution model
- `error` — shared protocol-neutral error type

## Intended Consumers

- `crispy-m3u`
- `crispy-xmltv`
- `crispy-xtream`
- `crispy-stalker`
- `crispy-iptv-tools`
- `crispy-catchup`
- `crispy-media-probe`
- `crispy-stream-checker`

## Non-Goals

- app-specific database models
- Flutter/FFI concerns
- persistence
- transport-specific API clients

## License

Review project licensing before public publication. This README draft does not finalize licensing policy.
