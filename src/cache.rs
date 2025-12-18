//! Team code mapping cache for cross-platform market matching.
//!
//! This module provides bidirectional mapping between Polymarket and Kalshi
//! team codes to enable accurate market discovery across platforms.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

const CACHE_FILE: &str = "kalshi_team_cache.json";

/// Team code cache - bidirectional mapping between Poly and Kalshi team codes
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TeamCache {
    /// Forward: "league:poly_code" -> "kalshi_code"
    #[serde(serialize_with = "serialize_boxed_map", deserialize_with = "deserialize_boxed_map")]
    forward: HashMap<Box<str>, Box<str>>,
    /// Reverse: "league:kalshi_code" -> "poly_code"
    #[serde(skip)]
    reverse: HashMap<Box<str>, Box<str>>,
}

fn serialize_boxed_map<S>(map: &HashMap<Box<str>, Box<str>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeMap;
    let mut ser_map = serializer.serialize_map(Some(map.len()))?;
    for (k, v) in map {
        ser_map.serialize_entry(k.as_ref(), v.as_ref())?;
    }
    ser_map.end()
}

fn deserialize_boxed_map<'de, D>(deserializer: D) -> Result<HashMap<Box<str>, Box<str>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let string_map: HashMap<String, String> = HashMap::deserialize(deserializer)?;
    Ok(string_map
        .into_iter()
        .map(|(k, v)| (k.into_boxed_str(), v.into_boxed_str()))
        .collect())
}

#[allow(dead_code)]
impl TeamCache {
    /// Load cache from JSON file
    pub fn load() -> Self {
        Self::load_from(CACHE_FILE)
    }

    /// Load from specific path
    pub fn load_from<P: AsRef<Path>>(path: P) -> Self {
        let mut cache = match std::fs::read_to_string(path.as_ref()) {
            Ok(contents) => {
                serde_json::from_str(&contents).unwrap_or_else(|e| {
                    tracing::warn!("Failed to parse team cache: {}", e);
                    Self::default()
                })
            }
            Err(_) => {
                tracing::info!("No team cache found at {:?}, starting empty", path.as_ref());
                Self::default()
            }
        };
        cache.rebuild_reverse();
        cache
    }

    /// Save cache to JSON file
    pub fn save(&self) -> Result<()> {
        self.save_to(CACHE_FILE)
    }

    /// Save to specific path
    pub fn save_to<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let json = serde_json::to_string_pretty(&self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Get Kalshi code for a Polymarket team code
    /// e.g., ("epl", "che") -> "cfc"
    pub fn poly_to_kalshi(&self, league: &str, poly_code: &str) -> Option<String> {
        let mut key_buf = String::with_capacity(league.len() + 1 + poly_code.len());
        key_buf.push_str(&league.to_ascii_lowercase());
        key_buf.push(':');
        key_buf.push_str(&poly_code.to_ascii_lowercase());
        self.forward.get(key_buf.as_str()).map(|s| s.to_string())
    }

    /// Get Polymarket code for a Kalshi team code (reverse lookup)
    /// e.g., ("epl", "cfc") -> "che"
    pub fn kalshi_to_poly(&self, league: &str, kalshi_code: &str) -> Option<String> {
        let mut key_buf = String::with_capacity(league.len() + 1 + kalshi_code.len());
        key_buf.push_str(&league.to_ascii_lowercase());
        key_buf.push(':');
        key_buf.push_str(&kalshi_code.to_ascii_lowercase());

        self.reverse
            .get(key_buf.as_str())
            .map(|s| s.to_string())
            .or_else(|| Some(kalshi_code.to_ascii_lowercase()))
    }

    /// Add or update a mapping
    pub fn insert(&mut self, league: &str, poly_code: &str, kalshi_code: &str) {
        let league_lower = league.to_ascii_lowercase();
        let poly_lower = poly_code.to_ascii_lowercase();
        let kalshi_lower = kalshi_code.to_ascii_lowercase();

        let forward_key: Box<str> = format!("{}:{}", league_lower, poly_lower).into();
        let reverse_key: Box<str> = format!("{}:{}", league_lower, kalshi_lower).into();

        self.forward.insert(forward_key, kalshi_lower.into());
        self.reverse.insert(reverse_key, poly_lower.into());
    }

    /// Number of mappings
    pub fn len(&self) -> usize {
        self.forward.len()
    }

    pub fn is_empty(&self) -> bool {
        self.forward.is_empty()
    }

    /// Rebuild reverse lookup map from forward mappings
    fn rebuild_reverse(&mut self) {
        self.reverse.clear();
        self.reverse.reserve(self.forward.len());
        for (key, kalshi_code) in &self.forward {
            if let Some((league, poly)) = key.split_once(':') {
                let reverse_key: Box<str> = format!("{}:{}", league, kalshi_code).into();
                self.reverse.insert(reverse_key, poly.into());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cache_lookup() {
        let mut cache = TeamCache::default();
        cache.insert("epl", "che", "cfc");
        cache.insert("epl", "mun", "mun");
        
        assert_eq!(cache.poly_to_kalshi("epl", "che"), Some("cfc".to_string()));
        assert_eq!(cache.poly_to_kalshi("epl", "CHE"), Some("cfc".to_string()));
        assert_eq!(cache.kalshi_to_poly("epl", "cfc"), Some("che".to_string()));
    }
}
