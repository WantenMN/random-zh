#![cfg_attr(not(doctest),doc = include_str!("../README.md"))]

mod data;
mod random_zh;

pub use random_zh::{RandomZhOptions, random_zh};
