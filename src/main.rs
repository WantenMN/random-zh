use clap::Parser;
use random_zh::{RandomZhOptions, random_zh};

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// A command-line tool to generate random Chinese characters based on various criteria
struct Cli {
    /// Number of characters to generate
    #[arg(short, long)]
    count: Option<usize>,

    /// Level range for characters (e.g., 1,3 for levels 1 to 3 and 1,1 for level 1 only)
    #[arg(short, long, value_name = "LEVEL_RANGE")]
    level_range: Option<String>,

    /// Stroke count range for characters (e.g., 1,36 for 1 to 36 strokes, 1,1 for 1 stroke only)
    #[arg(short, long, value_name = "STROKE_COUNT_RANGE")]
    stroke_count_range: Option<String>,

    /// Allow duplicate characters; when not enabled, if `count` exceeds the total number of unique characters, only the total number of unique characters will be returned instead of `count`
    #[arg(short = 'd', long)]
    allow_duplicates: bool,
}

fn main() {
    let cli = Cli::parse();

    // Parse level range if provided
    let level_range = cli.level_range.as_deref().and_then(|lr| {
        let parts: Vec<&str> = lr.split(',').collect();
        if parts.len() == 2 {
            Some((parts[0].parse::<u8>().ok()?, parts[1].parse::<u8>().ok()?))
        } else {
            None
        }
    });

    // Parse stroke count range if provided
    let stroke_count_range = cli.stroke_count_range.as_deref().and_then(|scr| {
        let parts: Vec<&str> = scr.split(',').collect();
        if parts.len() == 2 {
            Some((parts[0].parse::<u8>().ok()?, parts[1].parse::<u8>().ok()?))
        } else {
            None
        }
    });

    // Create options for random_zh
    let options = RandomZhOptions {
        count: cli.count,
        level_range,
        stroke_count_range,
        allow_duplicates: cli.allow_duplicates,
    };

    // Generate random characters
    let characters = random_zh(options);
    println!("{:?}", characters);
}
