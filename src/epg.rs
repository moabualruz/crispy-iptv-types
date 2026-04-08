//! Protocol-agnostic EPG (Electronic Programme Guide) types.
//!
//! These types are shared across XMLTV-oriented and provider-oriented crates.
//! The XMLTV-facing structures preserve the subset of the XMLTV DTD that this
//! ecosystem currently models explicitly, including programme metadata,
//! ratings, subtitles, and channel URL/icon compatibility helpers.

use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

/// A single EPG programme entry.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EpgProgramme {
    /// Channel ID this programme belongs to (XMLTV channel@id).
    pub channel: String,

    /// Programme start time (UTC timestamp).
    pub start: Option<i64>,

    /// Programme stop time (UTC timestamp).
    pub stop: Option<i64>,

    /// Optional XMLTV broadcast-related timestamps/IDs.
    pub pdc_start: Option<String>,
    pub vps_start: Option<String>,
    pub showview: Option<String>,
    pub videoplus: Option<String>,
    pub clumpidx: Option<String>,

    /// Programme title(s), potentially multilingual.
    #[serde(default)]
    pub title: SmallVec<[EpgStringWithLang; 1]>,

    /// Subtitle(s).
    #[serde(default)]
    pub sub_title: SmallVec<[EpgStringWithLang; 1]>,

    /// Description(s).
    #[serde(default)]
    pub desc: SmallVec<[EpgStringWithLang; 1]>,

    /// Category / genre tags.
    #[serde(default)]
    pub category: SmallVec<[EpgStringWithLang; 2]>,

    /// Keyword(s) for the programme.
    #[serde(default)]
    pub keyword: SmallVec<[EpgStringWithLang; 2]>,

    /// Language(s) of the programme.
    #[serde(default)]
    pub language: SmallVec<[EpgStringWithLang; 1]>,

    /// Original language of the programme.
    pub orig_language: Option<EpgStringWithLang>,

    /// Credits (directors, actors, writers, etc.).
    pub credits: Option<EpgCredits>,

    /// Original air date.
    pub date: Option<String>,

    /// Programme length with units.
    pub length: Option<EpgLength>,

    /// Channel/programme URLs with optional system attribute.
    #[serde(default)]
    pub url: SmallVec<[EpgUrl; 1]>,

    /// Country/countries associated with the programme.
    #[serde(default)]
    pub country: SmallVec<[EpgStringWithLang; 1]>,

    /// Episode numbering (xmltv_ns, onscreen, etc.).
    #[serde(default)]
    pub episode_num: SmallVec<[EpgEpisodeNumber; 1]>,

    /// Video properties (aspect ratio, colour, quality).
    pub video: Option<EpgVideo>,

    /// Audio properties (stereo mode, presence).
    #[serde(default)]
    pub audio: SmallVec<[EpgAudio; 1]>,

    /// Previous showing metadata.
    pub previously_shown: Option<EpgPreviouslyShown>,

    /// Premiere / last-chance explanation text when available.
    pub premiere: Option<EpgStringWithLang>,
    pub last_chance: Option<EpgStringWithLang>,

    /// Broadcast flags.
    #[serde(default)]
    pub is_new: bool,
    #[serde(default)]
    pub is_premiere: bool,
    #[serde(default)]
    pub is_rerun: bool,
    #[serde(default)]
    pub is_last_chance: bool,

    /// Subtitle metadata.
    #[serde(default)]
    pub subtitles: SmallVec<[EpgSubtitles; 1]>,

    /// Content ratings.
    #[serde(default)]
    pub rating: SmallVec<[EpgRating; 1]>,

    /// Star ratings (critic scores).
    #[serde(default)]
    pub star_rating: SmallVec<[EpgRating; 1]>,

    /// Reviews / critiques.
    #[serde(default)]
    pub review: SmallVec<[EpgReview; 1]>,

    /// Programme images (poster, backdrop, still).
    #[serde(default)]
    pub image: SmallVec<[EpgImage; 1]>,

    /// Programme icon.
    pub icon: Option<EpgIcon>,
}

/// A string value with an optional language attribute.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct EpgStringWithLang {
    pub value: String,
    pub lang: Option<String>,
}

impl EpgStringWithLang {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            lang: None,
        }
    }

    pub fn with_lang(value: impl Into<String>, lang: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            lang: Some(lang.into()),
        }
    }
}

/// Credits for a programme (cast & crew).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct EpgCredits {
    #[serde(default)]
    pub director: SmallVec<[EpgPerson; 2]>,
    #[serde(default)]
    pub actor: SmallVec<[EpgPerson; 4]>,
    #[serde(default)]
    pub writer: SmallVec<[EpgPerson; 2]>,
    #[serde(default)]
    pub adapter: SmallVec<[EpgPerson; 1]>,
    #[serde(default)]
    pub producer: SmallVec<[EpgPerson; 1]>,
    #[serde(default)]
    pub composer: SmallVec<[EpgPerson; 1]>,
    #[serde(default)]
    pub editor: SmallVec<[EpgPerson; 1]>,
    #[serde(default)]
    pub presenter: SmallVec<[EpgPerson; 2]>,
    #[serde(default)]
    pub commentator: SmallVec<[EpgPerson; 1]>,
    #[serde(default)]
    pub guest: SmallVec<[EpgPerson; 2]>,
}

/// A person with optional role, guest flag, and repeated media/link data.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct EpgPerson {
    pub name: String,
    pub role: Option<String>,
    #[serde(default)]
    pub guest: bool,
    #[serde(default, skip_serializing_if = "SmallVec::is_empty")]
    pub images: SmallVec<[String; 1]>,
    #[serde(default, skip_serializing_if = "SmallVec::is_empty")]
    pub urls: SmallVec<[String; 1]>,
}

impl EpgPerson {
    pub fn primary_image(&self) -> Option<&str> {
        self.images.first().map(String::as_str)
    }

    pub fn primary_url(&self) -> Option<&str> {
        self.urls.first().map(String::as_str)
    }
}

/// Episode numbering entry.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct EpgEpisodeNumber {
    pub value: String,
    /// Numbering system: "xmltv_ns", "onscreen", etc.
    pub system: Option<String>,
}

/// Programme image with type metadata.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct EpgImage {
    pub url: String,
    /// Image type: "poster", "backdrop", "still".
    pub image_type: Option<String>,
    pub size: Option<String>,
    pub orient: Option<String>,
    /// Upstream image system/source identifier when available.
    pub system: Option<String>,
}

/// Programme icon (typically a small thumbnail).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct EpgIcon {
    pub src: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

/// Content or star rating.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct EpgRating {
    pub value: String,
    pub system: Option<String>,
    #[serde(default)]
    pub icons: SmallVec<[EpgIcon; 1]>,
}

/// Video properties for a programme (XMLTV `<video>`).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct EpgVideo {
    pub present: Option<bool>,
    pub colour: Option<bool>,
    pub aspect: Option<String>,
    pub quality: Option<String>,
}

/// Audio properties for a programme (XMLTV `<audio>`).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct EpgAudio {
    pub present: Option<bool>,
    pub stereo: Option<String>,
}

/// A review for a programme (XMLTV `<review>`).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct EpgReview {
    pub value: String,
    pub review_type: Option<String>,
    pub source: Option<String>,
    pub reviewer: Option<String>,
    pub lang: Option<String>,
}

/// A URL value with an optional XMLTV `system` attribute.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct EpgUrl {
    pub value: String,
    pub system: Option<String>,
}

/// Programme length with explicit XMLTV units.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EpgLength {
    pub value: u32,
    pub units: EpgLengthUnit,
}

/// XMLTV length units.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum EpgLengthUnit {
    #[default]
    Minutes,
    Seconds,
    Hours,
}

/// Metadata for a prior showing of a programme.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct EpgPreviouslyShown {
    pub start: Option<String>,
    pub channel: Option<String>,
}

/// Subtitle metadata.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct EpgSubtitles {
    pub subtitle_type: Option<EpgSubtitleType>,
    pub language: Option<EpgStringWithLang>,
}

/// XMLTV subtitle types.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum EpgSubtitleType {
    Teletext,
    Onscreen,
    DeafSigned,
}

/// An XMLTV channel definition.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct EpgChannel {
    pub id: String,
    #[serde(default)]
    pub display_name: SmallVec<[EpgStringWithLang; 1]>,
    /// Single icon (backward compat).
    pub icon: Option<EpgIcon>,
    /// Single URL (backward compat).
    pub url: Option<EpgUrl>,
    /// Multiple icons.
    #[serde(default)]
    pub icons: SmallVec<[EpgIcon; 1]>,
    /// Multiple URLs.
    #[serde(default)]
    pub urls: SmallVec<[EpgUrl; 1]>,
}

impl EpgChannel {
    pub fn primary_icon(&self) -> Option<&EpgIcon> {
        self.icons.first().or(self.icon.as_ref())
    }

    pub fn primary_url(&self) -> Option<&EpgUrl> {
        self.urls.first().or(self.url.as_ref())
    }

    pub fn set_primary_icon(&mut self, icon: EpgIcon) {
        if let Some(first) = self.icons.first_mut() {
            *first = icon.clone();
        } else {
            self.icons.push(icon.clone());
        }
        self.icon = Some(icon);
    }

    pub fn set_primary_url(&mut self, url: EpgUrl) {
        if let Some(first) = self.urls.first_mut() {
            *first = url.clone();
        } else {
            self.urls.push(url.clone());
        }
        self.url = Some(url);
    }

    /// Normalize the singular compatibility fields against the canonical plural
    /// vectors and vice versa.
    pub fn normalize_legacy_fields(&mut self) {
        if self.icons.is_empty() {
            if let Some(icon) = self.icon.clone() {
                self.icons.push(icon);
            }
        } else if self.icon.is_none() {
            self.icon = self.icons.first().cloned();
        }

        if self.urls.is_empty() {
            if let Some(url) = self.url.clone() {
                self.urls.push(url);
            }
        } else if self.url.is_none() {
            self.url = self.urls.first().cloned();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn programme_default_has_empty_new_fields() {
        let p = EpgProgramme::default();
        assert!(p.title.is_empty());
        assert!(p.language.is_empty());
        assert!(p.url.is_empty());
        assert!(p.country.is_empty());
        assert!(p.subtitles.is_empty());
        assert!(p.length.is_none());
        assert!(p.previously_shown.is_none());
        assert!(p.channel.is_empty());
        assert!(!p.is_new);
    }

    #[test]
    fn string_with_lang_constructors() {
        let plain = EpgStringWithLang::new("Hello");
        assert_eq!(plain.value, "Hello");
        assert!(plain.lang.is_none());

        let lang = EpgStringWithLang::with_lang("Bonjour", "fr");
        assert_eq!(lang.value, "Bonjour");
        assert_eq!(lang.lang.as_deref(), Some("fr"));
    }

    #[test]
    fn epg_channel_normalizes_plural_from_singular() {
        let mut channel = EpgChannel {
            id: "ch1".into(),
            icon: Some(EpgIcon {
                src: "https://example.com/icon.png".into(),
                width: Some(100),
                height: Some(50),
            }),
            url: Some(EpgUrl {
                value: "https://example.com/watch".into(),
                system: Some("official".into()),
            }),
            ..Default::default()
        };

        channel.normalize_legacy_fields();

        assert_eq!(channel.icons.len(), 1);
        assert_eq!(channel.urls.len(), 1);
        assert_eq!(
            channel.primary_icon().map(|icon| icon.src.as_str()),
            Some("https://example.com/icon.png")
        );
        assert_eq!(
            channel.primary_url().map(|url| url.value.as_str()),
            Some("https://example.com/watch")
        );
    }

    #[test]
    fn epg_channel_normalizes_singular_from_plural() {
        let mut channel = EpgChannel {
            id: "ch1".into(),
            icons: SmallVec::from_iter([EpgIcon {
                src: "https://example.com/icon.png".into(),
                width: None,
                height: None,
            }]),
            urls: SmallVec::from_iter([EpgUrl {
                value: "https://example.com/watch".into(),
                system: Some("mirror".into()),
            }]),
            ..Default::default()
        };

        channel.normalize_legacy_fields();

        assert_eq!(
            channel.icon.as_ref().map(|icon| icon.src.as_str()),
            Some("https://example.com/icon.png")
        );
        assert_eq!(
            channel.url.as_ref().map(|url| url.value.as_str()),
            Some("https://example.com/watch")
        );
    }

    #[test]
    fn set_primary_helpers_keep_channel_fields_in_sync() {
        let mut channel = EpgChannel::default();
        channel.set_primary_icon(EpgIcon {
            src: "https://example.com/icon.png".into(),
            width: None,
            height: None,
        });
        channel.set_primary_url(EpgUrl {
            value: "https://example.com/watch".into(),
            system: Some("official".into()),
        });

        assert_eq!(
            channel.icon.as_ref().map(|icon| icon.src.as_str()),
            Some("https://example.com/icon.png")
        );
        assert_eq!(channel.icons.len(), 1);
        assert_eq!(
            channel.url.as_ref().and_then(|url| url.system.as_deref()),
            Some("official")
        );
        assert_eq!(channel.urls.len(), 1);
    }

    #[test]
    fn epg_programme_roundtrips_with_xmltv_fidelity_fields() {
        let programme = EpgProgramme {
            channel: "ch1".into(),
            pdc_start: Some("20250101120000 +0000".into()),
            vps_start: Some("20250101120000 +0000".into()),
            showview: Some("12345".into()),
            videoplus: Some("67890".into()),
            clumpidx: Some("0/2".into()),
            title: SmallVec::from_iter([EpgStringWithLang::with_lang("Show", "en")]),
            language: SmallVec::from_iter([EpgStringWithLang::with_lang("English", "en")]),
            url: SmallVec::from_iter([EpgUrl {
                value: "https://example.com/show".into(),
                system: Some("official".into()),
            }]),
            country: SmallVec::from_iter([EpgStringWithLang::new("GB")]),
            length: Some(EpgLength {
                value: 60,
                units: EpgLengthUnit::Minutes,
            }),
            previously_shown: Some(EpgPreviouslyShown {
                start: Some("20240101120000 +0000".into()),
                channel: Some("old-channel".into()),
            }),
            subtitles: SmallVec::from_iter([EpgSubtitles {
                subtitle_type: Some(EpgSubtitleType::Teletext),
                language: Some(EpgStringWithLang::with_lang("English", "en")),
            }]),
            rating: SmallVec::from_iter([EpgRating {
                value: "PG-13".into(),
                system: Some("MPAA".into()),
                icons: SmallVec::from_iter([EpgIcon {
                    src: "https://example.com/rating.png".into(),
                    width: None,
                    height: None,
                }]),
            }]),
            ..Default::default()
        };

        let json = serde_json::to_string(&programme).unwrap();
        let reparsed: EpgProgramme = serde_json::from_str(&json).unwrap();

        assert_eq!(reparsed.length.unwrap().units, EpgLengthUnit::Minutes);
        assert_eq!(
            reparsed.subtitles[0].subtitle_type,
            Some(EpgSubtitleType::Teletext)
        );
        assert_eq!(
            reparsed.rating[0].icons[0].src,
            "https://example.com/rating.png"
        );
        assert_eq!(reparsed.url[0].system.as_deref(), Some("official"));
    }

    #[test]
    fn epg_credits_roundtrip_preserves_roles_and_person_media() {
        let credits = EpgCredits {
            director: SmallVec::from_iter([EpgPerson {
                name: "Director".into(),
                role: None,
                guest: false,
                images: SmallVec::from_iter([String::from("https://example.com/director.jpg")]),
                urls: SmallVec::from_iter([String::from("https://example.com/director")]),
            }]),
            adapter: SmallVec::from_iter([EpgPerson {
                name: "Adapter".into(),
                role: Some("translator".into()),
                guest: false,
                images: SmallVec::new(),
                urls: SmallVec::from_iter([String::from("https://example.com/adapter")]),
            }]),
            editor: SmallVec::from_iter([EpgPerson {
                name: "Editor".into(),
                role: None,
                guest: false,
                images: SmallVec::new(),
                urls: SmallVec::new(),
            }]),
            ..Default::default()
        };

        let json = serde_json::to_string(&credits).unwrap();
        let reparsed: EpgCredits = serde_json::from_str(&json).unwrap();

        assert_eq!(
            reparsed.director[0].primary_image(),
            Some("https://example.com/director.jpg")
        );
        assert_eq!(reparsed.adapter[0].role.as_deref(), Some("translator"));
        assert_eq!(reparsed.editor[0].name, "Editor");
    }
}
