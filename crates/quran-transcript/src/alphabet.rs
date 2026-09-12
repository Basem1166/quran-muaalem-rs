use std::{
    collections::{BTreeMap, BTreeSet},
    sync::LazyLock,
};

use serde::Deserialize;

/// The Imlaey alphabet data used by the transcript engine.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct ImlaeyAlphabet {
    pub alphabet: String,
    pub hamazat: String,
    pub hamza: String,
    pub alef: String,
    pub alef_maksoora: String,
    pub taa_marboota: String,
    pub taa_mabsoota: String,
    pub haa: String,
    pub small_alef: String,
    pub tashkeel: String,
    pub skoon: String,
}

/// A special Uthmani-script pattern and its replacement options.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct SpecialPattern {
    pub pattern: String,
    pub attr_name: Option<String>,
    pub opts: Option<BTreeMap<String, String>>,
    pub target_pattern: Option<String>,
    pub pos: PatternPosition,
}

/// Where a special pattern may occur in its text.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PatternPosition {
    Start,
    Middle,
    End,
}

/// Structured Uthmani data that is not represented as a simple character constant.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct UthmaniStructuredData {
    pub hrof_moqtaa_disassemble: BTreeMap<String, String>,
    pub special_patterns: Vec<SpecialPattern>,
}

/// One exceptional Uthmani-to-Imlaey spelling pair.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct RasmPair {
    pub uthmani: String,
    pub imlaey: String,
}

/// Exceptional word mappings and the Imlaey prefixes that activate them.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct UniqueRasmMap {
    pub rasm_map: Vec<RasmPair>,
    pub imlaey_starts: Vec<String>,
}

/// The isti'aatha phrase in both supported scripts.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Istiaatha {
    pub imlaey: String,
    pub uthmani: String,
}

/// The sadaqa phrase in both supported scripts.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Sadaka {
    pub imlaey: String,
    pub uthmani: String,
}

/// Word sets used when deciding how an initial hamzat wasl is pronounced.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct BeginHamzatWasl {
    pub verbs_nouns_inter: BTreeSet<String>,
    pub verbs: BTreeSet<String>,
    pub damma_aarida_verbs: BTreeSet<String>,
    pub nouns: BTreeSet<String>,
}

/// Parsed data from the embedded Quran alphabet JSON file.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct QuranAlphabet {
    pub imlaey: ImlaeyAlphabet,
    pub uthmani: UthmaniStructuredData,
    pub unique_rasm_map: UniqueRasmMap,
    pub istiaatha: Istiaatha,
    pub sadaka: Sadaka,
}

static QURAN_ALPHABET: LazyLock<QuranAlphabet> = LazyLock::new(|| {
    serde_json::from_str(crate::QURAN_ALPHABET_JSON)
        .expect("embedded Quran alphabet JSON must remain valid")
});

static BEGIN_HAMZAT_WASL: LazyLock<BeginHamzatWasl> = LazyLock::new(|| {
    serde_json::from_str(crate::BEGIN_WITH_HAMZAT_WASL_JSON)
        .expect("embedded hamzat-wasl JSON must remain valid")
});

/// Returns the parsed embedded Quran alphabet data.
///
/// The JSON is parsed only once and shared for the lifetime of the program.
#[must_use]
pub fn quran_alphabet() -> &'static QuranAlphabet {
    &QURAN_ALPHABET
}

/// Returns the parsed embedded hamzat-wasl word sets.
///
/// The JSON is parsed only once and shared for the lifetime of the program.
#[must_use]
pub fn begin_hamzat_wasl() -> &'static BeginHamzatWasl {
    &BEGIN_HAMZAT_WASL
}

/// Imlaey-script characters and character groups used by the Quran transcript engine.
pub mod imlaey {
    pub const ALPHABET: &str = "ءآأؤإئابةتثجحخدذرزسشصضطظعغفقكلمنهوىيًٌٍَُِّْٰ";
    pub const HAMAZAT: &str = "ءآأؤإئ";
    pub const HAMZA: char = 'ء';
    pub const ALEF: char = 'ا';
    pub const ALEF_MAKSOORA: char = 'ى';
    pub const TAA_MARBOOTA: char = 'ة';
    pub const TAA_MABSOOTA: char = 'ت';
    pub const HAA: char = 'ه';
    pub const SMALL_ALEF: char = 'ٰ';
    pub const TASHKEEL: &str = "ًٌٍَُِّْ";
    pub const SKOON: char = 'ْ';
}

/// Uthmani-script characters used by the Quran transcript engine.
pub mod uthmani {
    /// A normal space character.
    pub const SPACE: char = ' ';

    /// Arabic letter alif: ا
    pub const ALIF: char = 'ا';

    /// Arabic letter alif maksora: ى
    pub const ALIF_MAKSORA: char = 'ى';

    /// Basic Uthmani Arabic letters.
    pub const BAA: char = 'ب';
    pub const TAA_MABSOOTA: char = 'ت';
    pub const TAA_MARBOOTA: char = 'ة';
    pub const THAA: char = 'ث';
    pub const JEEM: char = 'ج';
    pub const HAA_MOHMALA: char = 'ح';
    pub const KHAA: char = 'خ';
    pub const DAAL: char = 'د';
    pub const THAAL: char = 'ذ';
    pub const RAA: char = 'ر';
    pub const ZAY: char = 'ز';
    pub const SEEN: char = 'س';
    pub const SHEEN: char = 'ش';
    pub const SAAD: char = 'ص';
    pub const DAAD: char = 'ض';
    pub const TAA_MOFAKHAMA: char = 'ط';
    pub const ZAA_MOFAKHAMA: char = 'ظ';
    pub const AYN: char = 'ع';
    pub const GHYN: char = 'غ';
    pub const FAA: char = 'ف';
    pub const QAF: char = 'ق';
    pub const KAF: char = 'ك';
    pub const LAM: char = 'ل';
    pub const MEEM: char = 'م';
    pub const NOON: char = 'ن';
    pub const HAA: char = 'ه';
    pub const WAW: char = 'و';
    pub const YAA: char = 'ي';

    /// Hamza forms used in Uthmani script.
    pub const HAMZA: char = 'ء';
    pub const HAMZA_ABOVE_ALIF: char = 'أ';
    pub const HAMZA_BELOW_ALIF: char = 'إ';
    pub const HAMZA_ABOVE_WAW: char = 'ؤ';
    pub const HAMZA_ABOVE_YAA: char = 'ئ';
    pub const HAMZA_MAMDODA: char = 'ٔ';

    /// Arabic vowel marks and tanween marks.
    pub const TANWEEN_FATH: char = 'ً';
    pub const TANWEEN_DAM: char = 'ٌ';
    pub const TANWEEN_KASR: char = 'ٍ';
    pub const FATHA: char = 'َ';
    pub const DAMA: char = 'ُ';
    pub const KASRA: char = 'ِ';

    /// Arabic shadda diacritic: ّ
    pub const SHADDA: char = 'ّ';

    /// Other Uthmani diacritics and Quranic marks.
    pub const RAS_HAAA: char = 'ْ';
    pub const MADD: char = 'ٓ';
    pub const HAMZAT_WASL: char = 'ٱ';
    pub const ALIF_KHNJARIA: char = 'ٰ';
    pub const SKOON_MOSTADEER: char = '۟';
    pub const SKOON_MOSTATEEL: char = '۠';
    pub const MEEM_IQLAB: char = 'ۢ';
    pub const IMALA_SIGN: char = '۪';
    pub const ISHMAM_SIGN: char = '۫';
    pub const TASHEEL_SIGN: char = '۬';
    pub const TANWEEN_IDHAAM_DTERMINER: char = 'ۭ';
    pub const KASHEEDA: char = 'ـ';

    /// Small Uthmani letters and marks.
    pub const SMALL_SEEN_ABOVE: char = 'ۜ';
    pub const SMALL_SEEN_BELOW: char = 'ۣ';
    pub const SMALL_WAW: char = 'ۥ';
    pub const SMALL_YAA_SILA: char = 'ۦ';
    pub const SMALL_YAA: char = 'ۧ';
    pub const SMALL_NOON: char = 'ۨ';

    /// Derived letter and diacritic groups from the original Python alphabet.
    pub const MADD_ALIF: &str = "َا";
    pub const MADD_WAW: &str = "ُو";
    pub const MADD_YAA: &str = "ِي";
    pub const NOON_IKHFAA_GROUP: &str = "صذثكجشقسدطزفتضظ";
    pub const NOON_IDGHAM_GROUP: &str = "يرملون";
    pub const HARAKAT_GROUP: &str = "َُِ";
    pub const HAMAZAT_GROUP: &str = "ءأإؤئٔ";
    pub const LETTERS_GROUP: &str = "اىبتةثجحخدذرزسشصضطظعغفقكلمنهويء";
    pub const PURE_LETTERS_GROUP: &str = "بتثجحخدذرزسشصضطظعغفقكلمنهويء";
    pub const PURE_LETTERS_WITHOUT_YAA_AND_WAW_GROUP: &str = "بتثجحخدذرزسشصضطظعغفقكلمنهء";
    pub const QLQLA_GROUP: &str = "قطبجد";

    /// Tanween variants used by tajweed rules.
    pub const TANWEEN_FATH_MOTHHAR: &str = "ً";
    pub const TANWEEN_DAM_MOTHHAR: &str = "ٌ";
    pub const TANWEEN_KASR_MOTHHAR: &str = "ٍ";
    pub const TANWEEN_FATH_MODGHAM: &str = "ًۭ";
    pub const TANWEEN_DAM_MODGHAM: &str = "ٌۭ";
    pub const TANWEEN_KASR_MODGHAM: &str = "ٍۭ";
    pub const TANWEEN_FATH_IQLAB: &str = "ًۢ";
    pub const TANWEEN_DAM_IQLAB: &str = "ٌۢ";
    pub const TANWEEN_KASR_IQLAB: &str = "ٍۢ";
}

/// Characters used by the Quran phonetic script.
pub mod phonetic {
    pub const HAMZA: char = super::uthmani::HAMZA;
    pub const BAA: char = super::uthmani::BAA;
    pub const TAA: char = super::uthmani::TAA_MABSOOTA;
    pub const THAA: char = super::uthmani::THAA;
    pub const JEEM: char = super::uthmani::JEEM;
    pub const HAA_MOHMALA: char = super::uthmani::HAA_MOHMALA;
    pub const KHAA: char = super::uthmani::KHAA;
    pub const DAAL: char = super::uthmani::DAAL;
    pub const THAAL: char = super::uthmani::THAAL;
    pub const RAA: char = super::uthmani::RAA;
    pub const ZAY: char = super::uthmani::ZAY;
    pub const SEEN: char = super::uthmani::SEEN;
    pub const SHEEN: char = super::uthmani::SHEEN;
    pub const SAAD: char = super::uthmani::SAAD;
    pub const DAAD: char = super::uthmani::DAAD;
    pub const TAA_MOFAKHAMA: char = super::uthmani::TAA_MOFAKHAMA;
    pub const ZAA_MOFAKHAMA: char = super::uthmani::ZAA_MOFAKHAMA;
    pub const AYN: char = super::uthmani::AYN;
    pub const GHYN: char = super::uthmani::GHYN;
    pub const FAA: char = super::uthmani::FAA;
    pub const QAF: char = super::uthmani::QAF;
    pub const KAF: char = super::uthmani::KAF;
    pub const LAM: char = super::uthmani::LAM;
    pub const MEEM: char = super::uthmani::MEEM;
    pub const NOON: char = super::uthmani::NOON;
    pub const HAA: char = super::uthmani::HAA;
    pub const WAW: char = super::uthmani::WAW;
    pub const YAA: char = super::uthmani::YAA;

    /// Long-vowel characters.
    pub const ALIF: char = super::uthmani::ALIF;
    pub const YAA_MADD: char = super::uthmani::SMALL_YAA_SILA;
    pub const WAW_MADD: char = super::uthmani::SMALL_WAW;

    /// Short-vowel characters.
    pub const FATHA: char = super::uthmani::FATHA;
    pub const DAMA: char = super::uthmani::DAMA;
    pub const KASRA: char = super::uthmani::KASRA;

    /// Special phonetic-script characters.
    pub const FATHA_MOMALA: char = super::uthmani::IMALA_SIGN;
    pub const ALIF_MOMALA: char = super::uthmani::KASHEEDA;
    pub const HAMZA_MOSAHALA: char = '\u{0672}';
    pub const QLQLA: char = '\u{0687}';
    pub const NOON_MOKHFAH: char = '\u{06ba}';
    pub const MEEM_MOKHFAH: char = '\u{06fe}';
    pub const SAKT: char = super::uthmani::SMALL_SEEN_ABOVE;
    pub const DAMA_MOKHTALASA: char = '\u{0619}';
}

/// Phonetic character groups used to classify pronunciation properties.
pub mod phonetic_groups {
    pub const CORE: &str = "ءبتثجحخدذرزسشصضطظعغفقكلمنهوياۥۦ\u{06fe}\u{06ba}ـ\u{0672}";
    pub const RESIDUALS: &str = "َُِ\u{0687}۪ۜ\u{0619}";
    pub const HARAKAT: &str = "َُِ";
    pub const HAMS: &str = "فحثهشخصسكت";
    pub const SHIDDA: &str = "ءجدقطبكت";
    pub const BETWEEN_SHIDDA_RAKHAWA: &str = "لنعمر";
    pub const TAFKHEEM: &str = "خصضغطقظ";
    pub const ITBAAQ: &str = "صضطظ";
    pub const SAFEER: &str = "صزس";
    pub const QALQAL: &str = "قطبجد";
    pub const TIKRAR: &str = "ر";
    pub const TAFASHIE: &str = "ش";
    pub const ISTITALA: &str = "ض";
    pub const GHONNA: &str = "نم\u{06ba}\u{06fe}";
}

#[cfg(test)]
mod tests {
    use super::{
        PatternPosition, begin_hamzat_wasl, imlaey, phonetic, phonetic_groups, quran_alphabet,
        uthmani,
    };
    use crate::QURAN_ALPHABET_JSON;

    fn source_text<'a>(alphabet: &'a serde_json::Value, section: &str, name: &str) -> &'a str {
        alphabet[section][name]
            .as_str()
            .unwrap_or_else(|| panic!("missing {section} alphabet value: {name}"))
    }

    fn source_character(alphabet: &serde_json::Value, section: &str, name: &str) -> char {
        let value = source_text(alphabet, section, name);

        let mut characters = value.chars();
        let character = characters
            .next()
            .unwrap_or_else(|| panic!("Uthmani alphabet value is empty: {name}"));

        assert!(
            characters.next().is_none(),
            "Uthmani alphabet value must be one character: {name}"
        );

        character
    }

    fn join_characters(characters: &[char]) -> String {
        characters.iter().copied().collect()
    }

    #[test]
    fn uthmani_constants_match_the_source_data() {
        let alphabet: serde_json::Value =
            serde_json::from_str(QURAN_ALPHABET_JSON).expect("alphabet JSON must be valid");

        for (name, constant) in [
            ("space", uthmani::SPACE),
            ("alif", uthmani::ALIF),
            ("alif_maksora", uthmani::ALIF_MAKSORA),
            ("baa", uthmani::BAA),
            ("taa_mabsoota", uthmani::TAA_MABSOOTA),
            ("taa_marboota", uthmani::TAA_MARBOOTA),
            ("thaa", uthmani::THAA),
            ("jeem", uthmani::JEEM),
            ("haa_mohmala", uthmani::HAA_MOHMALA),
            ("khaa", uthmani::KHAA),
            ("daal", uthmani::DAAL),
            ("thaal", uthmani::THAAL),
            ("raa", uthmani::RAA),
            ("zay", uthmani::ZAY),
            ("seen", uthmani::SEEN),
            ("sheen", uthmani::SHEEN),
            ("saad", uthmani::SAAD),
            ("daad", uthmani::DAAD),
            ("taa_mofakhama", uthmani::TAA_MOFAKHAMA),
            ("zaa_mofakhama", uthmani::ZAA_MOFAKHAMA),
            ("ayn", uthmani::AYN),
            ("ghyn", uthmani::GHYN),
            ("faa", uthmani::FAA),
            ("qaf", uthmani::QAF),
            ("kaf", uthmani::KAF),
            ("lam", uthmani::LAM),
            ("meem", uthmani::MEEM),
            ("noon", uthmani::NOON),
            ("haa", uthmani::HAA),
            ("waw", uthmani::WAW),
            ("yaa", uthmani::YAA),
            ("hamza", uthmani::HAMZA),
            ("hamza_above_alif", uthmani::HAMZA_ABOVE_ALIF),
            ("hamza_below_alif", uthmani::HAMZA_BELOW_ALIF),
            ("hamza_above_waw", uthmani::HAMZA_ABOVE_WAW),
            ("hamza_above_yaa", uthmani::HAMZA_ABOVE_YAA),
            ("hamza_mamdoda", uthmani::HAMZA_MAMDODA),
            ("tanween_fath", uthmani::TANWEEN_FATH),
            ("tanween_dam", uthmani::TANWEEN_DAM),
            ("tanween_kasr", uthmani::TANWEEN_KASR),
            ("fatha", uthmani::FATHA),
            ("dama", uthmani::DAMA),
            ("kasra", uthmani::KASRA),
            ("shadda", uthmani::SHADDA),
            ("ras_haaa", uthmani::RAS_HAAA),
            ("madd", uthmani::MADD),
            ("hamzat_wasl", uthmani::HAMZAT_WASL),
            ("alif_khnjaria", uthmani::ALIF_KHNJARIA),
            ("small_seen_above", uthmani::SMALL_SEEN_ABOVE),
            ("small_seen_below", uthmani::SMALL_SEEN_BELOW),
            ("small_waw", uthmani::SMALL_WAW),
            ("small_yaa_sila", uthmani::SMALL_YAA_SILA),
            ("small_yaa", uthmani::SMALL_YAA),
            ("small_noon", uthmani::SMALL_NOON),
            ("skoon_mostadeer", uthmani::SKOON_MOSTADEER),
            ("skoon_mostateel", uthmani::SKOON_MOSTATEEL),
            ("meem_iqlab", uthmani::MEEM_IQLAB),
            ("imala_sign", uthmani::IMALA_SIGN),
            ("ishmam_sign", uthmani::ISHMAM_SIGN),
            ("tasheel_sign", uthmani::TASHEEL_SIGN),
            (
                "tanween_idhaam_dterminer",
                uthmani::TANWEEN_IDHAAM_DTERMINER,
            ),
            ("kasheeda", uthmani::KASHEEDA),
        ] {
            assert_eq!(constant, source_character(&alphabet, "uthmani", name));
        }

        for (name, group) in [
            ("madd_alif", uthmani::MADD_ALIF),
            ("madd_waw", uthmani::MADD_WAW),
            ("madd_yaa", uthmani::MADD_YAA),
            ("noon_ikhfaa_group", uthmani::NOON_IKHFAA_GROUP),
            ("noon_idgham_group", uthmani::NOON_IDGHAM_GROUP),
            ("harakat_group", uthmani::HARAKAT_GROUP),
            ("hamazat_group", uthmani::HAMAZAT_GROUP),
            ("letters_group", uthmani::LETTERS_GROUP),
            ("pure_letters_group", uthmani::PURE_LETTERS_GROUP),
            (
                "pure_letters_without_yaa_and_waw_group",
                uthmani::PURE_LETTERS_WITHOUT_YAA_AND_WAW_GROUP,
            ),
            ("qlqla_group", uthmani::QLQLA_GROUP),
            ("tanween_fath_mothhar", uthmani::TANWEEN_FATH_MOTHHAR),
            ("tanween_dam_mothhar", uthmani::TANWEEN_DAM_MOTHHAR),
            ("tanween_kasr_mothhar", uthmani::TANWEEN_KASR_MOTHHAR),
            ("tanween_fath_modgham", uthmani::TANWEEN_FATH_MODGHAM),
            ("tanween_dam_modgham", uthmani::TANWEEN_DAM_MODGHAM),
            ("tanween_kasr_modgham", uthmani::TANWEEN_KASR_MODGHAM),
            ("tanween_fath_iqlab", uthmani::TANWEEN_FATH_IQLAB),
            ("tanween_dam_iqlab", uthmani::TANWEEN_DAM_IQLAB),
            ("tanween_kasr_iqlab", uthmani::TANWEEN_KASR_IQLAB),
        ] {
            assert_eq!(group, source_text(&alphabet, "uthmani", name));
        }
    }

    #[test]
    fn imlaey_constants_match_the_source_data() {
        let alphabet: serde_json::Value =
            serde_json::from_str(QURAN_ALPHABET_JSON).expect("alphabet JSON must be valid");

        for (name, constant) in [
            ("alphabet", imlaey::ALPHABET),
            ("hamazat", imlaey::HAMAZAT),
            ("tashkeel", imlaey::TASHKEEL),
        ] {
            assert_eq!(constant, source_text(&alphabet, "imlaey", name));
        }

        for (name, constant) in [
            ("hamza", imlaey::HAMZA),
            ("alef", imlaey::ALEF),
            ("alef_maksoora", imlaey::ALEF_MAKSOORA),
            ("taa_marboota", imlaey::TAA_MARBOOTA),
            ("taa_mabsoota", imlaey::TAA_MABSOOTA),
            ("haa", imlaey::HAA),
            ("small_alef", imlaey::SMALL_ALEF),
            ("skoon", imlaey::SKOON),
        ] {
            assert_eq!(constant, source_character(&alphabet, "imlaey", name));
        }
    }

    #[test]
    fn structured_alphabet_data_parses_from_the_embedded_json() {
        let alphabet = quran_alphabet();

        assert_eq!(alphabet.imlaey.alphabet, imlaey::ALPHABET);
        assert!(!alphabet.uthmani.hrof_moqtaa_disassemble.is_empty());
        assert!(!alphabet.uthmani.special_patterns.is_empty());

        let first_pattern = &alphabet.uthmani.special_patterns[0];
        assert_eq!(first_pattern.pos, PatternPosition::Start);
        assert!(first_pattern.target_pattern.is_some());

        assert_eq!(alphabet.unique_rasm_map.rasm_map.len(), 2);
        assert_eq!(alphabet.unique_rasm_map.imlaey_starts.len(), 3);
        assert!(!alphabet.istiaatha.imlaey.is_empty());
        assert!(!alphabet.istiaatha.uthmani.is_empty());
        assert!(!alphabet.sadaka.imlaey.is_empty());
        assert!(!alphabet.sadaka.uthmani.is_empty());
    }

    #[test]
    fn hamzat_wasl_word_sets_parse_from_the_embedded_json() {
        let words = begin_hamzat_wasl();

        assert_eq!(words.verbs_nouns_inter.len(), 1);
        assert_eq!(words.verbs.len(), 335);
        assert_eq!(words.damma_aarida_verbs.len(), 5);
        assert_eq!(words.nouns.len(), 52);
        assert!(words.verbs.contains("ٱهْدِنَا"));
        assert!(words.nouns.contains("ٱسْمُ"));
    }

    #[test]
    fn phonetic_constants_match_the_python_mappings() {
        for (constant, source) in [
            (phonetic::HAMZA, uthmani::HAMZA),
            (phonetic::BAA, uthmani::BAA),
            (phonetic::TAA, uthmani::TAA_MABSOOTA),
            (phonetic::THAA, uthmani::THAA),
            (phonetic::JEEM, uthmani::JEEM),
            (phonetic::HAA_MOHMALA, uthmani::HAA_MOHMALA),
            (phonetic::KHAA, uthmani::KHAA),
            (phonetic::DAAL, uthmani::DAAL),
            (phonetic::THAAL, uthmani::THAAL),
            (phonetic::RAA, uthmani::RAA),
            (phonetic::ZAY, uthmani::ZAY),
            (phonetic::SEEN, uthmani::SEEN),
            (phonetic::SHEEN, uthmani::SHEEN),
            (phonetic::SAAD, uthmani::SAAD),
            (phonetic::DAAD, uthmani::DAAD),
            (phonetic::TAA_MOFAKHAMA, uthmani::TAA_MOFAKHAMA),
            (phonetic::ZAA_MOFAKHAMA, uthmani::ZAA_MOFAKHAMA),
            (phonetic::AYN, uthmani::AYN),
            (phonetic::GHYN, uthmani::GHYN),
            (phonetic::FAA, uthmani::FAA),
            (phonetic::QAF, uthmani::QAF),
            (phonetic::KAF, uthmani::KAF),
            (phonetic::LAM, uthmani::LAM),
            (phonetic::MEEM, uthmani::MEEM),
            (phonetic::NOON, uthmani::NOON),
            (phonetic::HAA, uthmani::HAA),
            (phonetic::WAW, uthmani::WAW),
            (phonetic::YAA, uthmani::YAA),
            (phonetic::ALIF, uthmani::ALIF),
            (phonetic::YAA_MADD, uthmani::SMALL_YAA_SILA),
            (phonetic::WAW_MADD, uthmani::SMALL_WAW),
            (phonetic::FATHA, uthmani::FATHA),
            (phonetic::DAMA, uthmani::DAMA),
            (phonetic::KASRA, uthmani::KASRA),
            (phonetic::FATHA_MOMALA, uthmani::IMALA_SIGN),
            (phonetic::ALIF_MOMALA, uthmani::KASHEEDA),
            (phonetic::SAKT, uthmani::SMALL_SEEN_ABOVE),
        ] {
            assert_eq!(constant, source);
        }

        assert_eq!(phonetic::HAMZA_MOSAHALA, '\u{0672}');
        assert_eq!(phonetic::QLQLA, '\u{0687}');
        assert_eq!(phonetic::NOON_MOKHFAH, '\u{06ba}');
        assert_eq!(phonetic::MEEM_MOKHFAH, '\u{06fe}');
        assert_eq!(phonetic::DAMA_MOKHTALASA, '\u{0619}');
    }

    #[test]
    #[allow(clippy::too_many_lines)] // Mirrors Python's complete group construction order.
    fn phonetic_groups_match_the_python_construction_order() {
        assert_eq!(
            phonetic_groups::CORE,
            join_characters(&[
                phonetic::HAMZA,
                phonetic::BAA,
                phonetic::TAA,
                phonetic::THAA,
                phonetic::JEEM,
                phonetic::HAA_MOHMALA,
                phonetic::KHAA,
                phonetic::DAAL,
                phonetic::THAAL,
                phonetic::RAA,
                phonetic::ZAY,
                phonetic::SEEN,
                phonetic::SHEEN,
                phonetic::SAAD,
                phonetic::DAAD,
                phonetic::TAA_MOFAKHAMA,
                phonetic::ZAA_MOFAKHAMA,
                phonetic::AYN,
                phonetic::GHYN,
                phonetic::FAA,
                phonetic::QAF,
                phonetic::KAF,
                phonetic::LAM,
                phonetic::MEEM,
                phonetic::NOON,
                phonetic::HAA,
                phonetic::WAW,
                phonetic::YAA,
                phonetic::ALIF,
                phonetic::WAW_MADD,
                phonetic::YAA_MADD,
                phonetic::MEEM_MOKHFAH,
                phonetic::NOON_MOKHFAH,
                phonetic::ALIF_MOMALA,
                phonetic::HAMZA_MOSAHALA,
            ])
        );
        assert_eq!(
            phonetic_groups::RESIDUALS,
            join_characters(&[
                phonetic::FATHA,
                phonetic::DAMA,
                phonetic::KASRA,
                phonetic::QLQLA,
                phonetic::FATHA_MOMALA,
                phonetic::SAKT,
                phonetic::DAMA_MOKHTALASA,
            ])
        );
        assert_eq!(
            phonetic_groups::HARAKAT,
            join_characters(&[phonetic::FATHA, phonetic::DAMA, phonetic::KASRA])
        );
        assert_eq!(
            phonetic_groups::HAMS,
            join_characters(&[
                phonetic::FAA,
                phonetic::HAA_MOHMALA,
                phonetic::THAA,
                phonetic::HAA,
                phonetic::SHEEN,
                phonetic::KHAA,
                phonetic::SAAD,
                phonetic::SEEN,
                phonetic::KAF,
                phonetic::TAA,
            ])
        );
        assert_eq!(
            phonetic_groups::SHIDDA,
            join_characters(&[
                phonetic::HAMZA,
                phonetic::JEEM,
                phonetic::DAAL,
                phonetic::QAF,
                phonetic::TAA_MOFAKHAMA,
                phonetic::BAA,
                phonetic::KAF,
                phonetic::TAA,
            ])
        );
        assert_eq!(
            phonetic_groups::BETWEEN_SHIDDA_RAKHAWA,
            join_characters(&[
                phonetic::LAM,
                phonetic::NOON,
                phonetic::AYN,
                phonetic::MEEM,
                phonetic::RAA,
            ])
        );
        assert_eq!(
            phonetic_groups::TAFKHEEM,
            join_characters(&[
                phonetic::KHAA,
                phonetic::SAAD,
                phonetic::DAAD,
                phonetic::GHYN,
                phonetic::TAA_MOFAKHAMA,
                phonetic::QAF,
                phonetic::ZAA_MOFAKHAMA,
            ])
        );
        assert_eq!(
            phonetic_groups::ITBAAQ,
            join_characters(&[
                phonetic::SAAD,
                phonetic::DAAD,
                phonetic::TAA_MOFAKHAMA,
                phonetic::ZAA_MOFAKHAMA,
            ])
        );
        assert_eq!(
            phonetic_groups::SAFEER,
            join_characters(&[phonetic::SAAD, phonetic::ZAY, phonetic::SEEN])
        );
        assert_eq!(
            phonetic_groups::QALQAL,
            join_characters(&[
                phonetic::QAF,
                phonetic::TAA_MOFAKHAMA,
                phonetic::BAA,
                phonetic::JEEM,
                phonetic::DAAL,
            ])
        );
        assert_eq!(phonetic_groups::TIKRAR, phonetic::RAA.to_string());
        assert_eq!(phonetic_groups::TAFASHIE, phonetic::SHEEN.to_string());
        assert_eq!(phonetic_groups::ISTITALA, phonetic::DAAD.to_string());
        assert_eq!(
            phonetic_groups::GHONNA,
            join_characters(&[
                phonetic::NOON,
                phonetic::MEEM,
                phonetic::NOON_MOKHFAH,
                phonetic::MEEM_MOKHFAH,
            ])
        );
    }
}
