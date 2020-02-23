use std::fmt::{Display, Error, Formatter};
use std::sync::Arc;

use regex::Regex;

use crate::engine::util::regex_match;
use crate::item::{ItemWrapper, MatchedItem, MatchedRange, Rank};
use crate::SkimItem;
use crate::{CaseMatching, MatchEngine, MatchEngineFactory};

//------------------------------------------------------------------------------
// Regular Expression engine
#[derive(Debug)]
pub struct RegexEngine {
    query_regex: Option<Regex>,
}

impl RegexEngine {
    pub fn builder(query: &str, case: CaseMatching) -> Self {
        let mut query_builder = String::new();

        match case {
            CaseMatching::Respect => {}
            CaseMatching::Ignore => query_builder.push_str("(?i)"),
            CaseMatching::Smart => {}
        }

        query_builder.push_str(query);

        RegexEngine {
            query_regex: Regex::new(&query_builder).ok(),
        }
    }

    pub fn build(self) -> Self {
        self
    }
}

impl MatchEngine for RegexEngine {
    fn match_item(&self, item: Arc<ItemWrapper>) -> Option<MatchedItem> {
        let mut matched_result = None;
        for &(start, end) in item.get_matching_ranges().as_ref() {
            if self.query_regex.is_none() {
                matched_result = Some((0, 0));
                break;
            }

            matched_result =
                regex_match(&item.text()[start..end], &self.query_regex).map(|(s, e)| (s + start, e + start));

            if matched_result.is_some() {
                break;
            }
        }

        let (begin, end) = matched_result?;
        let score = (end - begin) as i64;
        let rank = Rank {
            score: -score,
            index: item.get_index() as i64,
            begin: begin as i64,
            end: end as i64,
        };

        Some(
            MatchedItem::builder(item)
                .rank(rank)
                .matched_range(MatchedRange::ByteRange(begin, end))
                .build(),
        )
    }
}

impl Display for RegexEngine {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "(Regex: {})",
            self.query_regex
                .as_ref()
                .map_or("".to_string(), |re| re.as_str().to_string())
        )
    }
}
