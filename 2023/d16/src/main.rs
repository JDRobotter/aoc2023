use std::{io::Read, ops::Range, time::Duration};
use utils::asciimap::AsciiMap;

fn main() -> Result<(), std::io::Error> {
    let mut input = "".into();
    std::fs::File::open("input")?.read_to_string(&mut input)?;
    //dbg!(puzzle1(&input));
    //dbg!(puzzle2(&input));
    Ok(())
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}
type D = Direction;

#[derive(Debug)]
struct Beam {
    pub x: isize,
    pub y: isize,
    pub dir: Direction,
}

impl Beam {
    fn new(x: isize, y: isize, dir: Direction) -> Self {
        Self { x, y, dir }
    }

    fn advance(&mut self) {
        match self.dir {
            D::Right => self.x += 1,
            D::Left => self.x -= 1,
            D::Up => self.y -= 1,
            D::Down => self.y += 1,
        }
    }

    /// return true if beam is aligned with object, false otherwise
    fn is_aligned_with(&self, c: char) -> bool {
        match (c, &self.dir) {
            ('<' | '>', D::Right | D::Left) => true,
            ('<' | '>', _) => false,
            ('^' | 'v', D::Up | D::Down) => true,
            ('^' | 'v', _) => false,
            ('-', D::Right | D::Left) => true,
            ('|', D::Right | D::Left) => false,
            ('-', D::Up | D::Down) => false,
            ('|', D::Up | D::Down) => true,
            (c, _) => panic!("invalid char to check for alignement {c:?}"),
        }
    }

    /// reflect beam onto mirror
    fn reflect(&mut self, c: char) {
        // change beam direction
        match (c, &self.dir) {
            ('\\', D::Right) => self.dir = D::Down,
            ('\\', D::Down) => self.dir = D::Right,
            ('\\', D::Left) => self.dir = D::Up,
            ('\\', D::Up) => self.dir = D::Left,
            ('/', D::Right) => self.dir = D::Up,
            ('/', D::Up) => self.dir = D::Right,
            ('/', D::Left) => self.dir = D::Down,
            ('/', D::Down) => self.dir = D::Left,
            _ => panic!("invalid char to check for reflection"),
        }
        // advance beam
        self.advance();
    }

    /// split beam and into two new beams
    fn split(self) -> (Beam, Beam) {
        let Beam { x, y, dir } = self;
        match dir {
            D::Right | D::Left => (Beam::new(x, y - 1, D::Up), Beam::new(x, y + 1, D::Down)),
            D::Up | D::Down => (Beam::new(x - 1, y, D::Left), Beam::new(x + 1, y, D::Right)),
        }
    }

    fn set_on_map(&self, map: &mut AsciiMap) {
        let c = match self.dir {
            D::Right => '>',
            D::Left => '<',
            D::Up => '^',
            D::Down => 'v',
        };
        map.iset(self.x, self.y, c);
    }
}

struct LavaFloor {
    pub map: AsciiMap,
    pub energized_map: AsciiMap,
}

impl LavaFloor {
    fn new(map: AsciiMap) -> Self {
        let energized_map = map.clone_and_fill('.');
        Self { map, energized_map }
    }

    fn from_input(input: &str) -> Self {
        let map = AsciiMap::from_multi_lines(input);
        Self::new(map)
    }

    fn emit_beam_xyd(&mut self, (x, y): (isize, isize), dir: Direction) {
        self.emit_beam(Beam::new(x, y, dir));
    }

    /// Emit laser beam from given position going given direction
    /// and update enegergized map
    fn emit_beam(&mut self, mut beam: Beam) {
        loop {
            // energize current position
            self.energized_map.iset(beam.x, beam.y, '#');

            // advance beam
            match self.map.iget(beam.x, beam.y) {
                Some('.') => {
                    // just like in examples, replace empty space with beam direction
                    beam.set_on_map(&mut self.map);
                    // empty space, move beam forward
                    beam.advance();
                }
                Some(c @ ('<' | '^' | '>' | 'v')) => {
                    // beam reached a tile already energized
                    // if beam is aligned with previous beam stop here because beam
                    // will energized tiles already energized by previous beam
                    if beam.is_aligned_with(*c) {
                        break;
                    } else {
                        // otherwise erase previous beam direction and move forward
                        beam.set_on_map(&mut self.map);
                        beam.advance();
                    }
                }
                Some(c @ ('|' | '-')) => {
                    // splitter encountered
                    if !beam.is_aligned_with(*c) {
                        // split will occur
                        let (lbeam, rbeam) = beam.split();
                        // advance both beams
                        self.emit_beam(lbeam);
                        self.emit_beam(rbeam);
                        break;
                    } else {
                        // beam goes forward as if it encountered empty space
                        beam.advance();
                    }
                }
                Some(c @ ('\\' | '/')) => {
                    // mirror encountered
                    beam.reflect(*c);
                }

                Some(c) => panic!("unhandled char reached: {c:?}"),
                None => {
                    // end of map reached
                    break;
                }
            }
        }
    }

    /// Return number of tile energized by laser
    fn energized(&self) -> usize {
        self.energized_map.count('#')
    }
}

pub fn find_highest_energy(input: &str) -> usize {
    // load map
    let map = AsciiMap::from_multi_lines(input);
    let (w, h) = map.size();
    let w = w as isize;
    let h = h as isize;

    let inputs = vec![
        (0, w, 0, 1, Direction::Down),
        (0, w, h - 1, h, Direction::Up),
        (0, 1, 0, h, Direction::Right),
        (w - 1, w, 0, h, Direction::Left),
    ];

    let mut maxnrj = 0usize;
    for (xl, xh, yl, yh, direction) in inputs {
        for x in xl..xh {
            for y in yl..yh {
                let mut lf = LavaFloor::new(map.clone());
                lf.emit_beam_xyd((x, y), direction);
                let nrj = lf.energized();

                maxnrj = nrj.max(maxnrj);
                eprintln!("{x} {y} {nrj}");
            }
        }
    }

    maxnrj
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn example1() {
        let input = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";
        //
        let solved_energized_map = AsciiMap::from_multi_lines(
            "######....
.#...#....
.#...#####
.#...##...
.#...##...
.#...##...
.#..####..
########..
.#######..
.#...#.#..",
        );
        let mut lf = LavaFloor::from_input(input);
        // emit starts top-left corner going right
        lf.emit_beam_xyd((0, 0), Direction::Right);
        // assert maps from AOC instructions
        assert_eq!(lf.energized_map, solved_energized_map);
        // count energized tiles
        assert_eq!(lf.energized() as u64, 46);
    }

    #[ignore]
    #[test]
    fn good_part1_subsection() {
        let input = "\\|.....-.....\\....../.....-.........
...............|...|...........-....
.........|....\\......-..............
...-................................
\\...|...|-..........................
...............|.....|.......\\..|...
.\\....\\............./............|..
./..........................\\..\\....
............/....................-..
..................................|/
............|||.....\\....|....|.....
......-...........................-.
./............/.-|.\\/.......-./\\....
.-..-.............-\\................
..|/..............................|.
...-/.....\\........\\...|............
................-...|............/..
............\\............./\\........
..................|...../...........
........../................../......
.../...........\\....|...............
/..\\................................
.....-.......\\.........-.-.....|....
.........-....-.....................
.....\\\\.............................
.............|.|....................";

        let mut lf = LavaFloor::from_input(input);
        // emit starts top-left corner going right
        lf.emit_beam_xyd((0, 0), Direction::Right);
        // count energized tiles
        assert_eq!(lf.energized() as u64, 46);
    }

    #[test]
    fn example2() {
        let input = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";
        //
        assert_eq!(find_highest_energy(&input), 51);
    }

    #[test]
    fn good_part1() {
        let mut input = "".into();
        std::fs::File::open("input")
            .unwrap()
            .read_to_string(&mut input)
            .unwrap();
        let mut lf = LavaFloor::from_input(&input.trim());
        // emit starts top-left corner going right
        lf.emit_beam_xyd((0, 0), Direction::Right);
        // count energized tiles
        assert_eq!(lf.energized() as u64, 7060);
    }

    #[test]
    fn good_part2() {
        let mut input = "".into();
        std::fs::File::open("input")
            .unwrap()
            .read_to_string(&mut input)
            .unwrap();
        assert_eq!(find_highest_energy(&input.trim()), 7493);
    }
}
