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

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn _day3() {
    println!("Day 3 - 1");
    _three_one();
}

/*
(a) 97 => 1
(z) 122 => 26
(A) 65 => 27
(Z) 90 => 52
*/
fn _three_one() {
    let file = File::open("./3/3-input").unwrap();
    let reader = BufReader::new(file);

    let priorities = reader
        .lines()
        .map(|line_result: Result<String, _>| line_result.unwrap().into_bytes())
        .map(|bytes: Vec<u8>| {
            let mut map = HashMap::new();
            for i in 0..(bytes.len() / 2) {
                let val = *bytes.get(i).unwrap() as i64;
                map.insert(val, 0);
            }
            for i in (bytes.len() / 2)..bytes.len() {
                let val = *bytes.get(i).unwrap() as i64;
                if map.contains_key(&val) {
                    return match val {
                        65..=90 => val - 38,
                        97..=122 => val - 96,
                        _ => 0,
                    };
                }
            }
            0
        })
        .sum::<i64>();
    println!("Priorities {}", priorities)
}
