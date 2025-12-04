use std::str::FromStr;

pub fn from_separated_values<T: FromStr>(input: &str) -> Vec<Vec<T>> {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .filter_map(|vstr| vstr.parse().ok())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_line() {
        let input = "1   2
3   4
5   6
";

        assert_eq!(
            from_separated_values::<u8>(input),
            vec![vec![1, 2], vec![3, 4], vec![5, 6]]
        );
    }

    #[test]
    fn it_works() {
        let input = "1   2
3   4
5   6";

        assert_eq!(
            from_separated_values::<u8>(input),
            vec![vec![1, 2], vec![3, 4], vec![5, 6]]
        );
    }
}
