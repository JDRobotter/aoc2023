use std::io::Read;

fn main() -> Result<(), std::io::Error> {
    let mut input = "".into();
    std::fs::File::open("input")?.read_to_string(&mut input)?;

    dbg!(puzzle(&input, |bank| max_joltage(bank, 2)));
    dbg!(puzzle(&input, |bank| max_joltage(bank, 12)));
    Ok(())
}

fn puzzle(input: &str, max_joltage: impl Fn(&[u64]) -> u64) -> u64 {
    let banks: Vec<Vec<u64>> = input
        .trim()
        // parse each bank of batteries
        .lines()
        .map(|line| {
            // parse each battery
            line.chars()
                .map(|joltage| joltage.to_digit(10).unwrap() as u64)
                .collect()
        })
        .collect();

    banks.iter().map(|bank| max_joltage(&bank) as u64).sum()
}
fn max_joltage(bank: &[u64], n: usize) -> u64 {
    let mut start_index = 0usize;
    let mut acc = 0;
    // select which batteries to turn on
    for k in 0..n {
        // starting from start index going left to right select the highest joltage
        let (i, max) = bank[start_index..(bank.len() - n + k + 1)]
            .iter()
            .enumerate()
            // in case of equality max_by_key return the last element,
            // .rev() allow us to select the first element
            .rev()
            .max_by_key(|(_, jolt)| **jolt)
            .unwrap();
        // update start index
        start_index += i + 1;
        // accumulate joltage
        acc = 10 * acc + max;
    }
    acc
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn example1() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(puzzle(input, |bank| max_joltage(bank, 2)), 357);
    }

    #[test]
    fn example2() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(puzzle(input, |bank| max_joltage(bank, 12)), 3121910778619);
    }

    #[test]
    fn fixing_max_joltage() {
        let bank: Vec<u64> = "818181911112111"
            .chars()
            .map(|joltage| joltage.to_digit(10).unwrap() as u64)
            .collect();

        assert_eq!(max_joltage(&bank, 12), 888911112111);
    }

    #[test]
    fn good_part1() {
        let mut input = "".into();
        std::fs::File::open("input")
            .unwrap()
            .read_to_string(&mut input)
            .unwrap();
        assert_eq!(puzzle(&input, |bank| max_joltage(bank, 2)), 17095);
    }
}
