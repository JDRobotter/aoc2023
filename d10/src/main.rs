#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct XY {
    pub x: isize,
    pub y: isize,
}

impl std::ops::Neg for XY {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl std::ops::Add for XY {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl XY {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    const UP: XY = XY { x: 0, y: -1 };
    const DOWN: XY = XY { x: 0, y: 1 };
    const LEFT: XY = XY { x: -1, y: 0 };
    const RIGHT: XY = XY { x: 1, y: 0 };
}
struct Field {
    width: usize,
    height: usize,
    orig: Vec<char>,
    tiles: Vec<char>,
}

impl Field {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            orig: Vec::new(),
            tiles: Vec::new(),
        }
    }

    pub fn print(&self) {
        let tiles = self.tiles.chunks(self.width);
        for (line, oline) in tiles.zip(self.orig.chunks(self.width)) {
            let mut s = String::new();
            line.iter().for_each(|&c| s.push(c));
            s.push(' ');
            oline.iter().for_each(|&c| s.push(c));
            println!("{s}");
        }
    }

    fn xy_from_idx(&self, idx: usize) -> XY {
        let idx = idx as isize;
        let w = self.width as isize;
        XY::new(idx % w, idx / w)
    }

    fn idx_from_xy(&self, p: XY) -> usize {
        let k = p.y * (self.width as isize) + p.x;
        assert!(k >= 0);
        k as usize
    }

    pub fn push_line(&mut self, line: &str) {
        let w = line.len();
        if self.width > 0 {
            assert_eq!(w, self.width);
        }
        self.width = w;

        line.chars().for_each(|c| {
            self.orig.push(c);
            self.tiles.push(c);
        });

        self.height += 1;
    }

    fn find_start(&self) -> XY {
        let (k, _) = self
            .tiles
            .iter()
            .enumerate()
            .find(|&(_, &c)| c == 'S')
            .unwrap();
        self.xy_from_idx(k)
    }

    fn set_orig_tile(&mut self, xy: XY, c: char) {
        let k = self.idx_from_xy(xy);
        self.orig[k] = c;
    }

    fn set_tile(&mut self, xy: XY, c: char) {
        let k = self.idx_from_xy(xy);
        self.tiles[k] = c;
    }

    fn get_tile(&self, xy: XY) -> char {
        let k = self.idx_from_xy(xy);
        self.tiles[k]
    }

    fn get_orig_tile(&self, xy: XY) -> char {
        let k = self.idx_from_xy(xy);
        self.orig[k]
    }

    fn get_pipe_connections(&self, c: char) -> Option<(XY, XY)> {
        match c {
            '|' => Some((XY::UP, XY::DOWN)),
            '-' => Some((XY::LEFT, XY::RIGHT)),
            'L' => Some((XY::UP, XY::RIGHT)),
            'J' => Some((XY::UP, XY::LEFT)),
            '7' => Some((XY::DOWN, XY::LEFT)),
            'F' => Some((XY::DOWN, XY::RIGHT)),
            '.' => None,
            'M' => None,
            _ => panic!("unknown tile"),
        }
    }

    fn connect_back(&self, pos: XY, direction: XY) -> bool {
        // check if neighboor pipe connect back to tile
        let pipe = self.get_tile(pos + direction);
        if let Some((a, b)) = self.get_pipe_connections(pipe) {
            a == -direction || b == -direction
        } else {
            // neighboor is not a pipe
            false
        }
    }

    pub fn advance(&mut self, pos: &XY) -> XY {
        let pipe = self.get_tile(*pos);
        // mark tile
        self.set_tile(*pos, 'M');
        let (a, b) = self.get_pipe_connections(pipe).unwrap();
        let a = *pos + a;
        let b = *pos + b;

        // do not go to marked tiles
        if self.get_tile(a) == 'M' {
            return b;
        }
        if self.get_tile(b) == 'M' {
            return a;
        }
        panic!("should not happen");
    }

    pub fn fix_start_position(&mut self, pos: XY, na: XY, nb: XY) {
        let tile = match (na, nb) {
            (XY::UP, XY::DOWN) => '|',
            (XY::LEFT, XY::RIGHT) => '-',
            (XY::UP, XY::RIGHT) => 'L',
            (XY::UP, XY::LEFT) => 'J',
            (XY::DOWN, XY::LEFT) => '7',
            (XY::DOWN, XY::RIGHT) => 'F',
            _ => panic!("unknown"),
        };

        self.set_tile(pos, tile);
        self.set_orig_tile(pos, tile);
    }

    pub fn measure_marked_area(&mut self) -> usize {
        // lets throw a ray from left to right counting wheter
        // we are inside or outside of marked area
        let mut area = 0;
        for y in 0..self.height {
            // rays start
            let mut inwall = 'X';
            let mut inside = false;
            for x in 0..self.width {
                let here = XY::new(x as isize, y as isize);
                if self.get_tile(here) == 'M' {
                    // tile is part of the loop we are measuring
                    let tile = self.get_orig_tile(here);
                    match (tile, inwall) {
                        ('|', _) => {
                            // ray traverse a wall, switch outside/inside
                            inside = !inside;
                        }
                        ('F', _) => {
                            // ray enter a wall through an 'F' corner
                            inwall = 'F';
                        }
                        ('L', _) => {
                            // ray enter a wall through an 'L' corner
                            inwall = 'L';
                        }
                        ('J', 'F') | ('7', 'L') => {
                            // ray leave a wall and as done through it
                            inwall = 'X';
                            inside = !inside;
                        }
                        _ => {}
                    }
                } else {
                    // this tile is not part of the loop and thus can be outside OR inside
                    if inside {
                        // mark tile as inside
                        self.set_tile(here, 'I');
                        area += 1;
                    } else {
                    }
                }
            }
        }

        area
    }

    pub fn solve(&mut self) {
        let start = self.find_start();

        // find starting position neighboor pipes
        let mut neighboors: Vec<XY> = [XY::UP, XY::DOWN, XY::LEFT, XY::RIGHT]
            .iter()
            .filter_map(|&dir| self.connect_back(start, dir).then_some(dir))
            .collect();

        // rules only allow for 2 connecting pipes
        assert_eq!(neighboors.len(), 2);

        // fix starting position
        self.fix_start_position(start, neighboors[0], neighboors[1]);

        let mut a = start + neighboors[0];
        let mut b = start + neighboors[1];
        self.set_tile(start, 'M');

        // progress through pipe in both directions
        let mut steps = 1;
        loop {
            steps += 1;

            a = self.advance(&a);
            if a == b {
                break;
            }

            b = self.advance(&b);
            if b == a {
                break;
            }
        }

        // mark both a and b position to complete the loop
        self.set_tile(a, 'M');
        self.set_tile(b, 'M');

        println!("{a:?} {b:?} {steps}");
    }
}

fn main() {
    /*
        let data = ".....
        .S-7.
        .|.|.
        .L-J.
        .....";
        let data = "..F7.
    .FJ|.
    SJ.L7
    |F--J
    LJ...";
    */
    /*
        let data = "...........
    .S-------7.
    .|F-----7|.
    .||.....||.
    .||.....||.
    .|L-7.F-J|.
    .|..|.|..|.
    .L--J.L--J.
    ...........";
    */

    /*
        let data = ".F----7F7F7F7F-7....
    .|F--7||||||||FJ....
    .||.FJ||||||||L7....
    FJL7L7LJLJ||LJ.L-7..
    L--J.L7...LJS7F-7L7.
    ....F-J..F7FJ|L7L7L7
    ....L7.F7||L7|.L7L7|
    .....|FJLJ|FJ|F7|.LJ
    ....FJL-7.||.||||...
    ....L---J.LJ.LJLJ...";
    */
    let mut data = String::new();
    let mut file = std::fs::File::open("input").unwrap();
    use std::io::Read;
    file.read_to_string(&mut data).unwrap();

    let mut f = Field::new();
    for line in data.split('\n') {
        let line = line.trim();
        if line.len() == 0 {
            continue;
        }
        f.push_line(line);
    }

    f.solve();

    let area = f.measure_marked_area();
    f.print();
    println!("area = {area}");
    // use first line to guess
}
