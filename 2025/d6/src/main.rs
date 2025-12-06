use std::io::Read;

use utils::asciimap::AsciiMap;

fn main() -> Result<(), std::io::Error> {
    let mut input = "".into();
    std::fs::File::open("input")?.read_to_string(&mut input)?;
    puzzle1(&input);
    puzzle2(&input);
    Ok(())
}

fn parse_input_p1(input: &str) -> Vec<(Vec<i64>, char)> {
    let mut lines: Vec<&str> = input.trim().split('\n').collect();

    // pop symbols line
    let symbols = lines.pop().unwrap();
    let symbols: Vec<char> = symbols
        .split_ascii_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect();

    // parse numbers
    let numbers: Vec<Vec<i64>> = lines
        .into_iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    let numbers = utils::arrays::transpose(numbers);

    std::iter::zip(numbers, symbols).collect()
}

fn parse_input_p2(input: &str) -> Vec<(Vec<i64>, char)> {
    // trim trailing new line
    let input = input.trim_matches('\n');
    // load as map
    let map = AsciiMap::from_multi_lines_or(input, ' ');

    let h = map.height();
    //123 328  51 64
    // 45 64  387 23
    //  6 98  215 314
    //*   +   *   +

    // we should have an op in bottom left corner of map
    let mut op = *map.get(0, h - 1).unwrap();
    assert!(matches!(op, '+' | '*'), "h={h} op={op:?}");
    let mut xop = 0;
    let mut xnop;
    let mut x = 1;

    let mut operations = vec![];
    loop {
        // ----
        // continue marching right until we find another op
        let next_op;
        loop {
            match map.get(x, h - 1) {
                Some('+') => {
                    next_op = '+';
                    break;
                }
                Some('*') => {
                    next_op = '*';
                    break;
                }
                Some(' ') => {}
                None => {
                    // end of map reached
                    next_op = 'E';
                    break;
                }
                e => panic!("{e:?}"),
            }
            x += 1;
        }

        if next_op == 'E' {
            // xnop should point on next operation position
            // when we reach end of map next operation is two chars away from border
            x += 1;
        }
        xnop = x;

        let mut nums = vec![];
        // starting from next op position walk upward to read numbers
        x -= 1;
        while x != xop {
            // go backward
            x -= 1;
            // collect numbers into a string
            let num: String = (0..(h - 1)).map(|y| map.get(x, y).unwrap()).collect();
            // trim trailing and leading whitespaces
            let num = num.trim();
            // convert num to i64
            let num: i64 = num.parse().unwrap();
            nums.push(num);
        }

        operations.push((nums, op));

        // switch to next operation
        op = next_op;
        xop = xnop;
        x = xnop + 1;
        if op == 'E' {
            break;
        }
    }

    operations
}

fn do_op(op: char, x: i64, acc: Option<i64>) -> Option<i64> {
    match op {
        '+' => {
            let acc = acc.unwrap_or(0);
            Some(acc + x)
        }
        '*' => {
            let acc = acc.unwrap_or(1);
            Some(acc * x)
        }
        _ => panic!("unhandled op"),
    }
}

fn do_ops(ops: &[(Vec<i64>, char)]) -> i64 {
    let mut sum = 0;
    // perform operations
    for (nums, op) in ops {
        let mut acc = None;
        for x in nums {
            acc = do_op(*op, *x, acc);
        }
        sum += acc.unwrap_or(0);
    }

    sum
}

fn puzzle1(input: &str) -> u64 {
    let operations = parse_input_p1(input);
    do_ops(&operations) as u64
}

fn puzzle2(input: &str) -> u64 {
    let operations = parse_input_p2(input);
    do_ops(&operations) as u64
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn example1() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   + ";
        assert_eq!(puzzle1(input), 4277556);
    }

    #[test]
    fn example2() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   + ";
        assert_eq!(puzzle2(input), 3263827);
    }

    #[test]
    fn good_part1() {
        let mut input = "".into();
        std::fs::File::open("input")
            .unwrap()
            .read_to_string(&mut input)
            .unwrap();
        assert_eq!(puzzle1(&input), 6209956042374);
    }

    #[test]
    fn good_part2() {
        let mut input = "".into();
        std::fs::File::open("input")
            .unwrap()
            .read_to_string(&mut input)
            .unwrap();
        assert_eq!(puzzle2(&input), 12608160008022);
    }
}
