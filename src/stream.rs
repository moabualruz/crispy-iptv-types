//! Stream URL types and transport / format classification.

use serde::{Deserialize, Serialize};
use url::Url;

use crate::error::IptvError;

/// A stream URL with independently classified transport and stream format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamUrl {
    /// The raw URL string.
    pub url: String,
    /// Detected transport / scheme.
    pub protocol: StreamProtocol,
    /// Detected stream format / manifest kind.
    pub format: StreamFormat,
}

impl StreamUrl {
    /// Build a classified stream URL from a raw string without validation.
    ///
    /// This constructor preserves best-effort transport and stream-format hints
    /// even for non-URL inputs. Callers that need strict URL validation should
    /// use [`StreamUrl::try_parse`].
    pub fn classify(url: &str) -> Self {
        let parsed = Url::parse(url).ok();
        let protocol = parsed
            .as_ref()
            .map_or_else(|| StreamProtocol::from_raw(url), StreamProtocol::from_url);
        let format = parsed
            .as_ref()
            .map_or_else(|| StreamFormat::detect(url), StreamFormat::from_url);

        Self {
            url: url.to_string(),
            protocol,
            format,
        }
    }

    /// Convenience constructor equivalent to [`StreamUrl::classify`].
    pub fn from_raw(url: impl Into<String>) -> Self {
        let url = url.into();
        Self::classify(&url)
    }

    /// Strictly parse and classify a URL.
    pub fn try_parse(url: &str) -> Result<Self, IptvError> {
        let parsed = Url::parse(url).map_err(|err| IptvError::InvalidUrl(err.to_string()))?;
        Ok(Self {
            url: url.to_string(),
            protocol: StreamProtocol::from_url(&parsed),
            format: StreamFormat::from_url(&parsed),
        })
    }
}

/// Detected transport / scheme.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StreamProtocol {
    Http,
    Https,
    Rtmp,
    Rtmps,
    Rtsp,
    Udp,
    Rtp,
    Mms,
    Mmsh,
    #[default]
    Unknown,
}

impl StreamProtocol {
    /// Detect the transport from a raw URL string without validation.
    pub fn detect(url: &str) -> Self {
        Self::from_raw(url)
    }

    fn from_url(url: &Url) -> Self {
        match url.scheme().to_ascii_lowercase().as_str() {
            "http" => Self::Http,
            "https" => Self::Https,
            "rtmp" => Self::Rtmp,
            "rtmps" => Self::Rtmps,
            "rtsp" => Self::Rtsp,
            "udp" => Self::Udp,
            "rtp" => Self::Rtp,
            "mms" => Self::Mms,
            "mmsh" => Self::Mmsh,
            _ => Self::Unknown,
        }
    }

    fn from_raw(url: &str) -> Self {
        let lower = url.to_ascii_lowercase();
        if lower.starts_with("http://") {
            Self::Http
        } else if lower.starts_with("https://") {
            Self::Https
        } else if lower.starts_with("rtmp://") {
            Self::Rtmp
        } else if lower.starts_with("rtmps://") {
            Self::Rtmps
        } else if lower.starts_with("rtsp://") {
            Self::Rtsp
        } else if lower.starts_with("udp://") {
            Self::Udp
        } else if lower.starts_with("rtp://") {
            Self::Rtp
        } else if lower.starts_with("mms://") {
            Self::Mms
        } else if lower.starts_with("mmsh://") {
            Self::Mmsh
        } else {
            Self::Unknown
        }
    }
}

/// Detected stream manifest / media kind.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StreamFormat {
    #[default]
    Unknown,
    Hls,
    Dash,
    TransportStream,
}

impl StreamFormat {
    /// Detect the stream format from a raw URL string without validation.
    pub fn detect(url: &str) -> Self {
        if let Ok(parsed) = Url::parse(url) {
            return Self::from_url(&parsed);
        }

        let lower = raw_path(url).to_ascii_lowercase();
        if lower.ends_with(".m3u8") {
            Self::Hls
        } else if lower.ends_with(".mpd") {
            Self::Dash
        } else if lower.ends_with(".ts") {
            Self::TransportStream
        } else {
            Self::Unknown
        }
    }

    fn from_url(url: &Url) -> Self {
        let path = url.path().to_ascii_lowercase();
        if path.ends_with(".m3u8") {
            Self::Hls
        } else if path.ends_with(".mpd") {
            Self::Dash
        } else if path.ends_with(".ts") {
            Self::TransportStream
        } else {
            Self::Unknown
        }
    }
}

fn raw_path(url: &str) -> &str {
    let end = url.find(['?', '#']).unwrap_or(url.len());
    &url[..end]
}

/// Result of checking a stream's availability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamStatus {
    /// Whether the stream is reachable.
    pub available: bool,
    /// HTTP status code (if applicable).
    pub status_code: Option<u16>,
    /// Response time in milliseconds.
    pub response_time_ms: Option<u64>,
    /// Detected content type.
    pub content_type: Option<String>,
    /// Error message if unavailable.
    pub error: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_hls_preserves_https_transport() {
        let stream = StreamUrl::classify("https://example.com/live/stream.m3u8");
        assert_eq!(stream.protocol, StreamProtocol::Https);
        assert_eq!(stream.format, StreamFormat::Hls);
    }

    #[test]
    fn detect_dash_preserves_http_transport() {
        let stream = StreamUrl::classify("http://cdn.example.com/manifest.mpd");
        assert_eq!(stream.protocol, StreamProtocol::Http);
        assert_eq!(stream.format, StreamFormat::Dash);
    }

    #[test]
    fn detect_rtmps_distinct_from_rtmp() {
        let secure = StreamUrl::classify("rtmps://cdn.example.com/live/key");
        let insecure = StreamUrl::classify("rtmp://cdn.example.com/live/key");

        assert_eq!(secure.protocol, StreamProtocol::Rtmps);
        assert_eq!(insecure.protocol, StreamProtocol::Rtmp);
    }

    #[test]
    fn detect_transport_stream_without_collapsing_transport() {
        let stream = StreamUrl::classify("http://example.com/video.ts");
        assert_eq!(stream.protocol, StreamProtocol::Http);
        assert_eq!(stream.format, StreamFormat::TransportStream);
    }

    #[test]
    fn detect_unknown_for_invalid_raw_value() {
        let stream = StreamUrl::classify("not a url");
        assert_eq!(stream.protocol, StreamProtocol::Unknown);
        assert_eq!(stream.format, StreamFormat::Unknown);
    }

    #[test]
    fn detect_scheme_less_hls_with_query_suffix() {
        let stream = StreamUrl::classify("live/channel.m3u8?token=abc123");
        assert_eq!(stream.protocol, StreamProtocol::Unknown);
        assert_eq!(stream.format, StreamFormat::Hls);
    }

    #[test]
    fn detect_scheme_less_dash_with_fragment_suffix() {
        let stream = StreamUrl::classify("manifests/event.mpd#primary");
        assert_eq!(stream.protocol, StreamProtocol::Unknown);
        assert_eq!(stream.format, StreamFormat::Dash);
    }

    #[test]
    fn detect_scheme_less_transport_stream_with_query_and_fragment() {
        let stream = StreamUrl::classify("segments/chunk.ts?token=abc#frag");
        assert_eq!(stream.protocol, StreamProtocol::Unknown);
        assert_eq!(stream.format, StreamFormat::TransportStream);
    }

    #[test]
    fn try_parse_rejects_invalid_url() {
        let err = StreamUrl::try_parse("not a url").unwrap_err();
        assert!(matches!(err, IptvError::InvalidUrl(_)));
    }

    #[test]
    fn try_parse_validates_and_classifies() {
        let stream = StreamUrl::try_parse("https://cdn.example.com/live/stream.m3u8").unwrap();
        assert_eq!(stream.protocol, StreamProtocol::Https);
        assert_eq!(stream.format, StreamFormat::Hls);
    }
}
