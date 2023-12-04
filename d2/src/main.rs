fn usize_from_token(tok: &str) -> Option<usize> {
    match tok {
        "0" | "zero" => Some(0),
        "1" | "one" => Some(1),
        "2" | "two" => Some(2),
        "3" | "three" => Some(3),
        "4" | "four" => Some(4),
        "5" | "five" => Some(5),
        "6" | "six" => Some(6),
        "7" | "seven" => Some(7),
        "8" | "eight" => Some(8),
        "9" | "nine" => Some(9),
        _ => None,
    }
}

fn reverse(s: &String) -> String {
    let mut rs = s.clone();
    unsafe {
        rs.as_mut_vec().reverse();
    }
    rs
}

fn find_first(s: &str, tokens: &Vec<String>) -> Option<String> {
    tokens
        .iter()
        .filter_map(|tok| s.find(tok).map(|n| (tok, n)))
        .min_by_key(|(_tok, ref n)| *n)
        .map(|(tok, _n)| tok.clone())
}

fn process(s: &str) -> Option<usize> {
    let tokens = vec![
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four",
        "five", "six", "seven", "eight", "nine",
    ]
    .iter()
    .map(|&s| String::from(s))
    .collect();

    let l = find_first(s, &tokens)?;
    let rs: String = s.chars().rev().collect();
    let rtokens = tokens.iter().map(reverse).collect();
    let r = find_first(&rs, &rtokens)?;
    let r = reverse(&r);

    let l = usize_from_token(&l);
    let r = usize_from_token(&r);

    if let (Some(l), Some(r)) = (l, r) {
        Some(10 * l + r)
    } else {
        None
    }
}

use std::fs;
use std::io::Read;

fn main() {
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
