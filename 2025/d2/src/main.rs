use std::io::Read;

fn main() -> Result<(), std::io::Error> {
    let mut input = "".into();
    std::fs::File::open("input")?.read_to_string(&mut input)?;
    let ans1 = puzzle(&input, validate1);
    dbg!(ans1);
    let ans2 = puzzle(&input, validate2);
    dbg!(ans2);
    Ok(())
}

fn puzzle(input: &str, validate: impl Fn(&str) -> bool) -> usize {
    input
        .trim()
        .split(',')
        .filter_map(|range| {
            let (a, b) = range.split_once('-').unwrap();
            let a: usize = a.parse().unwrap();
            let b: usize = b.parse().unwrap();

            let mut sum: usize = 0;

            // check all IDs in range
            for id in a..=b {
                if !validate(&format!("{}", id)) {
                    sum += id;
                }
            }

            Some(sum)
        })
        .sum()
}

/// Return true if given ID is valid, false otherwise
fn validate1(s: &str) -> bool {
    // an invalid ID is an string repeated twice
    // so string len as to be a multiple of 2 for an ID to be invalid
    if s.len() % 2 != 0 {
        return true;
    }

    // check if left and right parts are equals
    let mid = s.len() / 2;
    let (a, b) = s.split_at(mid);
    a != b
}

fn validate2(s: &str) -> bool {
    // biggest repeated chunk in ID is no big than len/2
    let max_chunk_len = s.len() / 2;

    // check each chunk length for repeated pattern
    for n in 1..=max_chunk_len {
        let s = s.as_bytes();
        let mut chunks = s.chunks(n);
        let base = chunks.next().unwrap();
        // check if all other elements match base
        if chunks.all(|x| x == base) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn example1() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(puzzle(input, validate1), 1227775554);
    }

    #[test]
    fn example2() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(puzzle(input, validate2), 4174379265);
    }

    #[test]
    fn test_validate1_true() {
        assert_eq!(validate1("453"), true);
        assert_eq!(validate1("123124"), true);
        assert_eq!(validate1("101"), true);
        assert_eq!(validate1("1111115"), true);
    }

    #[test]
    fn test_validate1_false() {
        assert_eq!(validate1("11"), false);
        assert_eq!(validate1("22"), false);
        assert_eq!(validate1("1212"), false);
    }

    #[test]
    fn test_validate2_false() {
        assert_eq!(validate2("11"), false);
        assert_eq!(validate2("22"), false);
        assert_eq!(validate2("99"), false);
        assert_eq!(validate2("111"), false);
        assert_eq!(validate2("999"), false);
        assert_eq!(validate2("1010"), false);
        assert_eq!(validate2("18851885"), false);
        assert_eq!(validate2("446446"), false);
    }

    #[test]
    fn test_validate2_true() {
        assert_eq!(validate2("123"), true);
        assert_eq!(validate2("212120"), true);
        assert_eq!(validate2("55556"), true);
        assert_eq!(validate2("111111115"), true);
    }

    #[test]
    fn good_part1() {
        let mut input = "".into();
        std::fs::File::open("input")
            .unwrap()
            .read_to_string(&mut input)
            .unwrap();
        assert_eq!(puzzle(&input, validate1), 34826702005);
    }
}
