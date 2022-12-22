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

pub fn _day1() {
    println!("Day 1 - 1");
    _one_one();

    println!("Day 1 - 2");
    _one_two();
}

fn _one_one() {
    let file = File::open("./1/1-input").unwrap();
    let reader = BufReader::new(file);

    let max_calories = reader
        .lines()
        .map(|line_result: Result<String, _>| match line_result {
            Ok(line) => line.parse::<i64>(),
            Err(_) => Ok(-1),
        })
        .map(|parsed_result: Result<i64, _>| parsed_result.unwrap_or(-1))
        .max()
        .unwrap();
    println!("max cals {}", max_calories);
}

fn _one_two() {
    let mut calories_sum = Vec::new();
    let file = File::open("./1/1-input").unwrap();
    let reader = BufReader::new(file);

    let mut calories = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        if line.eq("") {
            calories_sum.push(calories);
            calories = 0;
        } else {
            calories += line.parse::<i64>().unwrap();
        }
    }

    calories_sum.push(calories);

    calories_sum.sort();
    calories_sum.reverse();
    let mut sum = 0;
    sum += calories_sum[0];
    sum += calories_sum[1];
    sum += calories_sum[2];
    println!("top three sum {}", sum)
}
