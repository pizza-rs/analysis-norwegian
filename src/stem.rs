use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use pizza_engine::analysis::Token;
use pizza_engine::analysis::TokenFilter;

/// Norwegian light stemmer. Removes common noun/adjective/verb suffixes
/// for both Bokmål and Nynorsk.
#[derive(Clone, Debug, Default)]
pub struct NorwegianLightStemFilter;

impl NorwegianLightStemFilter {
    pub fn new() -> Self {
        Self
    }
}

impl TokenFilter for NorwegianLightStemFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let text = token.term.as_ref();
        let len = text.len();
        if len < 4 {
            return (false, None);
        }

        let stemmed = stem_norwegian(text);
        if stemmed.len() != len {
            token.term = Cow::Owned(stemmed);
        }
        (false, None)
    }
}

fn stem_norwegian(word: &str) -> String {
    let suffixes: &[&str] = &[
        // Longest first
        "hetenes", "hetene", "hetens",
        "heten", "heter",
        "elsene", "elsen", "elser",
        "ingene", "inger", "ingen",
        "erende", "ernes", "ering",
        "enes", "erte",
        "ene", "ane", "ere", "est",
        "ing", "het", "ens", "lig", "isk",
        "ert", "dom",
        "en", "er", "et", "ar", "es", "as", "te",
        "e", "a", "s",
    ];

    for suffix in suffixes {
        if word.ends_with(suffix) {
            let stem_len = word.len() - suffix.len();
            if stem_len >= 3 {
                return word[..stem_len].to_string();
            }
        }
    }
    word.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stem_plural() {
        let filter = NorwegianLightStemFilter::new();
        let mut token = Token {
            term: Cow::Borrowed("guttene"),
            start_offset: 0,
            end_offset: 7,
            position: 0,
        };
        let (deleted, _) = filter.filter(&mut token);
        assert!(!deleted);
        assert_eq!(token.term.as_ref(), "gutt");
    }

    #[test]
    fn test_short_word_unchanged() {
        let filter = NorwegianLightStemFilter::new();
        let mut token = Token {
            term: Cow::Borrowed("og"),
            start_offset: 0,
            end_offset: 2,
            position: 0,
        };
        let (deleted, _) = filter.filter(&mut token);
        assert!(!deleted);
        assert_eq!(token.term.as_ref(), "og");
    }
}
