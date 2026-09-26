use std::{
    collections::{HashMap, HashSet},
    sync::LazyLock,
};

use serde::Deserialize;

/// The Imlaey alphabet data used by the transcript engine.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
pub struct SpecialPattern {
    pub pattern: String,
    pub attr_name: Option<String>,
    pub opts: Option<HashMap<String, String>>,
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

/// The Uthmani alphabet exactly as stored in `quran-alphabet.json`.
///
/// Mirrors the Python `UthmaniAlphabet` dataclass: single characters first,
/// then the disassembled `hrof` map, `special_patterns`, and finally the
/// derived tanween and group strings also stored in the JSON.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct UthmaniAlphabet {
    pub alif: String,
    pub alif_maksora: String,
    pub baa: String,
    pub taa_mabsoota: String,
    pub taa_marboota: String,
    pub thaa: String,
    pub jeem: String,
    pub haa_mohmala: String,
    pub khaa: String,
    pub daal: String,
    pub thaal: String,
    pub raa: String,
    pub zay: String,
    pub seen: String,
    pub sheen: String,
    pub saad: String,
    pub daad: String,
    pub taa_mofakhama: String,
    pub zaa_mofakhama: String,
    pub ayn: String,
    pub ghyn: String,
    pub faa: String,
    pub qaf: String,
    pub kaf: String,
    pub lam: String,
    pub meem: String,
    pub noon: String,
    pub haa: String,
    pub waw: String,
    pub yaa: String,
    pub hamza: String,
    pub hamza_above_alif: String,
    pub hamza_below_alif: String,
    pub hamza_above_waw: String,
    pub hamza_above_yaa: String,
    pub hamza_mamdoda: String,
    pub tanween_fath: String,
    pub tanween_dam: String,
    pub tanween_kasr: String,
    pub fatha: String,
    pub dama: String,
    pub kasra: String,
    pub shadda: String,
    pub ras_haaa: String,
    pub madd: String,
    pub hamzat_wasl: String,
    pub alif_khnjaria: String,
    pub small_seen_above: String,
    pub small_seen_below: String,
    pub small_waw: String,
    pub small_yaa_sila: String,
    pub small_yaa: String,
    pub small_noon: String,
    pub skoon_mostadeer: String,
    pub skoon_mostateel: String,
    pub meem_iqlab: String,
    pub imala_sign: String,
    pub ishmam_sign: String,
    pub tasheel_sign: String,
    pub tanween_idhaam_dterminer: String,
    pub kasheeda: String,
    pub space: String,
    pub hrof_moqtaa_disassemble: HashMap<String, String>,
    pub special_patterns: Vec<SpecialPattern>,
    pub tanween_fath_mothhar: String,
    pub tanween_dam_mothhar: String,
    pub tanween_kasr_mothhar: String,
    pub tanween_fath_modgham: String,
    pub tanween_dam_modgham: String,
    pub tanween_kasr_modgham: String,
    pub tanween_fath_iqlab: String,
    pub tanween_dam_iqlab: String,
    pub tanween_kasr_iqlab: String,
    pub madd_alif: String,
    pub madd_waw: String,
    pub madd_yaa: String,
    pub noon_ikhfaa_group: String,
    pub noon_idgham_group: String,
    pub harakat_group: String,
    pub hamazat_group: String,
    pub letters_group: String,
    pub pure_letters_group: String,
    pub pure_letters_without_yaa_and_waw_group: String,
    pub qlqla_group: String,
}

/// One exceptional Uthmani-to-Imlaey spelling pair.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RasmPair {
    pub uthmani: String,
    pub imlaey: String,
}

/// Exceptional word mappings and the Imlaey prefixes that activate them.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct UniqueRasmMap {
    pub rasm_map: Vec<RasmPair>,
    pub imlaey_starts: Vec<String>,
}

/// The isti'aatha phrase in both supported scripts.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Istiaatha {
    pub imlaey: String,
    pub uthmani: String,
}

/// The sadaqa phrase in both supported scripts.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Sadaka {
    pub imlaey: String,
    pub uthmani: String,
}

/// Word sets used when deciding how an initial hamzat wasl is pronounced.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BeginHamzatWasl {
    pub verbs_nouns_inter: HashSet<String>,
    pub verbs: HashSet<String>,
    pub damma_aarida_verbs: HashSet<String>,
    pub nouns: HashSet<String>,
}

/// Parsed data from the embedded Quran alphabet JSON file.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct QuranAlphabet {
    pub imlaey: ImlaeyAlphabet,
    pub uthmani: UthmaniAlphabet,
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
    pub const HAMZA: &str = "ء";
    pub const ALEF: &str = "ا";
    pub const ALEF_MAKSOORA: &str = "ى";
    pub const TAA_MARBOOTA: &str = "ة";
    pub const TAA_MABSOOTA: &str = "ت";
    pub const HAA: &str = "ه";
    pub const SMALL_ALEF: &str = "ٰ";
    pub const TASHKEEL: &str = "ًٌٍَُِّْ";
    pub const SKOON: &str = "ْ";
}

/// Uthmani-script characters used by the Quran transcript engine.
///
/// Single characters are `&str` constants so derived groups can reuse them
/// with `constcat::concat!`, mirroring the Python `UthmaniAlphabet`
/// `__post_init__` concatenations (`self.fatha + self.alif`, ...).
pub mod uthmani {
    /// A normal space character.
    pub const SPACE: &str = " ";
    /// Arabic letter alif: ا
    pub const ALIF: &str = "ا";
    /// Arabic letter alif maksora: ى
    pub const ALIF_MAKSORA: &str = "ى";
    /// Basic Uthmani Arabic letters.
    pub const BAA: &str = "ب";
    pub const TAA_MABSOOTA: &str = "ت";
    pub const TAA_MARBOOTA: &str = "ة";
    pub const THAA: &str = "ث";
    pub const JEEM: &str = "ج";
    pub const HAA_MOHMALA: &str = "ح";
    pub const KHAA: &str = "خ";
    pub const DAAL: &str = "د";
    pub const THAAL: &str = "ذ";
    pub const RAA: &str = "ر";
    pub const ZAY: &str = "ز";
    pub const SEEN: &str = "س";
    pub const SHEEN: &str = "ش";
    pub const SAAD: &str = "ص";
    pub const DAAD: &str = "ض";
    pub const TAA_MOFAKHAMA: &str = "ط";
    pub const ZAA_MOFAKHAMA: &str = "ظ";
    pub const AYN: &str = "ع";
    pub const GHYN: &str = "غ";
    pub const FAA: &str = "ف";
    pub const QAF: &str = "ق";
    pub const KAF: &str = "ك";
    pub const LAM: &str = "ل";
    pub const MEEM: &str = "م";
    pub const NOON: &str = "ن";
    pub const HAA: &str = "ه";
    pub const WAW: &str = "و";
    pub const YAA: &str = "ي";
    /// Hamza forms used in Uthmani script.
    pub const HAMZA: &str = "ء";
    pub const HAMZA_ABOVE_ALIF: &str = "أ";
    pub const HAMZA_BELOW_ALIF: &str = "إ";
    pub const HAMZA_ABOVE_WAW: &str = "ؤ";
    pub const HAMZA_ABOVE_YAA: &str = "ئ";
    pub const HAMZA_MAMDODA: &str = "ٔ";
    /// Arabic vowel marks and tanween marks.
    pub const TANWEEN_FATH: &str = "ً";
    pub const TANWEEN_DAM: &str = "ٌ";
    pub const TANWEEN_KASR: &str = "ٍ";
    pub const FATHA: &str = "َ";
    pub const DAMA: &str = "ُ";
    pub const KASRA: &str = "ِ";
    /// Arabic shadda diacritic: ّ
    pub const SHADDA: &str = "ّ";
    /// Other Uthmani diacritics and Quranic marks.
    pub const RAS_HAAA: &str = "ْ";
    pub const MADD: &str = "ٓ";
    pub const HAMZAT_WASL: &str = "ٱ";
    pub const ALIF_KHNJARIA: &str = "ٰ";
    pub const SKOON_MOSTADEER: &str = "۟";
    pub const SKOON_MOSTATEEL: &str = "۠";
    pub const MEEM_IQLAB: &str = "ۢ";
    pub const IMALA_SIGN: &str = "۪";
    pub const ISHMAM_SIGN: &str = "۫";
    pub const TASHEEL_SIGN: &str = "۬";
    pub const TANWEEN_IDHAAM_DTERMINER: &str = "ۭ";
    pub const KASHEEDA: &str = "ـ";
    /// Small Uthmani letters and marks.
    pub const SMALL_SEEN_ABOVE: &str = "ۜ";
    pub const SMALL_SEEN_BELOW: &str = "ۣ";
    pub const SMALL_WAW: &str = "ۥ";
    pub const SMALL_YAA_SILA: &str = "ۦ";
    pub const SMALL_YAA: &str = "ۧ";
    pub const SMALL_NOON: &str = "ۨ";

    /// Derived groups mirroring Python `__post_init__` (`self.fatha + self.alif`, ...).
    pub const MADD_ALIF: &str = constcat::concat!(FATHA, ALIF);
    pub const MADD_WAW: &str = constcat::concat!(DAMA, WAW);
    pub const MADD_YAA: &str = constcat::concat!(KASRA, YAA);
    pub const NOON_IKHFAA_GROUP: &str = constcat::concat!(
        SAAD,
        THAAL,
        THAA,
        KAF,
        JEEM,
        SHEEN,
        QAF,
        SEEN,
        DAAL,
        TAA_MOFAKHAMA,
        ZAY,
        FAA,
        TAA_MABSOOTA,
        DAAD,
        ZAA_MOFAKHAMA
    );
    pub const NOON_IDGHAM_GROUP: &str = constcat::concat!(YAA, RAA, MEEM, LAM, WAW, NOON);
    pub const HARAKAT_GROUP: &str = constcat::concat!(FATHA, DAMA, KASRA);
    pub const HAMAZAT_GROUP: &str = constcat::concat!(
        HAMZA,
        HAMZA_ABOVE_ALIF,
        HAMZA_BELOW_ALIF,
        HAMZA_ABOVE_WAW,
        HAMZA_ABOVE_YAA,
        HAMZA_MAMDODA
    );
    pub const LETTERS_GROUP: &str = constcat::concat!(
        ALIF,
        ALIF_MAKSORA,
        BAA,
        TAA_MABSOOTA,
        TAA_MARBOOTA,
        THAA,
        JEEM,
        HAA_MOHMALA,
        KHAA,
        DAAL,
        THAAL,
        RAA,
        ZAY,
        SEEN,
        SHEEN,
        SAAD,
        DAAD,
        TAA_MOFAKHAMA,
        ZAA_MOFAKHAMA,
        AYN,
        GHYN,
        FAA,
        QAF,
        KAF,
        LAM,
        MEEM,
        NOON,
        HAA,
        WAW,
        YAA,
        HAMZA
    );
    pub const PURE_LETTERS_GROUP: &str = constcat::concat!(
        BAA,
        TAA_MABSOOTA,
        THAA,
        JEEM,
        HAA_MOHMALA,
        KHAA,
        DAAL,
        THAAL,
        RAA,
        ZAY,
        SEEN,
        SHEEN,
        SAAD,
        DAAD,
        TAA_MOFAKHAMA,
        ZAA_MOFAKHAMA,
        AYN,
        GHYN,
        FAA,
        QAF,
        KAF,
        LAM,
        MEEM,
        NOON,
        HAA,
        WAW,
        YAA,
        HAMZA
    );
    pub const PURE_LETTERS_WITHOUT_YAA_AND_WAW_GROUP: &str = constcat::concat!(
        BAA,
        TAA_MABSOOTA,
        THAA,
        JEEM,
        HAA_MOHMALA,
        KHAA,
        DAAL,
        THAAL,
        RAA,
        ZAY,
        SEEN,
        SHEEN,
        SAAD,
        DAAD,
        TAA_MOFAKHAMA,
        ZAA_MOFAKHAMA,
        AYN,
        GHYN,
        FAA,
        QAF,
        KAF,
        LAM,
        MEEM,
        NOON,
        HAA,
        HAMZA
    );
    pub const QLQLA_GROUP: &str = constcat::concat!(QAF, TAA_MOFAKHAMA, BAA, JEEM, DAAL);

    /// Tanween variants used by tajweed rules.
    pub const TANWEEN_FATH_MOTHHAR: &str = TANWEEN_FATH;
    pub const TANWEEN_DAM_MOTHHAR: &str = TANWEEN_DAM;
    pub const TANWEEN_KASR_MOTHHAR: &str = TANWEEN_KASR;
    pub const TANWEEN_FATH_MODGHAM: &str =
        constcat::concat!(TANWEEN_FATH, TANWEEN_IDHAAM_DTERMINER);
    pub const TANWEEN_DAM_MODGHAM: &str = constcat::concat!(TANWEEN_DAM, TANWEEN_IDHAAM_DTERMINER);
    pub const TANWEEN_KASR_MODGHAM: &str =
        constcat::concat!(TANWEEN_KASR, TANWEEN_IDHAAM_DTERMINER);
    pub const TANWEEN_FATH_IQLAB: &str = constcat::concat!(TANWEEN_FATH, MEEM_IQLAB);
    pub const TANWEEN_DAM_IQLAB: &str = constcat::concat!(TANWEEN_DAM, MEEM_IQLAB);
    pub const TANWEEN_KASR_IQLAB: &str = constcat::concat!(TANWEEN_KASR, MEEM_IQLAB);
}

/// Characters used by the Quran phonetic script.
pub mod phonetic {
    pub const HAMZA: &str = super::uthmani::HAMZA;
    pub const BAA: &str = super::uthmani::BAA;
    pub const TAA: &str = super::uthmani::TAA_MABSOOTA;
    pub const THAA: &str = super::uthmani::THAA;
    pub const JEEM: &str = super::uthmani::JEEM;
    pub const HAA_MOHMALA: &str = super::uthmani::HAA_MOHMALA;
    pub const KHAA: &str = super::uthmani::KHAA;
    pub const DAAL: &str = super::uthmani::DAAL;
    pub const THAAL: &str = super::uthmani::THAAL;
    pub const RAA: &str = super::uthmani::RAA;
    pub const ZAY: &str = super::uthmani::ZAY;
    pub const SEEN: &str = super::uthmani::SEEN;
    pub const SHEEN: &str = super::uthmani::SHEEN;
    pub const SAAD: &str = super::uthmani::SAAD;
    pub const DAAD: &str = super::uthmani::DAAD;
    pub const TAA_MOFAKHAMA: &str = super::uthmani::TAA_MOFAKHAMA;
    pub const ZAA_MOFAKHAMA: &str = super::uthmani::ZAA_MOFAKHAMA;
    pub const AYN: &str = super::uthmani::AYN;
    pub const GHYN: &str = super::uthmani::GHYN;
    pub const FAA: &str = super::uthmani::FAA;
    pub const QAF: &str = super::uthmani::QAF;
    pub const KAF: &str = super::uthmani::KAF;
    pub const LAM: &str = super::uthmani::LAM;
    pub const MEEM: &str = super::uthmani::MEEM;
    pub const NOON: &str = super::uthmani::NOON;
    pub const HAA: &str = super::uthmani::HAA;
    pub const WAW: &str = super::uthmani::WAW;
    pub const YAA: &str = super::uthmani::YAA;

    /// Long-vowel characters.
    pub const ALIF: &str = super::uthmani::ALIF;
    pub const YAA_MADD: &str = super::uthmani::SMALL_YAA_SILA;
    pub const WAW_MADD: &str = super::uthmani::SMALL_WAW;

    /// Short-vowel characters.
    pub const FATHA: &str = super::uthmani::FATHA;
    pub const DAMA: &str = super::uthmani::DAMA;
    pub const KASRA: &str = super::uthmani::KASRA;

    /// Special phonetic-script characters.
    pub const FATHA_MOMALA: &str = super::uthmani::IMALA_SIGN;
    pub const ALIF_MOMALA: &str = super::uthmani::KASHEEDA;
    pub const HAMZA_MOSAHALA: &str = "\u{0672}";
    pub const QLQLA: &str = "\u{0687}";
    pub const NOON_MOKHFAH: &str = "\u{06ba}";
    pub const MEEM_MOKHFAH: &str = "\u{06fe}";
    pub const SAKT: &str = super::uthmani::SMALL_SEEN_ABOVE;
    pub const DAMA_MOKHTALASA: &str = "\u{0619}";
}

/// Phonetic character groups used to classify pronunciation properties.
///
/// Built with `constcat::concat!` in the same construction order as Python.
pub mod phonetic_groups {
    use super::phonetic;
    pub const CORE: &str = constcat::concat!(
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
        phonetic::HAMZA_MOSAHALA
    );
    pub const RESIDUALS: &str = constcat::concat!(
        phonetic::FATHA,
        phonetic::DAMA,
        phonetic::KASRA,
        phonetic::QLQLA,
        phonetic::FATHA_MOMALA,
        phonetic::SAKT,
        phonetic::DAMA_MOKHTALASA
    );
    pub const HARAKAT: &str = constcat::concat!(phonetic::FATHA, phonetic::DAMA, phonetic::KASRA);
    pub const HAMS: &str = constcat::concat!(
        phonetic::FAA,
        phonetic::HAA_MOHMALA,
        phonetic::THAA,
        phonetic::HAA,
        phonetic::SHEEN,
        phonetic::KHAA,
        phonetic::SAAD,
        phonetic::SEEN,
        phonetic::KAF,
        phonetic::TAA
    );
    pub const SHIDDA: &str = constcat::concat!(
        phonetic::HAMZA,
        phonetic::JEEM,
        phonetic::DAAL,
        phonetic::QAF,
        phonetic::TAA_MOFAKHAMA,
        phonetic::BAA,
        phonetic::KAF,
        phonetic::TAA
    );
    pub const BETWEEN_SHIDDA_RAKHAWA: &str = constcat::concat!(
        phonetic::LAM,
        phonetic::NOON,
        phonetic::AYN,
        phonetic::MEEM,
        phonetic::RAA
    );
    pub const TAFKHEEM: &str = constcat::concat!(
        phonetic::KHAA,
        phonetic::SAAD,
        phonetic::DAAD,
        phonetic::GHYN,
        phonetic::TAA_MOFAKHAMA,
        phonetic::QAF,
        phonetic::ZAA_MOFAKHAMA
    );
    pub const ITBAAQ: &str = constcat::concat!(
        phonetic::SAAD,
        phonetic::DAAD,
        phonetic::TAA_MOFAKHAMA,
        phonetic::ZAA_MOFAKHAMA
    );
    pub const SAFEER: &str = constcat::concat!(phonetic::SAAD, phonetic::ZAY, phonetic::SEEN);
    pub const QALQAL: &str = constcat::concat!(
        phonetic::QAF,
        phonetic::TAA_MOFAKHAMA,
        phonetic::BAA,
        phonetic::JEEM,
        phonetic::DAAL
    );
    pub const TIKRAR: &str = phonetic::RAA;
    pub const TAFASHIE: &str = phonetic::SHEEN;
    pub const ISTITALA: &str = phonetic::DAAD;
    pub const GHONNA: &str = constcat::concat!(
        phonetic::NOON,
        phonetic::MEEM,
        phonetic::NOON_MOKHFAH,
        phonetic::MEEM_MOKHFAH
    );
}

#[cfg(test)]
mod tests {
    use super::{PatternPosition, begin_hamzat_wasl, imlaey, phonetic, quran_alphabet, uthmani};

    #[test]
    #[allow(clippy::too_many_lines)]
    fn uthmani_constants_match_the_source_data() {
        let source = &quran_alphabet().uthmani;

        for (constant, field) in [
            (uthmani::SPACE, source.space.as_str()),
            (uthmani::ALIF, source.alif.as_str()),
            (uthmani::ALIF_MAKSORA, source.alif_maksora.as_str()),
            (uthmani::BAA, source.baa.as_str()),
            (uthmani::TAA_MABSOOTA, source.taa_mabsoota.as_str()),
            (uthmani::TAA_MARBOOTA, source.taa_marboota.as_str()),
            (uthmani::THAA, source.thaa.as_str()),
            (uthmani::JEEM, source.jeem.as_str()),
            (uthmani::HAA_MOHMALA, source.haa_mohmala.as_str()),
            (uthmani::KHAA, source.khaa.as_str()),
            (uthmani::DAAL, source.daal.as_str()),
            (uthmani::THAAL, source.thaal.as_str()),
            (uthmani::RAA, source.raa.as_str()),
            (uthmani::ZAY, source.zay.as_str()),
            (uthmani::SEEN, source.seen.as_str()),
            (uthmani::SHEEN, source.sheen.as_str()),
            (uthmani::SAAD, source.saad.as_str()),
            (uthmani::DAAD, source.daad.as_str()),
            (uthmani::TAA_MOFAKHAMA, source.taa_mofakhama.as_str()),
            (uthmani::ZAA_MOFAKHAMA, source.zaa_mofakhama.as_str()),
            (uthmani::AYN, source.ayn.as_str()),
            (uthmani::GHYN, source.ghyn.as_str()),
            (uthmani::FAA, source.faa.as_str()),
            (uthmani::QAF, source.qaf.as_str()),
            (uthmani::KAF, source.kaf.as_str()),
            (uthmani::LAM, source.lam.as_str()),
            (uthmani::MEEM, source.meem.as_str()),
            (uthmani::NOON, source.noon.as_str()),
            (uthmani::HAA, source.haa.as_str()),
            (uthmani::WAW, source.waw.as_str()),
            (uthmani::YAA, source.yaa.as_str()),
            (uthmani::HAMZA, source.hamza.as_str()),
            (uthmani::HAMZA_ABOVE_ALIF, source.hamza_above_alif.as_str()),
            (uthmani::HAMZA_BELOW_ALIF, source.hamza_below_alif.as_str()),
            (uthmani::HAMZA_ABOVE_WAW, source.hamza_above_waw.as_str()),
            (uthmani::HAMZA_ABOVE_YAA, source.hamza_above_yaa.as_str()),
            (uthmani::HAMZA_MAMDODA, source.hamza_mamdoda.as_str()),
            (uthmani::TANWEEN_FATH, source.tanween_fath.as_str()),
            (uthmani::TANWEEN_DAM, source.tanween_dam.as_str()),
            (uthmani::TANWEEN_KASR, source.tanween_kasr.as_str()),
            (uthmani::FATHA, source.fatha.as_str()),
            (uthmani::DAMA, source.dama.as_str()),
            (uthmani::KASRA, source.kasra.as_str()),
            (uthmani::SHADDA, source.shadda.as_str()),
            (uthmani::RAS_HAAA, source.ras_haaa.as_str()),
            (uthmani::MADD, source.madd.as_str()),
            (uthmani::HAMZAT_WASL, source.hamzat_wasl.as_str()),
            (uthmani::ALIF_KHNJARIA, source.alif_khnjaria.as_str()),
            (uthmani::SMALL_SEEN_ABOVE, source.small_seen_above.as_str()),
            (uthmani::SMALL_SEEN_BELOW, source.small_seen_below.as_str()),
            (uthmani::SMALL_WAW, source.small_waw.as_str()),
            (uthmani::SMALL_YAA_SILA, source.small_yaa_sila.as_str()),
            (uthmani::SMALL_YAA, source.small_yaa.as_str()),
            (uthmani::SMALL_NOON, source.small_noon.as_str()),
            (uthmani::SKOON_MOSTADEER, source.skoon_mostadeer.as_str()),
            (uthmani::SKOON_MOSTATEEL, source.skoon_mostateel.as_str()),
            (uthmani::MEEM_IQLAB, source.meem_iqlab.as_str()),
            (uthmani::IMALA_SIGN, source.imala_sign.as_str()),
            (uthmani::ISHMAM_SIGN, source.ishmam_sign.as_str()),
            (uthmani::TASHEEL_SIGN, source.tasheel_sign.as_str()),
            (
                uthmani::TANWEEN_IDHAAM_DTERMINER,
                source.tanween_idhaam_dterminer.as_str(),
            ),
            (uthmani::KASHEEDA, source.kasheeda.as_str()),
        ] {
            assert_eq!(constant, field);
        }

        for (group, field) in [
            (uthmani::MADD_ALIF, source.madd_alif.as_str()),
            (uthmani::MADD_WAW, source.madd_waw.as_str()),
            (uthmani::MADD_YAA, source.madd_yaa.as_str()),
            (
                uthmani::NOON_IKHFAA_GROUP,
                source.noon_ikhfaa_group.as_str(),
            ),
            (
                uthmani::NOON_IDGHAM_GROUP,
                source.noon_idgham_group.as_str(),
            ),
            (uthmani::HARAKAT_GROUP, source.harakat_group.as_str()),
            (uthmani::HAMAZAT_GROUP, source.hamazat_group.as_str()),
            (uthmani::LETTERS_GROUP, source.letters_group.as_str()),
            (
                uthmani::PURE_LETTERS_GROUP,
                source.pure_letters_group.as_str(),
            ),
            (
                uthmani::PURE_LETTERS_WITHOUT_YAA_AND_WAW_GROUP,
                source.pure_letters_without_yaa_and_waw_group.as_str(),
            ),
            (uthmani::QLQLA_GROUP, source.qlqla_group.as_str()),
            (
                uthmani::TANWEEN_FATH_MOTHHAR,
                source.tanween_fath_mothhar.as_str(),
            ),
            (
                uthmani::TANWEEN_DAM_MOTHHAR,
                source.tanween_dam_mothhar.as_str(),
            ),
            (
                uthmani::TANWEEN_KASR_MOTHHAR,
                source.tanween_kasr_mothhar.as_str(),
            ),
            (
                uthmani::TANWEEN_FATH_MODGHAM,
                source.tanween_fath_modgham.as_str(),
            ),
            (
                uthmani::TANWEEN_DAM_MODGHAM,
                source.tanween_dam_modgham.as_str(),
            ),
            (
                uthmani::TANWEEN_KASR_MODGHAM,
                source.tanween_kasr_modgham.as_str(),
            ),
            (
                uthmani::TANWEEN_FATH_IQLAB,
                source.tanween_fath_iqlab.as_str(),
            ),
            (
                uthmani::TANWEEN_DAM_IQLAB,
                source.tanween_dam_iqlab.as_str(),
            ),
            (
                uthmani::TANWEEN_KASR_IQLAB,
                source.tanween_kasr_iqlab.as_str(),
            ),
        ] {
            assert_eq!(group, field);
        }
    }

    #[test]
    fn imlaey_constants_match_the_source_data() {
        let source = &quran_alphabet().imlaey;

        for (constant, field) in [
            (imlaey::ALPHABET, source.alphabet.as_str()),
            (imlaey::HAMAZAT, source.hamazat.as_str()),
            (imlaey::TASHKEEL, source.tashkeel.as_str()),
            (imlaey::HAMZA, source.hamza.as_str()),
            (imlaey::ALEF, source.alef.as_str()),
            (imlaey::ALEF_MAKSOORA, source.alef_maksoora.as_str()),
            (imlaey::TAA_MARBOOTA, source.taa_marboota.as_str()),
            (imlaey::TAA_MABSOOTA, source.taa_mabsoota.as_str()),
            (imlaey::HAA, source.haa.as_str()),
            (imlaey::SMALL_ALEF, source.small_alef.as_str()),
            (imlaey::SKOON, source.skoon.as_str()),
        ] {
            assert_eq!(constant, field);
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

        assert_eq!(phonetic::HAMZA_MOSAHALA, "ٲ");
        assert_eq!(phonetic::QLQLA, "\u{0687}");
        assert_eq!(phonetic::NOON_MOKHFAH, "\u{06ba}");
        assert_eq!(phonetic::MEEM_MOKHFAH, "\u{06fe}");
        assert_eq!(phonetic::DAMA_MOKHTALASA, "\u{0619}");
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn phonetic_groups_match_the_python_construction_order() {
        use super::phonetic_groups;

        assert_eq!(
            phonetic_groups::CORE,
            [
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
            ]
            .concat()
        );
        assert_eq!(
            phonetic_groups::RESIDUALS,
            [
                phonetic::FATHA,
                phonetic::DAMA,
                phonetic::KASRA,
                phonetic::QLQLA,
                phonetic::FATHA_MOMALA,
                phonetic::SAKT,
                phonetic::DAMA_MOKHTALASA,
            ]
            .concat()
        );
        assert_eq!(
            phonetic_groups::HARAKAT,
            [phonetic::FATHA, phonetic::DAMA, phonetic::KASRA].concat()
        );
        assert_eq!(
            phonetic_groups::HAMS,
            [
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
            ]
            .concat()
        );
        assert_eq!(
            phonetic_groups::SHIDDA,
            [
                phonetic::HAMZA,
                phonetic::JEEM,
                phonetic::DAAL,
                phonetic::QAF,
                phonetic::TAA_MOFAKHAMA,
                phonetic::BAA,
                phonetic::KAF,
                phonetic::TAA,
            ]
            .concat()
        );
        assert_eq!(
            phonetic_groups::BETWEEN_SHIDDA_RAKHAWA,
            [
                phonetic::LAM,
                phonetic::NOON,
                phonetic::AYN,
                phonetic::MEEM,
                phonetic::RAA,
            ]
            .concat()
        );
        assert_eq!(
            phonetic_groups::TAFKHEEM,
            [
                phonetic::KHAA,
                phonetic::SAAD,
                phonetic::DAAD,
                phonetic::GHYN,
                phonetic::TAA_MOFAKHAMA,
                phonetic::QAF,
                phonetic::ZAA_MOFAKHAMA,
            ]
            .concat()
        );
        assert_eq!(
            phonetic_groups::ITBAAQ,
            [
                phonetic::SAAD,
                phonetic::DAAD,
                phonetic::TAA_MOFAKHAMA,
                phonetic::ZAA_MOFAKHAMA,
            ]
            .concat()
        );
        assert_eq!(
            phonetic_groups::SAFEER,
            [phonetic::SAAD, phonetic::ZAY, phonetic::SEEN].concat()
        );
        assert_eq!(
            phonetic_groups::QALQAL,
            [
                phonetic::QAF,
                phonetic::TAA_MOFAKHAMA,
                phonetic::BAA,
                phonetic::JEEM,
                phonetic::DAAL,
            ]
            .concat()
        );
        assert_eq!(phonetic_groups::TIKRAR, phonetic::RAA);
        assert_eq!(phonetic_groups::TAFASHIE, phonetic::SHEEN);
        assert_eq!(phonetic_groups::ISTITALA, phonetic::DAAD);
        assert_eq!(
            phonetic_groups::GHONNA,
            [
                phonetic::NOON,
                phonetic::MEEM,
                phonetic::NOON_MOKHFAH,
                phonetic::MEEM_MOKHFAH,
            ]
            .concat()
        );
    }
}
