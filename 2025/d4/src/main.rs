use std::io::Read;

use utils::asciimap::AsciiMap;

fn main() -> Result<(), std::io::Error> {
    let mut input = "".into();
    std::fs::File::open("input")?.read_to_string(&mut input)?;

    dbg!(puzzle1(&input));
    dbg!(puzzle2(&input));
    Ok(())
}

/// check if paper roll is accessible
fn can_access(map: &AsciiMap, xy: (usize, usize)) -> bool {
    let xy = (xy.0 as isize, xy.1 as isize);

    // adjacent roll count
    let mut rolls = 0;

    // check 8 adjacent tiles
    const ADJ: [(isize, isize); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    for ixy in ADJ {
        match map.iget(xy.0 + ixy.0, xy.1 + ixy.1) {
            // roll found
            Some('@') => rolls += 1,
            _ => {}
        }
    }

    rolls < 4
}

fn puzzle1(input: &str) -> u64 {
    // convert input to 2D map
    let map = AsciiMap::from_multi_lines(input);

    accessible(&map).len() as u64
}

fn puzzle2(input: &str) -> u64 {
    // convert input to 2D map
    let mut map = AsciiMap::from_multi_lines(input);

    // count removed rolls
    let mut removed = 0;

    loop {
        // get accessible rolls positions in current map
        let rolls = accessible(&map);
        // return if no more rolls are accessible
        if rolls.is_empty() {
            break;
        }

        // count removed rolls
        removed += rolls.len();

        // mark all rolls as removed
        for xy in rolls {
            map.set(xy.0, xy.1, '-');
        }
        // retry
    }

    removed as u64
}

fn accessible(map: &AsciiMap) -> Vec<(usize, usize)> {
    // accessible rolls
    let mut rolls = vec![];

    // iterate through map elements and positions
    for (c, xy) in map.iter() {
        if c == '@' && can_access(&map, xy) {
            // roll is accessible
            rolls.push(xy);
        }
    }

    rolls
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn example1() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(puzzle1(input), 13);
    }

    #[test]
    fn example2() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(puzzle2(input), 43);
    }

    #[test]
    fn good_part1() {
        let mut input = "".into();
        std::fs::File::open("input")
            .unwrap()
            .read_to_string(&mut input)
            .unwrap();
        assert_eq!(puzzle1(&input), 1372);
    }
}
