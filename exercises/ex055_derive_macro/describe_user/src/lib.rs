//! derive マクロを「使う側」。#[derive(Describe)] を付けるだけで、
//! type_name() と field_names() が生成される。

use describe_derive::Describe;

#[derive(Describe)]
pub struct User {
    pub name: String,
    pub age: u32,
}

#[derive(Describe)]
pub struct Config {
    pub path: std::path::PathBuf,
    pub retries: u8,
    pub verbose: bool,
}

#[derive(Describe)]
pub struct Empty {}
