use crate::data::Data;
use rand::{Rng, rng, seq::SliceRandom};

/// Options for the `random_zh` function.
///
/// # Fields
/// - `count` (Optional): The number of random characters to generate. Defaults to 1 if not specified.
/// - `level_range` (Optional): A range of levels (e.g., `[1, 3]`) to filter the characters.
/// - `stroke_count_range` (Optional): A range of stroke counts (e.g., `[5, 10]`) to filter the characters.
/// - `allow_duplicates` (Default: `false`): Whether duplicate characters are allowed in the result.
pub struct RandomZhOptions {
    pub count: Option<usize>,
    pub level_range: Option<(u8, u8)>,
    pub stroke_count_range: Option<(u8, u8)>,
    pub allow_duplicates: bool,
}

impl Default for RandomZhOptions {
    fn default() -> Self {
        Self {
            count: None,
            level_range: None,
            stroke_count_range: None,
            allow_duplicates: false,
        }
    }
}

/// Generates random Chinese characters based on the specified options.
///
/// # Arguments
/// - `options` (RandomZhOptions): Configuration for generating random characters.
///
/// # Returns
/// A `Vec<char>` containing the randomly generated characters.
///
/// # Examples
/// ```
/// use random_zh::{random_zh, RandomZhOptions};
///
/// // Generate 1 random characters.
/// let chars = random_zh(RandomZhOptions {
///     ..Default::default()
/// });
/// println!("{:?}", chars);
///
/// // Generate 5 random characters.
/// let chars = random_zh(RandomZhOptions {
///     count: Some(5),
///     ..Default::default()
/// });
/// println!("{:?}", chars);
///
/// // Generate characters with a specific level range.
/// let chars = random_zh(RandomZhOptions {
///     level_range: Some((1, 1)), // (1, 2), (2, 3), etc.
///     ..Default::default()
/// });
/// println!("{:?}", chars);
///
/// // Generate characters with a specific stroke count range.
/// let chars = random_zh(RandomZhOptions {
///    stroke_count_range: Some((1, 36)), // (1, 6), (10, 10), etc.
///    ..Default::default()
/// });
///
/// // Generate characters with a stroke count range and allow duplicates.
/// let chars = random_zh(RandomZhOptions {
///     count: Some(50),
///     level_range: Some((1, 3)),
///     stroke_count_range: Some((1, 36)),
///     allow_duplicates: true,
/// });
/// println!("{:?}", chars);
/// ```
pub fn random_zh(options: RandomZhOptions) -> Vec<char> {
    // Load data
    let data = Data::new();

    // Filter characters by level range
    let mut candidates: Vec<char> = if let Some((min, max)) = options.level_range {
        data.levels
            .iter()
            .filter(|&(level, _)| *level >= min && *level <= max)
            .flat_map(|(_, chars)| chars.clone())
            .collect()
    } else {
        data.levels
            .values()
            .flat_map(|chars| chars.clone())
            .collect()
    };

    // Further filter by stroke count range
    if let Some((min, max)) = options.stroke_count_range {
        candidates = candidates
            .into_iter()
            .filter(|&c| {
                data.stroke_counts
                    .iter()
                    .any(|(&strokes, chars)| strokes >= min && strokes <= max && chars.contains(&c))
            })
            .collect();
    }

    // Shuffle the candidates
    let mut rng = rng();
    candidates.shuffle(&mut rng);

    let count = options.count.unwrap_or(1);

    if options.allow_duplicates {
        // Allow duplicates: Repeat characters as needed
        let mut result = Vec::new();
        for _ in 0..count {
            if !candidates.is_empty() {
                let index = rng.random_range(0..candidates.len());
                result.push(candidates[index]);
            }
        }
        result
    } else {
        // No duplicates: Take unique characters only
        candidates.into_iter().take(count).collect()
    }
}
