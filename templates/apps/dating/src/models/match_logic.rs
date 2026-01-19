use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    pub id: u32,
    pub name: String,
    pub age: u32,
    pub interests: Vec<String>,
    pub location: (f64, f64), // lat, long
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchScore {
    pub profile_id: u32,
    pub score: f32,
}

pub struct Matchmaker;

impl Matchmaker {
    /// Calculate match score between two profiles
    /// Score based on:
    /// - Common interests (10 points each)
    /// - Age gap (deduct 1 point per year diff)
    /// - Distance (deduct 0.5 per km approximation)
    pub fn calculate_score(a: &Profile, b: &Profile) -> f32 {
        let mut score = 0.0;

        // Interest Overlap
        for interest in &a.interests {
            if b.interests.contains(interest) {
                score += 10.0;
            }
        }

        // Age Gap
        let age_diff = (a.age as i32 - b.age as i32).abs();
        score -= age_diff as f32;

        // Simple distance approx (Euclidean for demo)
        let dist =
            ((a.location.0 - b.location.0).powi(2) + (a.location.1 - b.location.1).powi(2)).sqrt();
        score -= dist as f32 * 10.0; // Rough penalty

        score.max(0.0) // Minimum score 0
    }

    pub fn find_matches(target: &Profile, candidates: &[Profile]) -> Vec<MatchScore> {
        let mut matches: Vec<MatchScore> = candidates
            .iter()
            .filter(|p| p.id != target.id)
            .map(|p| MatchScore {
                profile_id: p.id,
                score: Self::calculate_score(target, p),
            })
            .collect();

        matches.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        matches
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_match() {
        let p1 = Profile {
            id: 1,
            name: "A".into(),
            age: 25,
            interests: vec!["coding".into(), "coffee".into()],
            location: (0.0, 0.0),
        };
        let p2 = p1.clone();

        // Self match (logic validation, though id check prevents self-match in list)
        // Score: 20 (interests) - 0 (age) - 0 (dist) = 20
        let score = Matchmaker::calculate_score(&p1, &p2);
        assert_eq!(score, 20.0);
    }

    #[test]
    fn test_ranking() {
        let me = Profile {
            id: 1,
            name: "Me".into(),
            age: 25,
            interests: vec!["coding".into()],
            location: (0.0, 0.0),
        };

        let good = Profile {
            id: 2,
            name: "Good".into(),
            age: 26,
            interests: vec!["coding".into()],
            location: (0.1, 0.1), // Close
        };

        let bad = Profile {
            id: 3,
            name: "Bad".into(),
            age: 50,
            interests: vec!["knitting".into()],
            location: (10.0, 10.0), // Far
        };

        let candidates = vec![good.clone(), bad.clone()];
        let matches = Matchmaker::find_matches(&me, &candidates);

        assert_eq!(matches[0].profile_id, 2);
        assert_eq!(matches[1].profile_id, 3);
        assert!(matches[0].score > matches[1].score);
    }
}
