#![cfg_attr(test, feature(assert_matches))]

use std::io::stdin;
use std::iter::Iterator;
use utf8_chars::BufReadCharsExt;

mod calculator;
mod registers;
mod stack;

fn main() -> anyhow::Result<()> {
	let mut calculator = calculator::Calculator::new();
	calculator.evaluate(&mut stdin().lock().chars().map(|c| c.map_err(|err| anyhow::anyhow!(err))))
}
