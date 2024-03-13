use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    let mut sum = 0;
    let mut map: HashMap<&str, i32> = HashMap::new();

    if let Ok(lines) = read_lines("data/input.txt") {
        for line in lines {
            let string = line.unwrap().clone();
            let str = string.split_once(':').unwrap();

            let mut map: HashMap<&str, i32> = HashMap::new();

            str.1
                .split(';')
                .map(|s| s.trim())
                .flat_map(|s| s.split(','))
                .for_each(|s| {
                    let (n, color_name) = s.trim().split_once(' ').unwrap();
                    let num = n.parse::<i32>().unwrap();
                    map.entry(color_name)
                        .and_modify(|current_value| {
                            if num > *current_value {
                                *current_value = num;
                            }
                        })
                        .or_insert(num);
                    println!("{:?}", map);
                });

            let product = map.values().fold(1, |acc, x| acc * x);

            sum += product;
        }
    }

    println!("{}", sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
