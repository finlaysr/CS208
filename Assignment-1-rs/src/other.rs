// A bunch of useful things
use core::fmt;
use std::{fmt::Display, fs, path::Path, str::FromStr};

use rand::prelude::*;
#[derive(Debug)]

// Stores what kind of benchmark is being run
pub enum BenchType {
    Reversed,
    Linear,
    Random,
}

impl BenchType {
    pub fn from_file(path: &Path) -> Self {
        fs::read_to_string(path)
            .expect("Couldn't read bench type file")
            .lines()
            .next()
            .expect("Couldn't read first line of bench type file")
            .to_string()
            .parse()
            .expect("Invalid benchmark type")
    }
}

// For converting a string to this type
#[derive(Debug)]
pub struct ParseBenchTypeError;
impl FromStr for BenchType {
    type Err = ParseBenchTypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "reversed" => Ok(BenchType::Reversed),
            "linear" => Ok(BenchType::Linear),
            "random" => Ok(BenchType::Random),
            _ => Err(ParseBenchTypeError),
        }
    }
}

// Convert this type to a string
impl Display for BenchType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BenchType::Linear => "linear",
                BenchType::Reversed => "reversed",
                BenchType::Random => "random",
            },
        )
    }
}

pub fn random_array(size: usize) -> Vec<i32> {
    let mut rng = rand::rng();
    let mut ret = vec![];
    for _ in 0..size {
        ret.push(rng.random_range(-1000..=1000));
    }
    ret
}

pub fn reversed_array(size: usize) -> Vec<i32> {
    if size == 0 {
        return vec![];
    }
    (0..size as i32).rev().collect()
}

pub fn linear_array(size: usize) -> Vec<i32> {
    if size == 0 {
        return vec![];
    }
    (0..size as i32).collect()
}
