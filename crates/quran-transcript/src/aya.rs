//! C2a: Quran navigation (`Aya`) and shared format types.
//!
//! Ports blocks A (containers) and B (navigation) of `utils.py` from
//! `quran-transcript`. Suras and ayat use 1-based numbers (Quranic standard);
//! word indices are 0-based with exclusive ends (Python-style). All text
//! handling is character-based; this crate never byte-indexes Arabic text.

use std::{collections::HashMap, error::Error, fmt};

use crate::{alphabet, quran_data::quran_text};

/// Number of suras in the Holy Quran.
pub const NUM_SURAS: usize = 114;

/// Errors for Quran navigation and normalisation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AyaError {
    /// Sura number outside `1..=114`.
    InvalidSura {
        /// The rejected sura number.
        got: usize,
    },
    /// Aya number outside `1..=sura_len`.
    InvalidAya {
        /// The containing sura number.
        sura: usize,
        /// The rejected aya number.
        got: usize,
        /// Number of ayat in the sura.
        max: usize,
    },
    /// `ignore_taa_marboota` and `normalize_taat` set at the same time.
    ConflictingTaaOptions,
}

impl fmt::Display for AyaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSura { got } => write!(f, "wrong sura index {got} (want 1..=114)"),
            Self::InvalidAya { sura, got, max } => write!(
                f,
                "aya index out of range (sura_index={sura} aya_index={got}) and length of sura={max}"
            ),
            Self::ConflictingTaaOptions => write!(
                f,
                "you can not `ignore_taa_marboota` and `normalize_taat` at the same time"
            ),
        }
    }
}

impl Error for AyaError {}

/// One word-level Uthmani-to-Imlaey spelling pair.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RasmEntry {
    /// The Uthmani word (or words).
    pub uthmani: String,
    /// The Imlaey word (or words).
    pub imlaey: String,
}

/// Exceptional word mappings split per word, mirroring
/// [`AyaFormat::get_formatted_rasm_map`] in Python.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RasmFormat {
    /// Uthmani words per map entry.
    pub uthmani: Vec<Vec<String>>,
    /// Imlaey words per map entry.
    pub imlaey: Vec<Vec<String>>,
}

/// A word span with an exclusive end; `end` of `None` means to the last word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WordSpan {
    /// Start word index (inclusive).
    pub start: usize,
    /// End word index (exclusive), or the end of the aya.
    pub end: Option<usize>,
}

/// A bidirectional word index between the two scripts (exclusive boundary).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuranWordIndex {
    /// Word index in Imlaey script.
    pub imlaey: usize,
    /// Word index in Uthmani script.
    pub uthmani: usize,
}

/// One aya in both scripts with its metadata, mirroring `AyaFormat`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AyaFormat {
    /// Sura number, 1-based.
    pub sura_idx: usize,
    /// Aya number, 1-based.
    pub aya_idx: usize,
    /// Name of the sura.
    pub sura_name: String,
    /// Number of ayat in the sura.
    pub num_ayat_in_sura: usize,
    /// The aya in Uthmani script.
    pub uthmani: String,
    /// Uthmani words of the aya.
    pub uthmani_words: Vec<String>,
    /// The aya in Imlaey script.
    pub imlaey: String,
    /// Imlaey words of the aya.
    pub imlaey_words: Vec<String>,
    /// Istiaatha in Uthmani script.
    pub istiaatha_uthmani: String,
    /// Istiaatha in Imlaey script.
    pub istiaatha_imlaey: String,
    /// Word-level script map; `None` here because the map asset is not
    /// embedded (see C1). The step-4 alignment engine reconstructs the
    /// mapping from the word lists instead.
    pub rasm_map: Option<Vec<RasmEntry>>,
    /// Bismillah in Uthmani script (`None` unless first aya of a sura
    /// that has one).
    pub bismillah_uthmani: Option<String>,
    /// Bismillah in Imlaey script (`None` unless first aya of a sura
    /// that has one).
    pub bismillah_imlaey: Option<String>,
    /// Word-level Bismillah map; `None` (map asset not embedded).
    pub bismillah_map: Option<Vec<RasmEntry>>,
}

/// Output of the Imlaey-to-Uthmani encoder (step 4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncodingOutput {
    /// Map from Imlaey word indices to Uthmani word indices.
    pub imlaey2uthmani: HashMap<usize, usize>,
    /// Uthmani words (with optional istiaatha/bismillah/sadaka affixes).
    pub uthmani_words: Vec<String>,
    /// Imlaey words (with optional istiaatha/bismillah/sadaka affixes).
    pub imlaey_words: Vec<String>,
    /// `(start, end)` word span of the core aya content.
    pub aya_imlaey_span_words: (usize, usize),
    /// Word span of the istiaatha prefix, if present.
    pub istiaatha_imlaey_span_words: Option<(usize, usize)>,
    /// Word span of the bismillah prefix, if present.
    pub bismillah_imlaey_span_words: Option<(usize, usize)>,
    /// Word span of the sadaka suffix, if present.
    pub sadaka_imlaey_span_words: Option<(usize, usize)>,
}

/// Output of converting one Imlaey span to Uthmani (step 4).
// The four flags mirror Python `Imlaey2uthmaniOutput` one-to-one.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::struct_excessive_bools)]
pub struct Imlaey2uthmaniOutput {
    /// The input Imlaey text segment.
    pub imlaey: String,
    /// The converted Uthmani text.
    pub uthmani: String,
    /// Start of the Quranic content, if any.
    pub quran_start: Option<QuranWordIndex>,
    /// End of the Quranic content, if any.
    pub quran_end: Option<QuranWordIndex>,
    /// Whether the segment contains istiaatha.
    pub has_istiaatha: bool,
    /// Whether the segment contains bismillah.
    pub has_bismillah: bool,
    /// Whether the segment contains sadaka.
    pub has_sadaka: bool,
    /// Whether the segment contains core Quranic text.
    pub has_quran: bool,
}

/// A text segment in both scripts with dual indexing (step 5).
///
/// Note: Python names the Imlaey field `imalaey` (typo); it is spelled
/// correctly here.
// The four flags mirror Python `SegmentScripts` one-to-one.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::struct_excessive_bools)]
pub struct SegmentScripts {
    /// Full Imlaey script text.
    pub imlaey: String,
    /// Full Uthmani script text.
    pub uthmani: String,
    /// Whether the segment contains istiaatha.
    pub has_istiaatha: bool,
    /// Whether the segment contains bismillah.
    pub has_bismillah: bool,
    /// Whether the segment contains sadaka.
    pub has_sadaka: bool,
    /// Whether the segment contains core Quranic text.
    pub has_quran: bool,
    /// `(sura, aya, word)` start position, if Quranic content exists.
    pub start_span: Option<(usize, usize, QuranWordIndex)>,
    /// `(sura, aya, word)` end position, if Quranic content exists.
    pub end_span: Option<(usize, usize, QuranWordIndex)>,
}

/// A cursor into the Quran, mirroring Python `Aya`.
///
/// Stores 1-based sura/aya numbers. Navigation (`step`, `ayat_after`) wraps
/// around the end of the Quran, exactly like the Python version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// The position of an `Aya` is a sura number plus an aya number; renaming
// either field would only obscure the port.
#[allow(clippy::struct_field_names)]
pub struct Aya {
    sura_no: usize,
    aya_no: usize,
    start_imlaey_word_idx: usize,
}

fn sura_len(sura: usize) -> usize {
    quran_text().quran.surahs[sura - 1].ayahs.len()
}

fn check_indices(sura: usize, aya: usize) -> Result<(), AyaError> {
    if !(1..=NUM_SURAS).contains(&sura) {
        return Err(AyaError::InvalidSura { got: sura });
    }
    let max = sura_len(sura);
    if !(1..=max).contains(&aya) {
        return Err(AyaError::InvalidAya {
            sura,
            got: aya,
            max,
        });
    }
    Ok(())
}

impl Aya {
    /// Creates a cursor at `(sura, aya)`, both 1-based.
    ///
    /// # Errors
    ///
    /// Returns [`AyaError::InvalidSura`] or [`AyaError::InvalidAya`] for
    /// out-of-range indices.
    #[must_use = "the cursor is not registered anywhere"]
    pub fn new(sura: usize, aya: usize) -> Result<Self, AyaError> {
        check_indices(sura, aya)?;
        Ok(Self {
            sura_no: sura,
            aya_no: aya,
            start_imlaey_word_idx: 0,
        })
    }

    /// Sura number, 1-based.
    #[must_use]
    pub fn sura_idx(&self) -> usize {
        self.sura_no
    }

    /// Aya number, 1-based.
    #[must_use]
    pub fn aya_idx(&self) -> usize {
        self.aya_no
    }

    /// Saved Imlaey word offset used by word-based stepping (step 4).
    #[must_use]
    pub fn start_imlaey_word_idx(&self) -> usize {
        self.start_imlaey_word_idx
    }

    /// Sets the saved Imlaey word offset, mirroring Python `set`.
    pub fn set_start_imlaey_word_idx(&mut self, idx: usize) {
        self.start_imlaey_word_idx = idx;
    }

    /// Moves this cursor to `(sura, aya)`, both 1-based.
    ///
    /// # Errors
    ///
    /// Returns [`AyaError::InvalidSura`] or [`AyaError::InvalidAya`] for
    /// out-of-range indices; the cursor is left unchanged on error.
    pub fn set(&mut self, sura: usize, aya: usize) -> Result<(), AyaError> {
        check_indices(sura, aya)?;
        self.sura_no = sura;
        self.aya_no = aya;
        Ok(())
    }

    /// Returns a new cursor at `(sura, aya)`, both 1-based.
    ///
    /// # Errors
    ///
    /// Returns [`AyaError::InvalidSura`] or [`AyaError::InvalidAya`] for
    /// out-of-range indices.
    #[must_use = "the new cursor is not registered anywhere"]
    pub fn set_new(&self, sura: usize, aya: usize) -> Result<Self, AyaError> {
        let mut new = *self;
        new.set(sura, aya)?;
        new.start_imlaey_word_idx = 0;
        Ok(new)
    }

    /// Returns the aya this cursor points at.
    #[must_use]
    pub fn get(&self) -> AyaFormat {
        let document = quran_text();
        let sura = &document.quran.surahs[self.sura_no - 1];
        let aya = &sura.ayahs[self.aya_no - 1];
        let alphabet = alphabet::quran_alphabet();
        AyaFormat {
            sura_idx: self.sura_no,
            aya_idx: self.aya_no,
            sura_name: sura.name.clone(),
            num_ayat_in_sura: sura.ayahs.len(),
            uthmani: aya.uthmani.clone(),
            uthmani_words: aya.uthmani.split(' ').map(str::to_owned).collect(),
            imlaey: aya.imlaey.clone(),
            imlaey_words: aya.imlaey.split(' ').map(str::to_owned).collect(),
            istiaatha_uthmani: alphabet.istiaatha.uthmani.clone(),
            istiaatha_imlaey: alphabet.istiaatha.imlaey.clone(),
            rasm_map: None,
            bismillah_uthmani: aya.bismillah_uthmani.clone(),
            bismillah_imlaey: aya.bismillah_imlaey.clone(),
            bismillah_map: None,
        }
    }

    /// Whether this is the last aya of its sura.
    #[must_use]
    pub fn is_last(&self) -> bool {
        self.aya_no == self.get().num_ayat_in_sura
    }

    /// Returns the cursor `steps` ayat after (or before, if negative).
    ///
    /// Wraps around the ends of the Quran, exactly like Python `step`.
    #[must_use]
    pub fn step(&self, steps: i32) -> Self {
        let mut sura_no = self.sura_no;
        let mut aya_no = self.aya_no;
        if steps >= 0 {
            for _ in 0..steps {
                aya_no += 1;
                if aya_no > sura_len(sura_no) {
                    aya_no = 1;
                    sura_no = sura_no % NUM_SURAS + 1;
                }
            }
        } else {
            for _ in steps..0 {
                if aya_no > 1 {
                    aya_no -= 1;
                } else {
                    sura_no = (sura_no + NUM_SURAS - 2) % NUM_SURAS + 1;
                    aya_no = sura_len(sura_no);
                }
            }
        }
        Self {
            sura_no,
            aya_no,
            start_imlaey_word_idx: 0,
        }
    }

    /// Iterates over ayat starting at this cursor.
    ///
    /// With `Some(n)` yields `n` ayat with wraparound (mirroring Python
    /// `get_ayat_after(num_ayat=n)`); with `None` yields up to and including
    /// the last aya of the Quran without wrapping.
    pub fn ayat_after(&self, num_ayat: Option<usize>) -> impl Iterator<Item = Self> {
        let mut current = Some(*self);
        let mut remaining = num_ayat;
        std::iter::from_fn(move || {
            if remaining == Some(0) {
                return None;
            }
            let out = current.take()?;
            if let Some(left) = remaining.as_mut() {
                // Bounded mode wraps around the whole Quran like Python.
                *left -= 1;
                current = Some(out.step(1));
            } else if out.sura_no == NUM_SURAS && out.aya_no == sura_len(out.sura_no) {
                // Unbounded mode stops at the last aya without wrapping.
            } else {
                current = Some(out.step(1));
            }
            Some(out)
        })
    }
}

impl fmt::Display for Aya {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Aya(sura_idx={}, aya_idx={})", self.sura_no, self.aya_no)
    }
}

#[cfg(test)]
mod tests {
    use super::{Aya, AyaError};

    #[test]
    fn get_returns_the_expected_fatiha_view() {
        let aya = Aya::new(1, 1).expect("1:1 must exist");
        let view = aya.get();

        assert_eq!(view.sura_idx, 1);
        assert_eq!(view.aya_idx, 1);
        assert_eq!(view.sura_name, "الفاتحة");
        assert_eq!(view.num_ayat_in_sura, 7);
        assert!(!view.uthmani.is_empty());
        assert!(!view.imlaey.is_empty());
        assert_eq!(view.uthmani_words.join(" "), view.uthmani);
        assert_eq!(view.imlaey_words.join(" "), view.imlaey);
        assert!(!view.istiaatha_uthmani.is_empty());
        assert!(!view.istiaatha_imlaey.is_empty());
        // Fatiha's Bismillah is an aya, so no separate prefix.
        assert!(view.bismillah_uthmani.is_none());
        assert!(view.bismillah_imlaey.is_none());
        assert!(view.rasm_map.is_none());
    }

    #[test]
    fn bismillah_prefix_only_on_first_ayat() {
        assert!(
            Aya::new(2, 1)
                .expect("2:1")
                .get()
                .bismillah_uthmani
                .is_some()
        );
        assert!(
            Aya::new(2, 1)
                .expect("2:1")
                .get()
                .bismillah_imlaey
                .is_some()
        );
        assert!(
            Aya::new(2, 2)
                .expect("2:2")
                .get()
                .bismillah_uthmani
                .is_none()
        );
        // Tawbah has no Bismillah at all.
        assert!(
            Aya::new(9, 1)
                .expect("9:1")
                .get()
                .bismillah_uthmani
                .is_none()
        );
    }

    #[test]
    fn constructor_rejects_out_of_range_indices() {
        assert_eq!(Aya::new(0, 1), Err(AyaError::InvalidSura { got: 0 }));
        assert_eq!(Aya::new(115, 1), Err(AyaError::InvalidSura { got: 115 }));
        assert_eq!(
            Aya::new(1, 8),
            Err(AyaError::InvalidAya {
                sura: 1,
                got: 8,
                max: 7
            })
        );
        assert_eq!(
            Aya::new(114, 7),
            Err(AyaError::InvalidAya {
                sura: 114,
                got: 7,
                max: 6
            })
        );
    }

    #[test]
    fn step_moves_across_ayat_suras_and_wraps() {
        let first = Aya::new(1, 1).expect("1:1");
        assert_eq!(first.step(0), first);
        assert_eq!(first.step(1), Aya::new(1, 2).expect("1:2"));
        assert_eq!(
            Aya::new(1, 7).expect("1:7").step(1),
            Aya::new(2, 1).expect("2:1")
        );
        assert_eq!(
            Aya::new(2, 1).expect("2:1").step(-1),
            Aya::new(1, 7).expect("1:7")
        );
        assert_eq!(
            Aya::new(114, 6).expect("114:6").step(1),
            Aya::new(1, 1).expect("1:1")
        );
        assert_eq!(
            Aya::new(1, 1).expect("1:1").step(-1),
            Aya::new(114, 6).expect("114:6")
        );
        // A full Quran cycle is 6236 ayat.
        assert_eq!(first.step(6236), first);
        assert_eq!(first.step(-6236), first);
    }

    #[test]
    fn set_and_set_new_update_positions() {
        let mut aya = Aya::new(1, 1).expect("1:1");
        aya.set(114, 2).expect("114:2 must exist");
        assert_eq!((aya.sura_idx(), aya.aya_idx()), (114, 2));
        assert!(aya.set(114, 7).is_err());
        // Failed `set` leaves the cursor unchanged.
        assert_eq!((aya.sura_idx(), aya.aya_idx()), (114, 2));

        let branched = aya.set_new(4, 4).expect("4:4 must exist");
        assert_eq!((branched.sura_idx(), branched.aya_idx()), (4, 4));
        assert_eq!((aya.sura_idx(), aya.aya_idx()), (114, 2));
        assert!(aya.set_new(0, 1).is_err());
    }

    #[test]
    fn is_last_detects_sura_ends() {
        assert!(Aya::new(1, 7).expect("1:7").is_last());
        assert!(!Aya::new(1, 1).expect("1:1").is_last());
        assert!(Aya::new(114, 6).expect("114:6").is_last());
    }

    #[test]
    fn ayat_after_yields_bounded_windows_with_wraparound() {
        let start = Aya::new(114, 5).expect("114:5");
        let collected: Vec<Aya> = start.ayat_after(Some(10)).collect();

        assert_eq!(collected.len(), 10);
        assert_eq!(collected[0], start);
        assert_eq!(collected[1], Aya::new(114, 6).expect("114:6"));
        assert_eq!(collected[2], Aya::new(1, 1).expect("1:1"));
        assert_eq!(
            *collected.last().expect("non-empty"),
            Aya::new(2, 1).expect("2:1")
        );
    }

    #[test]
    fn ayat_after_without_bound_runs_to_the_end_of_the_quran() {
        let from_start: Vec<Aya> = Aya::new(1, 1).expect("1:1").ayat_after(None).collect();
        assert_eq!(from_start.len(), 6236);
        assert_eq!(
            *from_start.last().expect("non-empty"),
            Aya::new(114, 6).expect("114:6")
        );

        let from_end: Vec<Aya> = Aya::new(114, 6).expect("114:6").ayat_after(None).collect();
        assert_eq!(from_end.len(), 1);
    }
}
