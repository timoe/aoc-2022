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

fn main() {
    one_one();
}

fn one_one() {
    let file = File::open("./1/1-input").unwrap();
    let reader = BufReader::new(file);

    let mut calories = 0;
    let mut max_calories = 0;
    let mut elve = 1;
    let mut max_elve = 1;
    for line in reader.lines() {
        let line = line.unwrap();

        if line.eq("") {
            if calories > max_calories {
                max_calories = calories;
                max_elve = elve;
            }
            calories = 0;
            elve = elve + 1;
        } else {
            calories += line.parse::<i64>().unwrap();
        }
    }

    println!("max cals {}, max elve {}", max_calories, max_elve)
}