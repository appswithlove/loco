#![allow(dead_code)]
use serde::{Deserialize, Serialize};

// Auth
#[derive(Debug, Deserialize)]
pub struct AuthResponse {
    pub user: AuthUser,
    pub project: AuthProject,
}

#[derive(Debug, Deserialize)]
pub struct AuthUser {
    pub id: u64,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthProject {
    pub id: u64,
    pub name: String,
    pub url: String,
}

// Locales
#[derive(Debug, Deserialize, Serialize)]
pub struct Locale {
    pub code: String,
    pub name: String,
    #[serde(default)]
    pub plurals: Option<PluralRules>,
    #[serde(default)]
    pub progress: Option<LocaleProgress>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PluralRules {
    pub length: u32,
    pub equation: String,
    pub forms: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LocaleProgress {
    #[serde(rename = "num-translated")]
    pub num_translated: Option<u32>,
    #[serde(rename = "num-untranslated")]
    pub num_untranslated: Option<u32>,
    #[serde(rename = "num-approved")]
    pub num_approved: Option<u32>,
    #[serde(rename = "num-pending")]
    pub num_pending: Option<u32>,
    pub translated: Option<u32>,
    pub untranslated: Option<u32>,
    pub flagged: Option<u32>,
}

// Assets
#[derive(Debug, Deserialize, Serialize)]
pub struct Asset {
    pub id: String,
    #[serde(rename = "type")]
    pub asset_type: Option<String>,
    #[serde(default)]
    pub context: String,
    #[serde(default)]
    pub notes: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub progress: Option<AssetProgress>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AssetProgress {
    #[serde(rename = "num-translated")]
    pub num_translated: Option<u32>,
    #[serde(rename = "num-locales")]
    pub num_locales: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct CreateAssetRequest {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<String>,
}

// Translations
#[derive(Debug, Deserialize)]
pub struct Translation {
    pub id: String,
    #[serde(rename = "type")]
    pub translation_type: Option<String>,
    pub translated: bool,
    #[serde(deserialize_with = "deserialize_flagged")]
    pub flagged: Option<String>,
    pub status: Option<String>,
    pub translation: String,
    pub revision: Option<u32>,
    pub modified: Option<String>,
    /// Locale info — present in array responses as a nested object,
    /// absent in single-translation responses.
    #[serde(default, deserialize_with = "deserialize_locale")]
    pub locale: Option<TranslationLocale>,
}

#[derive(Debug, Deserialize)]
pub struct TranslationLocale {
    pub code: String,
    pub name: String,
}

fn deserialize_locale<'de, D>(deserializer: D) -> Result<Option<TranslationLocale>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de;

    struct LocaleVisitor;

    impl<'de> de::Visitor<'de> for LocaleVisitor {
        type Value = Option<TranslationLocale>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a locale object or null")
        }

        fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_map<A: de::MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
            let tl = TranslationLocale::deserialize(de::value::MapAccessDeserializer::new(map))?;
            Ok(Some(tl))
        }
    }

    deserializer.deserialize_any(LocaleVisitor)
}

/// The Loco API returns `false` when unflagged, or a string like `"fuzzy"` when flagged.
fn deserialize_flagged<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de;

    struct FlaggedVisitor;

    impl<'de> de::Visitor<'de> for FlaggedVisitor {
        type Value = Option<String>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string or boolean false")
        }

        fn visit_bool<E: de::Error>(self, v: bool) -> Result<Self::Value, E> {
            if v {
                Ok(Some("flagged".to_string()))
            } else {
                Ok(None)
            }
        }

        fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
            if v.is_empty() {
                Ok(None)
            } else {
                Ok(Some(v.to_string()))
            }
        }

        fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }
    }

    deserializer.deserialize_any(FlaggedVisitor)
}

// Import
#[derive(Debug, Deserialize)]
pub struct ImportResult {
    pub message: String,
    #[serde(default)]
    pub status: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct ImportProgress {
    pub progress: u32,
    pub success: Option<String>,
    pub error: Option<String>,
}

// Tags
#[derive(Debug, Deserialize, Serialize)]
pub struct Tag {
    pub name: String,
}

// Generic API error response
#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub status: u16,
    pub error: String,
}
