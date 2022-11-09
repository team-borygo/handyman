use strsim::normalized_levenshtein;

use crate::bookmark::Bookmark;

pub fn score(query: &str, bookmark: &Bookmark) -> f64 {
  vec![
    normalized_levenshtein(&query.to_lowercase(), &bookmark.title.to_lowercase()),
    normalized_levenshtein(&query.to_lowercase(), &bookmark.content.to_lowercase()),
  ]
  .iter()
  .max_by(|a, b| a.partial_cmp(b).unwrap())
  .unwrap()
  .to_owned()
}
