//! Norwegian stop words (from Lucene/Snowball project).

use alloc::borrow::Cow;
use alloc::vec::Vec;
use hashbrown::HashSet;
use once_cell::sync::Lazy;
use pizza_engine::analysis::{Token, TokenFilter};

/// Default Norwegian stop words sourced from Apache Lucene.
static DEFAULT_STOP_WORDS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let words: &[&str] = &[
    "alle",
    "at",
    "av",
    "bare",
    "begge",
    "ble",
    "blei",
    "bli",
    "blir",
    "blitt",
    "både",
    "båe",
    "da",
    "de",
    "deg",
    "dei",
    "deim",
    "deira",
    "deires",
    "dem",
    "den",
    "denne",
    "der",
    "dere",
    "deres",
    "det",
    "dette",
    "di",
    "din",
    "disse",
    "ditt",
    "du",
    "dykk",
    "dykkar",
    "då",
    "eg",
    "ein",
    "eit",
    "eitt",
    "eller",
    "elles",
    "en",
    "enn",
    "er",
    "et",
    "ett",
    "etter",
    "for",
    "fordi",
    "fra",
    "før",
    "ha",
    "hadde",
    "han",
    "hans",
    "har",
    "hennar",
    "henne",
    "hennes",
    "her",
    "hjå",
    "ho",
    "hoe",
    "honom",
    "hoss",
    "hossen",
    "hun",
    "hva",
    "hvem",
    "hver",
    "hvilke",
    "hvilken",
    "hvis",
    "hvor",
    "hvordan",
    "hvorfor",
    "i",
    "ikke",
    "ikkje",
    "ingen",
    "ingi",
    "inkje",
    "inn",
    "inni",
    "ja",
    "jeg",
    "kan",
    "kom",
    "korleis",
    "korso",
    "kun",
    "kunne",
    "kva",
    "kvar",
    "kvarhelst",
    "kven",
    "kvi",
    "kvifor",
    "man",
    "mange",
    "me",
    "med",
    "medan",
    "meg",
    "meget",
    "mellom",
    "men",
    "mi",
    "min",
    "mine",
    "mitt",
    "mot",
    "mykje",
    "ned",
    "no",
    "noe",
    "noen",
    "noka",
    "noko",
    "nokon",
    "nokor",
    "nokre",
    "nå",
    "når",
    "og",
    "også",
    "om",
    "opp",
    "oss",
    "over",
    "på",
    "samme",
    "seg",
    "selv",
    "si",
    "sia",
    "sidan",
    "siden",
    "sin",
    "sine",
    "sitt",
    "sjøl",
    "skal",
    "skulle",
    "slik",
    "so",
    "som",
    "somme",
    "somt",
    "så",
    "sånn",
    "til",
    "um",
    "upp",
    "ut",
    "uten",
    "var",
    "vart",
    "varte",
    "ved",
    "vere",
    "verte",
    "vi",
    "vil",
    "ville",
    "vore",
    "vors",
    "vort",
    "vår",
    "være",
    "vært",
    "å",
    ];
    words.iter().copied().collect()
});

/// Removes Norwegian stop words from the token stream.
#[derive(Clone, Debug)]
pub struct NorwegianStopFilter {
    stop_words: HashSet<String>,
}

impl Default for NorwegianStopFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl NorwegianStopFilter {
    pub fn new() -> Self {
        Self {
            stop_words: DEFAULT_STOP_WORDS.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn with_words(words: &[&str]) -> Self {
        Self {
            stop_words: words.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl TokenFilter for NorwegianStopFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let term = token.term.as_ref();
        if self.stop_words.contains(term) {
            return (true, None);
        }
        (false, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stop_word_count() {
        assert!(DEFAULT_STOP_WORDS.len() >= 172);
    }

    #[test]
    fn test_filters_stop_word() {
        let f = NorwegianStopFilter::new();
        let word = DEFAULT_STOP_WORDS.iter().next().unwrap();
        let mut token = Token::new(word, 0, word.len() as u32, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted);
    }

    #[test]
    fn test_passes_non_stop_word() {
        let f = NorwegianStopFilter::new();
        let mut token = Token::new("xyzzy_not_a_stop_word", 0, 21, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(!deleted);
    }

    #[test]
    fn test_custom_words() {
        let f = NorwegianStopFilter::with_words(&["custom", "words"]);
        let mut token = Token::new("custom", 0, 6, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted);
    }
}
