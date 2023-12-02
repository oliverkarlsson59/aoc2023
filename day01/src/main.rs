use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let ints = get_integers(false);
    let part_a: u32 = ints.iter().map(|r| r.first().unwrap()*10 + r.last().unwrap()).sum();
    println!("Part A: {:?}", part_a);

    let ints_2 = get_integers(true);
    let part_b: u32 = ints_2.iter().map(|r| r.first().unwrap()*10 + r.last().unwrap()).sum();
    println!("Part B: {:?}", part_b);
}

fn get_integers(check_letters: bool) -> Vec<Vec<u32>> {
    let mut _nums: Vec<Vec<u32>> = Vec::new();
    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines {
            let _in_vec: Vec<u32> = Vec::new();
            if let Ok(mut str) = line {
                if check_letters {
                    replace_letters(&mut str);
                }
                let v: Vec<u32> = str.chars().filter(|c| c.is_numeric()).map(|c| c.to_digit(10).unwrap()).collect();
                _nums.push(v);
            }
        }   
    }
    _nums
}

fn replace_letters(str: &mut String) {
    let nums = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3","4","5","6","7","8","9"];
    let val = [1,2,3,4,5,6,7,8,9,1,2,3,4,5,6,7,8,9];
    let mut first = (usize::MAX, String::from(""));
    let mut last = (usize::MIN, String::from(""));

    for num in nums {
        if let Some(index) = str.find(num) {
            if index <= first.0 {
                first = (index, num.to_string());
            }
        }
        if let Some(index) = str.rfind(num) {
            if index >= last.0 {
                last = (index, num.to_string());
            }
        }
    }

    if let Some(idx) = &nums.iter().position(|&r| r == first.1) {
        *str = str.replace(&first.1, &val[*idx].to_string());
    }

    if let Some(idy) = &nums.iter().position(|&r| r == last.1) {
        *str = str.replace(&last.1, &val[*idy].to_string());
    }
    
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>where P:AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
    
