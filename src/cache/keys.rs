/// Cache key for a creator profile. TTL: 5 minutes.
pub fn creator(username: &str) -> String {
    format!("creator:{}", username)
}

/// Cache key for a creator's tip list. TTL: 1 minute.
pub fn creator_tips(username: &str) -> String {
    format!("creator:{}:tips", username)
}
