#[cfg(test)]
mod tests {
    use random_zh::{RandomZhOptions, random_zh};

    #[test]
    fn test_random_zh_default() {
        // Default options should return 1 random character
        let chars = random_zh(RandomZhOptions::default());
        assert_eq!(chars.len(), 1);
    }

    #[test]
    fn test_random_zh_with_count() {
        // Generate exactly 10 characters
        let count = 10;
        let chars = random_zh(RandomZhOptions {
            count: Some(count),
            ..Default::default()
        });
        assert_eq!(chars.len(), count);
    }

    #[test]
    fn test_random_zh_allow_duplicates() {
        // Generate 50 characters allowing duplicates
        let count = 50;
        let chars = random_zh(RandomZhOptions {
            count: Some(count),
            stroke_count_range: Some((1, 1)),
            allow_duplicates: true,
            ..Default::default()
        });
        assert_eq!(chars.len(), count);
    }
    #[test]
    fn test_random_zh_disallow_duplicates() {
        // Generate 50 characters without allowing duplicates
        let count = 50;
        let chars = random_zh(RandomZhOptions {
            count: Some(count),
            stroke_count_range: Some((1, 1)),
            allow_duplicates: false,
            ..Default::default()
        });
        assert!(chars.len() < count);
    }

    #[test]
    fn test_empty_candidates() {
        // When no characters match the filters, result should be empty
        let chars = random_zh(RandomZhOptions {
            level_range: Some((99, 100)),        // Non-existent levels
            stroke_count_range: Some((99, 100)), // Non-existent stroke counts
            count: Some(10),
            ..Default::default()
        });
        assert!(chars.is_empty());
    }
}
