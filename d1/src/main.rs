fn process(s: &str) -> Option<usize> {
    let l = s.chars().filter(|&c| char::is_numeric(c)).next()?;
    let r = s.chars().rev().filter(|&c| char::is_numeric(c)).next()?;

    let mut number = String::from(l);
    number.push(r);

    usize::from_str_radix(&number, 10).ok()
}

use std::fs;
use std::io::Read;

fn main() {
    //let lines = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];

    let mut file = fs::File::open("input").unwrap();

    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    let lines: Vec<&str> = s.split('\n').collect();

    let mut sum = 0;
    for line in lines {
        let rv = process(line);
        println!("{} => {:?}", line, rv);
        if let Some(rv) = rv {
            sum += rv;
        }
    }

    println!("sum = {sum}");
}
