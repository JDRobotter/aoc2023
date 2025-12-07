use std::{collections::HashMap, io::Read};
use utils::asciimap::AsciiMap;

fn main() -> Result<(), std::io::Error> {
    let mut input = "".into();
    std::fs::File::open("input")?.read_to_string(&mut input)?;
    dbg!(puzzle1(&input));
    dbg!(puzzle2(&input));
    Ok(())
}

// Quantum Tachyon Manifold (tm)
struct QTM {
    map: AsciiMap,
    // number of splits
    nsplits: usize,
    // splitter below each splitter count
    splitters: HashMap<(usize, usize), usize>,
}

impl QTM {
    fn new(input: &str) -> Self {
        let map = AsciiMap::from_multi_lines(input);
        Self {
            map,
            nsplits: 0,
            splitters: HashMap::new(),
        }
    }

    fn solve1(&mut self) -> usize {
        self.process();
        self.nsplits
    }

    fn solve2(&mut self) -> usize {
        self.process()
    }

    // return number of created timelines
    fn process(&mut self) -> usize {
        // find starting position
        let (x, y) = self.map.find('S').unwrap();

        // tachyon beam progress down from starting emitter
        let ns = self.advance_beam(x, y + 1);

        // number of timelines is the number of encountered splitters + 1
        ns + 1
    }

    // advance beam and return the number of splitters encountered by the beam
    fn advance_beam(&mut self, x: usize, y: usize) -> usize {
        let x = x;
        let mut y = y;

        loop {
            match self.map.get(x, y) {
                Some('^') => {
                    if let Some(ns) = self.splitters.get(&(x, y)) {
                        // splitter was already visited, returned cached value
                        return *ns;
                    }

                    // one splitter encountered
                    let mut ns = 1;
                    // recurse into left beam and right beam
                    ns += self.advance_beam(x - 1, y);
                    ns += self.advance_beam(x + 1, y);

                    // count one beam split
                    self.nsplits += 1;

                    // upon leaving this beam store number of splitters below this one
                    self.splitters.insert((x, y), ns);

                    return ns;
                }
                Some('.' | '|') => {
                    // empty space or existing beam, replace with beam and continue downward
                    self.map.set(x, y, '|');
                    y += 1;
                }
                Some(c) => panic!("unknown char: {c:?}"),
                None => {
                    // end of self.map reached
                    return 0;
                }
            }
        }
    }
}

#[allow(unused)]
fn puzzle1(input: &str) -> u64 {
    QTM::new(input).solve1() as u64
}

#[allow(unused)]
fn puzzle2(input: &str) -> u64 {
    QTM::new(input).solve2() as u64
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn example1() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(puzzle1(input), 21);
    }

    #[test]
    fn example2() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(puzzle2(input), 40);
    }

    #[test]
    fn example2_smaller() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............";
        assert_eq!(puzzle2(input), 8);
    }

    #[test]
    fn good_part1() {
        let mut input = "".into();
        std::fs::File::open("input")
            .unwrap()
            .read_to_string(&mut input)
            .unwrap();
        assert_eq!(puzzle1(&input), 1524);
    }

    #[test]
    fn good_part2() {
        let mut input = "".into();
        std::fs::File::open("input")
            .unwrap()
            .read_to_string(&mut input)
            .unwrap();
        assert_eq!(puzzle2(&input), 32982105837605);
    }
}
