fn main() {
    let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    let input = std::fs::read_to_string("input").unwrap();

    let mut lines = input.lines();
    // iterate over ordering rules
    let mut rules = vec![];
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let mut split = line.split('|');
        let a: i32 = split.next().unwrap().parse().unwrap();
        let b: i32 = split.next().unwrap().parse().unwrap();
        rules.push((a, b))
    }

    // iterate over updates
    let mut updates = vec![];
    while let Some(line) = lines.next() {
        let update: Vec<i32> = line.split(',').map(|x| x.parse().unwrap()).collect();
        updates.push(update);
    }

    //
    let mut result_p1 = 0;
    let mut result_p2 = 0;
    for update in updates {
        let b = check_order(&update, &rules);
        if b {
            // find middle page and sum value
            result_p1 += update[update.len() / 2]
        } else {
            // invalid order
            let mut nup = update.clone();
            for _ in 0..nup.len() {
                reorder(&mut nup, &rules);
            }
            //let b = check_order(&nup, &rules);
            result_p2 += nup[nup.len() / 2]
        }
    }
    println!("P1: result = {result_p1}");
    println!("P2: result = {result_p2}");
}

fn check_order(update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
    // iterate over rules
    for (before, after) in rules {
        let bidx = update.iter().rev().position(|&x| x == *before);
        let aidx = update.iter().position(|&x| x == *after);
        if let (Some(bidx), Some(aidx)) = (bidx, aidx) {
            let bidx = update.len() - bidx;
            if bidx > aidx {
                return false;
            };
        }
    }
    return true;
}

fn reorder(update: &mut Vec<i32>, rules: &Vec<(i32, i32)>) {
    // iterate over rules
    for (before, after) in rules {
        let bidx = update.iter().rev().position(|&x| x == *before);
        let aidx = update.iter().position(|&x| x == *after);
        if let (Some(bidx), Some(aidx)) = (bidx, aidx) {
            let bidx = update.len() - bidx - 1;
            if bidx > aidx {
                // incorrectly ordered page
                utils::swap(update, aidx, bidx);
            };
        }
    }
}
