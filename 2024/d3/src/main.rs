use regex::Regex;

fn main() {
    //let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    //let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let input = std::fs::read_to_string("./input").unwrap();

    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let result: i32 = re
        .captures_iter(&input)
        .map(|m| {
            let lv: i32 = m.get(1).unwrap().as_str().parse().unwrap();
            let rv: i32 = m.get(2).unwrap().as_str().parse().unwrap();
            lv * rv
        })
        .sum();
    println!("P1 result = {result}");

    let re = Regex::new(r"(?:do\(\)|don't\(\)|mul\(([0-9]+),([0-9]+)\))").unwrap();

    let mut sum = 0;
    let mut mul_enabled = true;
    for m in re.captures_iter(&input) {
        let fm = m.get(0).unwrap().as_str();
        if fm.starts_with("do()") {
            mul_enabled = true;
        } else if fm.starts_with("don't()") {
            mul_enabled = false;
        } else {
            if mul_enabled {
                let lv: i32 = m.get(1).unwrap().as_str().parse().unwrap();
                let rv: i32 = m.get(2).unwrap().as_str().parse().unwrap();
                sum += lv * rv
            }
        }
    }
    println!("P2 result = {sum}");
}

#[cfg(test)]
mod tests {
    use super::*;
}
