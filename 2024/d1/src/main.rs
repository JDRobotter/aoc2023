fn main() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3";

    let input = std::fs::read_to_string("./input").unwrap();

    let vs = utils::inputs::from_separated_values::<i32>(&input);
    println!("{vs:?}");

    // transpose vectors
    let vs = utils::arrays::transpose(vs);
    println!("{vs:?}");

    let mut lvs = vs[0].clone();
    let mut rvs = vs[1].clone();

    // compute similarity (part 2)
    let similarity: i32 = lvs
        .iter()
        .map(|&lv| {
            let count = rvs.iter().filter(|&&rv| rv == lv).count();
            lv * (count as i32)
        })
        .sum();
    println!("similarity = {similarity}");

    // sort both vectors in place
    lvs.sort();
    rvs.sort();

    // zip both sorted list, compute distance and sum (part 1)
    let distance: i32 = std::iter::zip(lvs, rvs).map(|(a, b)| (a - b).abs()).sum();
    println!("distance = {distance}");
}
