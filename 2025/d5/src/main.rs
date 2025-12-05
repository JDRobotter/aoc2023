use std::io::Read;

fn main() -> Result<(), std::io::Error> {
    let mut input = "".into();
    std::fs::File::open("input")?.read_to_string(&mut input)?;
    Ok(())
}

fn is_fresh(id: usize, db: &[(usize, usize)]) -> bool {
    // iterate through db ranges
    for &(a, b) in db {
        // ingredient is fresh if its ID is in ANY range
        if a <= id && id <= b {
            return true;
        }
    }
    return false;
}

fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    // split both parts of input
    let (db, items) = input.trim().split_once("\n\n").unwrap();

    // parse IDs database
    let db: Vec<(usize, usize)> = db
        .trim()
        .split('\n')
        .map(|s| {
            let (a, b) = s.split_once('-').unwrap();
            let a: usize = a.parse().unwrap();
            let b: usize = b.parse().unwrap();
            (a, b)
        })
        .collect();

    // parse ingredients
    let items: Vec<usize> = items
        .trim()
        .split('\n')
        .map(|s| {
            let id: usize = s.parse().unwrap();
            id
        })
        .collect();

    (db, items)
}

fn puzzle1(input: &str) -> u64 {
    let (db, items) = parse_input(input);

    // check ingredients freshness and count them
    let mut fresh = 0;
    for item in items {
        if is_fresh(item, &db) {
            fresh += 1;
        }
    }

    fresh
}

/// find if x is in any db range, return index if so
fn inr(db: &Vec<(usize, usize)>, x: usize, exclude: Option<usize>) -> Option<usize> {
    for (k, r) in db.iter().enumerate() {
        if Some(k) == exclude {
            continue;
        }
        if r.0 <= x && x <= r.1 {
            return Some(k);
        }
    }
    None
}

fn find_fully_included(db: &Vec<(usize, usize)>) -> Option<usize> {
    // check if any range is fully included in another
    for (k, &r) in db.iter().enumerate() {
        // check
        match (inr(db, r.0, Some(k)), inr(db, r.1, Some(k))) {
            (Some(k1), Some(k2)) if k1 == k2 => return Some(k),
            _ => {}
        }
    }
    return None;
}

fn remove_overlaps(db: &mut Vec<(usize, usize)>) {
    loop {
        if let Some(k) = find_fully_included(db) {
            // remove range from database
            db.remove(k);
        } else {
            break;
        }
    }
}

/// extend current non-overlaping db with provided range
fn extend(db: &mut Vec<(usize, usize)>, mut nr: (usize, usize)) {
    //          [----------]    [-------] [-------]
    // case 1          [------]
    // case 2                [-----]
    // case 3          [----------------------]

    // check if left boundary is in any existing range
    if let Some(k) = inr(&db, nr.0, None) {
        // if so extend new range to include this left boundary
        nr.0 = db[k].0;
    }

    // check if right boundary is in any existing range
    if let Some(k) = inr(&db, nr.1, None) {
        // if so extend existing range to include this right boundary
        nr.1 = db[k].1;
    }

    // append new range to db
    db.push(nr);

    // remove any overlaping ranges in current db
    remove_overlaps(db);
}

fn puzzle2(input: &str) -> u64 {
    let (db, _items) = parse_input(input);

    // create a non-overlaping db
    let mut nodb = vec![];
    for range in db {
        extend(&mut nodb, range);
    }

    let sum: usize = nodb.iter().map(|&r| r.1 - r.0 + 1).sum();
    sum as u64
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn example1() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!(puzzle1(input), 3);
    }

    #[test]
    fn example2() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!(puzzle2(input), 14);
    }

    #[test]
    fn good_part1() {
        let mut input = "".into();
        std::fs::File::open("input")
            .unwrap()
            .read_to_string(&mut input)
            .unwrap();
        assert_eq!(puzzle1(&input), 567);
    }

    #[test]
    fn good_part2() {
        let mut input = "".into();
        std::fs::File::open("input")
            .unwrap()
            .read_to_string(&mut input)
            .unwrap();
        assert_eq!(puzzle2(&input), 354149806372909);
    }
}
