use utils::asciimap::AsciiMap;

#[derive(Clone, Copy)]
enum BeamDirection {
    Up,
    Left,
    Down,
    Right,
}

struct Beam {
    pub x: isize,
    pub y: isize,
    pub direction: BeamDirection,
}

impl Beam {
    fn new(x: isize, y: isize, direction: BeamDirection) -> Self {
        Self { x, y, direction }
    }

    fn advance(&mut self) {
        match self.direction {
            BeamDirection::Up => self.y -= 1,
            BeamDirection::Left => self.x -= 1,
            BeamDirection::Down => self.y += 1,
            BeamDirection::Right => self.x += 1,
        }
    }

    fn turn(&mut self, direction: BeamDirection) {
        self.direction = direction;
    }

    fn turn_and_advance(&mut self, direction: BeamDirection) {
        self.direction = direction;
        self.advance();
    }
}

use BeamDirection as BD;

struct Contraption {
    map: AsciiMap,
}

impl Contraption {
    fn new(map: AsciiMap) -> Self {
        Self { map }
    }

    fn solve(&self) {
        let mut beams = vec![Beam::new(0, 0, BeamDirection::Right)];
        let mut energized = self.map.clone();
        loop {
            energized.print();
            std::thread::sleep(std::time::Duration::from_millis(200));

            let mut inactive = true;
            let mut new_beams = vec![];
            for beam in &mut beams {
                println!("{} {}", beam.x, beam.y);

                energized.iset(beam.x, beam.y, '#');

                if let Some(tile) = self.map.iget(beam.x, beam.y) {
                    inactive = false;
                    match (beam.direction, tile) {
                        (_, '.') => beam.advance(),
                        (BD::Up, '\\') => beam.turn_and_advance(BD::Left),
                        (BD::Left, '\\') => beam.turn_and_advance(BD::Up),
                        (BD::Down, '\\') => beam.turn_and_advance(BD::Right),
                        (BD::Right, '\\') => beam.turn_and_advance(BD::Down),
                        (BD::Up, '/') => beam.turn_and_advance(BD::Right),
                        (BD::Right, '/') => beam.turn_and_advance(BD::Up),
                        (BD::Down, '/') => beam.turn_and_advance(BD::Left),
                        (BD::Left, '/') => beam.turn_and_advance(BD::Down),
                        (BD::Left | BD::Right, '-') => beam.advance(),
                        (BD::Up | BD::Down, '|') => beam.advance(),
                        (BD::Up | BD::Down, '-') => {
                            let mut nbeam = Beam::new(beam.x, beam.y, BeamDirection::Right);
                            nbeam.advance();
                            new_beams.push(nbeam);
                            beam.turn_and_advance(BD::Left);
                        }
                        (BD::Left | BD::Right, '|') => {
                            let mut nbeam = Beam::new(beam.x, beam.y, BeamDirection::Up);
                            nbeam.advance();
                            new_beams.push(nbeam);
                            beam.turn_and_advance(BD::Down);
                        }
                        _ => unreachable!(),
                    }
                }
            }

            beams.append(&mut new_beams);

            if inactive {
                break;
            }
        }
    }
}

fn main() {
    let mut data = String::new();
    let mut file = std::fs::File::open("_input").unwrap();
    use std::io::Read;
    file.read_to_string(&mut data).unwrap();

    let con = Contraption::new(AsciiMap::from_multi_lines(data));
    con.solve();
}
