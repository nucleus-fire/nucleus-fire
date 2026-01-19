use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sonar {
    // word -> list of (doc_id, term_frequency)
    index: HashMap<String, Vec<(String, usize)>>,
    // doc_id -> doc_length
    doc_lengths: HashMap<String, usize>,
    avg_doc_length: f64,
    total_docs: usize,
}

impl Sonar {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
            doc_lengths: HashMap::new(),
            avg_doc_length: 0.0,
            total_docs: 0,
        }
    }
}

impl Default for Sonar {
    fn default() -> Self {
        Self::new()
    }
}

impl Sonar {
    pub fn index(&mut self, id: &str, content: &str) {
        self.index_document(Document {
            id: id.to_string(),
            content: content.to_string(),
        });
    }

    pub fn remove(&mut self, id: &str) {
        // Simple invalidation (in a real system, would need more complex cleanup)
        // For now, we effectively "remove" by resetting its length to 0 which kills score?
        // No, full removal is expensive in inverted index.
        // We'll implemented basic removal from doc_lengths which invalidates it in search logic if we check.
        self.doc_lengths.remove(id);
        // Note: tokens remain in index but won't be score-able if we add a check.
    }

    pub fn index_document(&mut self, doc: Document) {
        let tokens = self.tokenize(&doc.content);
        let doc_len = tokens.len();

        let mut term_freqs = HashMap::new();
        for token in tokens {
            *term_freqs.entry(token).or_insert(0) += 1;
        }

        for (term, count) in term_freqs {
            self.index
                .entry(term)
                .or_default()
                .push((doc.id.clone(), count));
        }

        self.doc_lengths.insert(doc.id, doc_len);
        self.total_docs += 1;

        // Update avg length
        let total_len: usize = self.doc_lengths.values().sum();
        if self.total_docs > 0 {
            self.avg_doc_length = total_len as f64 / self.total_docs as f64;
        }
    }

    fn tokenize(&self, text: &str) -> Vec<String> {
        text.to_lowercase()
            .split_whitespace()
            .map(|s| s.replace(|c: char| !c.is_alphanumeric(), ""))
            .filter(|s| !s.is_empty())
            .collect()
    }

    pub fn search(&self, query: &str) -> Vec<SearchResult> {
        self.search_with_limit(query, 100)
    }

    pub fn search_with_limit(&self, query: &str, limit: usize) -> Vec<SearchResult> {
        let tokens = self.tokenize(query);
        let mut scores: HashMap<String, f64> = HashMap::new();

        // BM25 Constants
        let k1 = 1.2;
        let b = 0.75;

        for term in tokens {
            if let Some(postings) = self.index.get(&term) {
                // IDF
                let doc_freq = postings.len();
                let idf = ((self.total_docs as f64 - doc_freq as f64 + 0.5)
                    / (doc_freq as f64 + 0.5)
                    + 1.0)
                    .ln();

                for (doc_id, tf) in postings {
                    // Ensure doc still exists
                    if let Some(&doc_len) = self.doc_lengths.get(doc_id) {
                        let tf_float = *tf as f64;

                        let num = tf_float * (k1 + 1.0);
                        let den =
                            tf_float + k1 * (1.0 - b + b * (doc_len as f64 / self.avg_doc_length));

                        *scores.entry(doc_id.clone()).or_insert(0.0) += idf * (num / den);
                    }
                }
            }
        }

        let mut results: Vec<SearchResult> = scores
            .into_iter()
            .map(|(id, score)| SearchResult { id, score })
            .collect();

        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        results.truncate(limit);
        results
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub score: f64,
}

pub type InvertedIndex = Sonar; // Type alias for compatibility

pub struct Polyglot;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bm25_ranking() {
        let mut index = InvertedIndex::new();

        index.index_document(Document {
            id: "1".to_string(),
            content: "Rust is a systems programming language".to_string(),
        });
        index.index_document(Document {
            id: "2".to_string(),
            content: "Python is a scripting language".to_string(),
        });
        index.index_document(Document {
            id: "3".to_string(),
            content: "Rust Rust Rust".to_string(),
        });

        // Query: "Rust"
        // Doc 3 should rank highest (highest TF), then Doc 1, Doc 2 not at all.
        let results = index.search("Rust");

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].id, "3"); // "Rust Rust Rust" should win
        assert_eq!(results[1].id, "1");
    }
}
