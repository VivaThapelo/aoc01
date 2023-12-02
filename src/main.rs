use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("../input.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open file {}: {}", display, why),
        Ok(file) => file,
    };

    let mut contents = String::new();
    contents = match file.read_to_string(&mut contents) {
        Err(why) => panic!("couldn't read file {}: {}", display, why),
        Ok(_) => contents,
    };

    let lines = contents
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut sum = 0;
    for line in lines {
        // split line into charaters 'chars'
        let chars = line.chars().collect::<Vec<char>>();
        let digits = get_nums(chars);
        if digits.len() == 1 {
            sum = sum + digits[0];
        } else if digits.len() > 1 {
            sum = sum + digits[0] + digits.last().unwrap();
        }
    }
    println!("{}", sum);
}

fn get_nums(mut chars: Vec<char>) -> Vec<i32> {
    let mut is_last_num =  false;
    let mut nums: Vec<i32> = Vec::new();
    let mut numb = String::new();
    chars.push(' ');
    for char in chars {
        if char.is_numeric() {
            numb.push(char);
            is_last_num = true;
        } else {
            if is_last_num && char != ' ' {
                nums.push(numb.parse::<i32>().unwrap_or_else(|e| {println!("error: {}", e); 0}));
            }
                numb = "".to_string();
                is_last_num = false;
        }
    }
    println!("X nums: {:?}", nums);
    nums
}