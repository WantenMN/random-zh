# random-zh

A Rust crate to generate random Chinese characters based on various criteria.

## Links

- [Crates.io](https://crates.io/crates/random-zh)
- [Code](https://github.com/WantenMN/random-zh)

## Installation

Add `random-zh` to your `Cargo.toml`:

```toml
[dependencies]
random-zh = "0.1.2"
```

Run the following command to fetch the crate:

```bash
cargo build
```

## Usage

```rust
use random_zh::{random_zh, RandomZhOptions};

// Get a random character
let random_character = random_zh(RandomZhOptions::default());

// Get 10 random characters
let ten_random_characters = random_zh(RandomZhOptions {
    count: Some(10),
    ..Default::default()
});

// Get characters within a level range
let characters_in_level_range = random_zh(RandomZhOptions {
    level_range: Some((1, 3)),
    ..Default::default()
});

// Get characters within a stroke count range
let characters_in_stroke_count_range = random_zh(RandomZhOptions {
    stroke_count_range: Some((5, 10)),
    ..Default::default()
});

// Get characters within a level range and stroke count range
let characters_within_ranges = random_zh(RandomZhOptions {
    level_range: Some((1, 3)),
    stroke_count_range: Some((5, 10)),
    ..Default::default()
});

// Allow duplicates
let characters_with_duplicates = random_zh(RandomZhOptions {
    count: Some(20),
    allow_duplicates: true,
    ..Default::default()
});
```

## API

### `random_zh(options: RandomZhOptions) -> Vec<char>`

Generates a vector of random Chinese characters based on the provided options.

#### `RandomZhOptions`

- `count: Option<usize>`: The number of random characters to generate. Defaults to 1 if not specified.
- `level_range: Option<(u8, u8)>`: The range of levels to include in the generated characters. Levels currently range from 1 to 3.
- `stroke_count_range: Option<(u8, u8)>`: The range of stroke counts to include in the generated characters. Stroke counts currently range from 1 to 36.
- `allow_duplicates: bool`: Whether to allow duplicate characters in the output. Defaults to `false`, if `count` exceeds the total number of unique characters, only the total number of unique characters will be returned instead of `count`

### Example: Generating Characters

```rust
let options = RandomZhOptions {
    count: Some(5),
    level_range: Some((1, 2)),
    stroke_count_range: Some((8, 12)),
    allow_duplicates: false,
};

let characters = random_zh(options);
println!("Random characters: {:?}", characters);
```

## CLI Usage

The random-zh binary allows generating random Chinese characters from the command line.

### Install from Cargo

```sh
cargo install random-zh
```

### Command Line Options

```
Usage: random-zh [OPTIONS]

Options:
  -c, --count <COUNT>
          Number of characters to generate
  -l, --level-range <LEVEL_RANGE>
          Level range for characters (e.g., 1,3 for levels 1 to 3, 1,1 for level 1 only)
  -s, --stroke-count-range <STROKE_COUNT_RANGE>
          Stroke count range for characters (e.g., 1,36 for 1 to 36 strokes, 1,1 for 1 stroke only)
  -d, --allow-duplicates
          Allow duplicate characters
  -h, --help
          Print help
  -V, --version
          Print version
```

### Examples

```sh
random-zh
# ['天']

random-zh -c 10
# ['稠', '𬭤', '霪', '悸', '垸', '腑', '姒', '运', '迨', '霭']

random-zh -c 10 -l 1,1
#['茎', '雷', '除', '暑', '颗', '目', '厉', '举', '槛', '瓶']

random-zh -c 10 -l 1,1 -s 1,1
# ['乙', '一']

random-zh -c 10 -l 1,1 -s 1,1 -d
# ['乙', '一', '一', '一', '一', '乙', '乙', '一', '一', '一']
```

## Level Descriptions

- **Level 1 (一级字表)**: Consists of 3,500 commonly used characters. These characters meet the basic needs of education and cultural dissemination.
- **Level 2 (二级字表)**: Contains 3,000 characters, slightly less common than Level 1. Together, Level 1 and Level 2 cover general usage in publishing, dictionary compilation, and information processing.
- **Level 3 (三级字表)**: Includes 1,605 characters for specialized use, including surnames, place names, technical terms, and less common words. These characters are particularly useful in areas related to public life and information processing.

## License

This project is licensed under the [MIT License](https://raw.githubusercontent.com/WantenMN/random-zh/main/LICENSE).
