use std::io::Read;
use std::str::FromStr;

fn main() -> Result<(), std::io::Error> {
    let mut input = "".into();
    std::fs::File::open("input")?.read_to_string(&mut input)?;
    let ans = puzzle(&input);
    dbg!(ans);
    Ok(())
}

/// wrap input integer to 0 - 100
fn dial_wrap(mut x: i32, r: i32, xings: &mut usize) -> i32 {
    for _ in 0..(r.abs()) {
        if r < 0 {
            x -= 1;
        } else if r > 0 {
            x += 1;
        }
        if x < 0 {
            x = x + 100;
        } else if x >= 100 {
            x = x - 100;
        }
        if x % 100 == 0 {
            *xings += 1;
        }
    }
    return x;
}

fn puzzle(input: &str) -> (usize, usize) {
    // convert input file to an array of rotation,
    // left rotation is -, right rotation is +
    let input: Vec<i32> = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| {
            // trim and split line
            let (lr, value) = s.trim().split_at(1);
            // convert value to integer
            let value = i32::from_str(value).unwrap();
            // apply sign
            match lr {
                "L" => -value,
                "R" => value,
                _ => panic!("Unknown entry: {s:?}"),
            }
        })
        .collect();

    // simulate dial rotation
    let mut zeros = 0usize;
    let mut xings = 0usize;
    // dial start at 50
    let mut dial = 50;
    for r in input {
        dial = dial_wrap(dial, r, &mut xings);
        dbg!(dial);
        if dial == 0 {
            zeros += 1;
        }
    }

    (zeros, xings)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn example() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(puzzle(input), (3, 6));
    }

    #[test]
    fn good() {
        let mut input = "".into();
        std::fs::File::open("input")
            .unwrap()
            .read_to_string(&mut input)
            .unwrap();
        assert_eq!(puzzle(&input), (1036, 6228));
    }
}
