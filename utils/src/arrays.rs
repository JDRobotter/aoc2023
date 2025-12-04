pub fn transpose<T: Copy>(ivs: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut ovs = vec![];

    // assume ivs to be a matrix (same number of elements per line)
    // use first line to set up vecs
    for _ in &ivs[0] {
        ovs.push(vec![]);
    }

    // transpose line and colums
    for v in ivs {
        for (idx, x) in v.iter().enumerate() {
            ovs[idx].push(*x);
        }
    }

    ovs
}
