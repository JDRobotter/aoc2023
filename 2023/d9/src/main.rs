fn solve(line: &str) -> (i32, i32) {
    let mut vs: Vec<i32> = line.split(' ').map(|s| s.parse().unwrap()).collect();
    let mut zero = false;
    let mut poped = vec![];
    let mut backward = vec![];
    while !zero {
        // store first element to extrapolate backward when we will reach all-zeroes
        backward.push(vs[0]);
        // compute difference element by element
        for i in 1..vs.len() {
            vs[i - 1] = vs[i] - vs[i - 1];
        }
        // new vector is smaller by 1
        poped.push(vs.pop().unwrap());
        // check if all elements are zero
        zero = vs.iter().all(|x| 0 == *x);
    }

    println!("{backward:?}");
    let mut extrapolated = 0;
    for &v in backward.iter().rev() {
        extrapolated = v - extrapolated;
    }

    (poped.iter().sum(), extrapolated)
}

fn main() {
    /*
    let data = "0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45";
    */
    let mut data = String::new();
    let mut file = std::fs::File::open("input").unwrap();
    use std::io::Read;
    file.read_to_string(&mut data).unwrap();

    let mut answer1 = 0;
    let mut answer2 = 0;
    for line in data.split('\n') {
        let line = line.trim();
        if line.len() == 0 {
            continue;
        }
        let (a, b) = solve(line);
        answer1 += a;
        answer2 += b;
    }
    println!("answer1 = {answer1}");
    println!("answer2 = {answer2}");
}
