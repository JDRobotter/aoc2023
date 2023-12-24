use utils::asciimap::{AsciiMap, Rotation};

pub struct Platform {
    map: AsciiMap,
}

impl Platform {
    pub fn new(map: AsciiMap) -> Self {
        Self { map }
    }

    pub fn print(&self) {
        self.map.print()
    }

    fn slide_column(map: &mut AsciiMap, x: usize) {
        let mut empty_position = None;
        // move along column
        for y in 0..map.height() {
            match map.get(x, y).unwrap() {
                '.' => {
                    // empty block
                    if empty_position.is_none() {
                        // mark this position if it's the first we encounter
                        empty_position = Some(y);
                    }
                }
                'O' => {
                    // round rock
                    if let Some(ey) = empty_position {
                        // we encountered earlier an empty position
                        map.swap(x, y, x, ey);
                        // update empty position
                        empty_position = Some(ey + 1);
                    }
                }
                '#' => {
                    // square rock
                    // reset empty position
                    empty_position = None;
                }

                _ => unreachable!(),
            }
        }
    }

    pub fn slide(map: &mut AsciiMap) {
        // each column
        for x in 0..map.width() {
            Self::slide_column(map, x);
        }
    }

    pub fn cycle(&mut self, n: usize) -> Vec<usize> {
        let mut ws = vec![];
        for _ in 0..n {
            self.map.set_rotation(Rotation::R0);
            Self::slide(&mut self.map);
            self.map.set_rotation(Rotation::R270);
            Self::slide(&mut self.map);
            self.map.set_rotation(Rotation::R180);
            Self::slide(&mut self.map);
            self.map.set_rotation(Rotation::R90);
            Self::slide(&mut self.map);

            self.map.set_rotation(Rotation::R0);
            let w = Self::load(&self.map);
            ws.push(w);
        }

        ws
    }

    pub fn load(map: &AsciiMap) -> usize {
        let mut w = 0;
        for x in 0..map.width() {
            for y in 0..map.height() {
                let c = map.get(x, y).unwrap();
                if *c == 'O' {
                    w += map.height() - y;
                }
            }
        }
        w
    }
}

fn find_repetition(ws: &Vec<usize>) -> Option<(usize, Vec<usize>)> {
    // start testing after an arbitrary number of iteration
    // hoping the sequence is stabilized
    let start = 100;
    let ws = &ws[start..];

    // try different sequences length
    for sl in 2..50 {
        if (0..sl).map(|k| ws[k] == ws[k + sl]).all(|b| b) {
            let values = (0..sl).map(|k| ws[k]).collect();
            return Some((start, values));
        }
    }

    None
}

fn main() {
    /*
    let data = "O....#....
    O.OO#....#
    .....##...
    OO.#O....O
    .O.....O#.
    O.#..O.#.#
    ..O..#O..O
    .......O..
    #....###..
    #OO..#....";
    */

    let mut data = String::new();
    let mut file = std::fs::File::open("input").unwrap();
    use std::io::Read;
    file.read_to_string(&mut data).unwrap();

    let map = AsciiMap::from_multi_lines(data);
    let mut platform = Platform::new(map);
    let ws = platform.cycle(1000);
    let (s, vs) = find_repetition(&ws).unwrap();
    println!("{s} {vs:?}");
    let n = 1_000_000_000;

    let k = (n - s - 1) % vs.len();
    println!("{k} {}", vs[k]);
}
