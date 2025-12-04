fn test_all(v: &Vec<i32>, f: impl Fn(i32, i32) -> bool) -> bool {
    for idx in 0..v.len() - 1 {
        if !f(v[idx], v[idx + 1]) {
            return false;
        }
    }
    true
}

fn all_increasing(v: &Vec<i32>) -> bool {
    test_all(v, |a, b| {
        let x = b - a;
        1 <= x && x <= 3
    })
}

fn all_decreasing(v: &Vec<i32>) -> bool {
    test_all(v, |a, b| {
        let x = b - a;
        -3 <= x && x <= -1
    })
}

fn check_allow_one_bad(v: &Vec<i32>, test: impl Fn(&Vec<i32>) -> bool) -> bool {
    // try full report, if safe return immediately
    if test(v) {
        return true;
    }

    for (idx, _) in v.iter().enumerate() {
        // clone report
        let mut rv = v.clone();
        // remove value from report
        rv.remove(idx);
        // test report, if safe return immediately
        if test(&rv) {
            return true;
        }
    }
    return false;
}

fn main() {
    let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    let input = std::fs::read_to_string("./input").unwrap();

    let reports = utils::inputs::from_separated_values::<i32>(&input);

    // part one
    let nsafes = reports
        .iter()
        .filter(|report| all_increasing(&report) || all_decreasing(&report))
        .count();
    println!("P1: {nsafes} safe reports");

    // part two
    let nsafes = reports
        .iter()
        .filter(|report| {
            check_allow_one_bad(&report, all_increasing)
                || check_allow_one_bad(&report, all_decreasing)
        })
        .count();
    println!("P1: {nsafes} safe reports");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increasing() {
        assert_eq!(all_increasing(&vec![1, 2, 3, 4, 5]), true);
        assert_eq!(all_increasing(&vec![1, 2, 3, 4, 4]), false);
        assert_eq!(all_increasing(&vec![5, 4, 3, 2, 1]), false);
        assert_eq!(all_increasing(&vec![1, 2, 4, 3, 5]), false);
        assert_eq!(all_increasing(&vec![1, 2, 3, 4, 6]), true);
        assert_eq!(all_increasing(&vec![1, 2, 3, 4, 7]), true);
        assert_eq!(all_increasing(&vec![1, 2, 3, 4, 8]), false);
    }

    #[test]
    fn test_decreasing() {
        assert_eq!(all_decreasing(&vec![5, 4, 3, 2, 1]), true);
        assert_eq!(all_decreasing(&vec![5, 4, 3, 2, 2]), false);
        assert_eq!(all_decreasing(&vec![1, 2, 3, 4, 5]), false);
        assert_eq!(all_decreasing(&vec![5, 4, 5, 2, 1]), false);
    }
}
