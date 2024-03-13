use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    let mut sum = 0;

    if let Ok(lines) = read_lines("src/data/input2.txt") {
        for line in lines {
            /*FIRST  STAR */
            // let num_vec: Vec<u32> = line
            //     .unwrap()
            //     .chars()
            //     .filter_map(|x| x.to_digit(10))
            //     .collect();
            // let num = num_vec.first().unwrap() * 10 + num_vec.last().unwrap();

            /*SECOND STAR */
            let num_vec = match_string(line.unwrap());
            let num = num_vec.first().unwrap() * 10 + num_vec.last().unwrap();
            sum += num;
        }
    }

    println!("{}", sum);
}

fn match_string(s: String) -> Vec<u32> {
    let mut vektor: Vec<u32> = vec![];
    let mut str = String::new();
    let numbers = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut times_matched = false;

    let mut last_char_to_match: Option<char>;
    let mut another_char: Option<char> = None;
    for char in s.chars() {
        match char {
            '0'..='9' => {
                if let Some(num) = char.to_digit(10) {
                    vektor.push(num);
                }
                str.clear()
            }
            _ => {
                str.push(char);
                match str.as_str() {
                    "one" => {
                        vektor.push(1);
                        let last_char = str.pop().unwrap();
                        str.clear();
                        str.push(last_char);
                    }
                    "two" => {
                        vektor.push(2);
                        let last_char = str.pop().unwrap();
                        str.clear();
                        str.push(last_char);
                    }
                    "three" => {
                        vektor.push(3);
                        let last_char = str.pop().unwrap();
                        str.clear();
                        str.push(last_char);
                    }
                    "four" => {
                        vektor.push(4);
                        let last_char = str.pop().unwrap();
                        str.clear();
                        str.push(last_char);
                    }
                    "five" => {
                        vektor.push(5);
                        let last_char = str.pop().unwrap();
                        str.clear();
                        str.push(last_char);
                    }
                    "six" => {
                        vektor.push(6);
                        let last_char = str.pop().unwrap();
                        str.clear();
                        str.push(last_char);
                    }
                    "seven" => {
                        vektor.push(7);
                        let last_char = str.pop().unwrap();
                        str.clear();
                        str.push(last_char);
                    }
                    "eight" => {
                        vektor.push(8);
                        let last_char = str.pop().unwrap();
                        str.clear();
                        str.push(last_char);
                    }
                    "nine" => {
                        vektor.push(9);
                        let last_char = str.pop().unwrap();
                        str.clear();
                        str.push(last_char);
                    }
                    _ => {
                        let is_prefix = numbers.iter().any(|&x| x.starts_with(&str));

                        if is_prefix == false {
                            let mut iter_str = str.chars().rev();
                            last_char_to_match = iter_str.next();
                            if times_matched {
                                another_char = iter_str.next();
                            }
                            str.clear();

                            if another_char != None {
                                str.push(another_char.unwrap())
                            }
                            str.push(last_char_to_match.unwrap());
                        } else {
                            times_matched = true;
                        }
                    }
                }
            }
        }
    }
    println!("{:?}", vektor);
    vektor
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
