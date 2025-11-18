use crate::models::types::Thought;
use chrono::{DateTime, Utc, Duration};

#[derive(Debug, Clone)]
pub enum FilterType {
    All,
    ByTag(String),
    HighRelevance(f64), // Threshold
    Recent(Duration),
    HighConfidence(f64),
    Search(String), // Text search
}

pub trait ThoughtFilter {
    fn matches(&self, thought: &Thought) -> bool;
}

impl ThoughtFilter for FilterType {
    fn matches(&self, thought: &Thought) -> bool {
        match self {
            FilterType::All => true,

            FilterType::ByTag(tag) => {
                thought.context_tags.iter()
                    .any(|t| t.to_lowercase().contains(&tag.to_lowercase()))
            }

            FilterType::HighRelevance(threshold) => {
                thought.relevance_score >= *threshold
            }

            FilterType::Recent(duration) => {
                let cutoff = Utc::now() - *duration;
                thought.timestamp >= cutoff
            }

            FilterType::HighConfidence(threshold) => {
                thought.confidence_score >= *threshold
            }

            FilterType::Search(query) => {
                let query_lower = query.to_lowercase();
                thought.content.to_lowercase().contains(&query_lower)
                    || thought.context_tags.iter()
                        .any(|tag| tag.to_lowercase().contains(&query_lower))
            }
        }
    }
}

impl FilterType {
    /// Get all available tag values from thoughts
    pub fn extract_tags(thoughts: &[Thought]) -> Vec<String> {
        use std::collections::HashSet;

        let mut tags: HashSet<String> = HashSet::new();
        for thought in thoughts {
            for tag in &thought.context_tags {
                tags.insert(tag.clone());
            }
        }

        let mut tag_vec: Vec<String> = tags.into_iter().collect();
        tag_vec.sort();
        tag_vec
    }
}
