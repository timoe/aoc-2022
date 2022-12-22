// Copyright 2022 Timo E aus E
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
// of the Software, and to permit persons to whom the Software is furnished to do
// so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Draw {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
struct Round(Draw, Draw);

impl Round {
    fn play(&self) -> i32 {
        match self.0 {
            Draw::Rock => match self.1 {
                Draw::Rock => 1 + 3,
                Draw::Paper => 2 + 6,
                Draw::Scissors => 3,
            },
            Draw::Paper => match self.1 {
                Draw::Rock => 1,
                Draw::Paper => 2 + 3,
                Draw::Scissors => 3 + 6,
            },
            Draw::Scissors => match self.1 {
                Draw::Rock => 1 + 6,
                Draw::Paper => 2,
                Draw::Scissors => 3 + 3,
            },
        }
    }
}

pub fn _day2() {
    println!("Day 2 - 1");
    two_one();
}

fn two_one() {
    let file = File::open("./2/2-input").unwrap();
    let reader = BufReader::new(file);

    let rounds: Vec<Round> = reader
        .lines()
        .map(|line_result: Result<String, _>| {
            let line = line_result.unwrap();
            let b = line.as_bytes();
            Round(as_draw(b[0]).unwrap(), as_draw(b[2]).unwrap())
        })
        .collect();

    let points: i32 = rounds.into_iter().map(|round: Round| round.play()).sum();
    println!("Points {}", points);
}

/*
  A for Rock,
  B for Paper,
  C for Scissors

  X for Rock,
  Y for Paper,
  Z for Scissors
*/
fn as_draw(_draw: u8) -> Result<Draw, String> {
    match _draw {
        65 | 88 => Ok(Draw::Rock),
        66 | 89 => Ok(Draw::Paper),
        67 | 90 => Ok(Draw::Scissors),
        _ => Err(String::from("`name` was empty; it must be nonempty.")),
    }
}
