use std::sync::LazyLock;

use serde::Deserialize;

/// The top-level shape shared by the embedded Quran data files.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct QuranDocument {
    pub quran: Quran,
}

/// All suras in Quran order.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Quran {
    #[serde(rename = "sura")]
    pub surahs: Vec<Surah>,
}

/// A sura and its ayat.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Surah {
    #[serde(rename = "@index")]
    pub index: String,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "aya")]
    pub ayahs: Vec<Ayah>,
}

/// One ayah in both Uthmani and Imlaey scripts.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Ayah {
    #[serde(rename = "@index")]
    pub index: String,
    #[serde(rename = "@uthmani")]
    pub uthmani: String,
    #[serde(rename = "@imlaey")]
    pub imlaey: String,
    #[serde(default)]
    pub rasm_map: Vec<RasmMapEntry>,
}

/// One word-level Uthmani-to-Imlaey mapping.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct RasmMapEntry {
    #[serde(rename = "@uthmani")]
    pub uthmani: String,
    #[serde(rename = "@imlaey")]
    pub imlaey: String,
}

static QURAN_TEXT: LazyLock<QuranDocument> = LazyLock::new(|| {
    serde_json::from_str(crate::QURAN_UTHMANI_IMLAEY_JSON)
        .expect("embedded Uthmani-Imlaey Quran JSON must remain valid")
});

static QURAN_MAPPING: LazyLock<QuranDocument> = LazyLock::new(|| {
    serde_json::from_str(crate::QURAN_UTHMANI_IMLAEY_MAP_JSON)
        .expect("embedded Uthmani-Imlaey mapping JSON must remain valid")
});

/// Returns the parsed Quran text containing Uthmani and Imlaey ayat.
///
/// The JSON is parsed only once and shared for the lifetime of the program.
#[must_use]
pub fn quran_text() -> &'static QuranDocument {
    &QURAN_TEXT
}

/// Returns the parsed Quran text and its word-level script mappings.
///
/// The JSON is parsed only once and shared for the lifetime of the program.
#[must_use]
pub fn quran_mapping() -> &'static QuranDocument {
    &QURAN_MAPPING
}

#[cfg(test)]
mod tests {
    use super::{quran_mapping, quran_text};

    #[test]
    fn quran_text_has_the_expected_structure() {
        let document = quran_text();

        assert_eq!(document.quran.surahs.len(), 114);

        let fatiha = &document.quran.surahs[0];
        assert_eq!(fatiha.index, "1");
        assert_eq!(fatiha.name, "الفاتحة");
        assert_eq!(fatiha.ayahs.len(), 7);

        let first_ayah = &fatiha.ayahs[0];
        assert_eq!(first_ayah.index, "1");
        assert!(!first_ayah.uthmani.is_empty());
        assert!(!first_ayah.imlaey.is_empty());
        assert!(first_ayah.rasm_map.is_empty());
    }

    #[test]
    fn quran_mapping_contains_word_level_pairs() {
        let document = quran_mapping();

        assert_eq!(document.quran.surahs.len(), 114);

        let first_ayah = &document.quran.surahs[0].ayahs[0];
        assert_eq!(first_ayah.rasm_map.len(), 4);
        assert_eq!(first_ayah.rasm_map[0].uthmani, "بِسْمِ");
        assert_eq!(first_ayah.rasm_map[0].imlaey, "بِسْمِ");

        let mapped_uthmani = first_ayah
            .rasm_map
            .iter()
            .map(|entry| entry.uthmani.as_str())
            .collect::<Vec<_>>()
            .join(" ");
        let mapped_imlaey = first_ayah
            .rasm_map
            .iter()
            .map(|entry| entry.imlaey.as_str())
            .collect::<Vec<_>>()
            .join(" ");

        assert_eq!(mapped_uthmani, first_ayah.uthmani);
        assert_eq!(mapped_imlaey, first_ayah.imlaey);
    }
}
