struct CombinePairs {
    i: usize,
    j: usize,
    n: usize,
}

impl CombinePairs {
    pub fn new(n: usize) -> Self {
        Self { i: 0, j: 0, n }
    }
}

impl Iterator for CombinePairs {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        // do a simple for i { for j {} } combination
        self.i += 1;
        // only count numbers below diagonal
        if self.i == (self.j + 1) {
            self.i = 0;
            self.j += 1;
            if self.j == self.n {
                return None;
            }
        }
        Some((self.i, self.j))
    }
}

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

impl std::ops::Sub for XY {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
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
}

pub fn grid_distance(a: XY, b: XY) -> isize {
    // starting from a
    // (9,0) => 9
    // #########
    // (9,1) => 10
    // ####
    //    ######
    // (9,3) => 11
    // ####
    //    ###
    //      ####
    // (9,4) => 12
    // ###
    //   ###
    //     ###
    //       ###
    let delta = b - a;
    delta.x.abs() + delta.y.abs()
}

struct Universe {
    width: usize,
    height: usize,
    tiles: Vec<char>,
    galaxies: Vec<XY>,
}

impl Universe {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            tiles: Vec::new(),
            galaxies: Vec::new(),
        }
    }

    pub fn print(&self) {
        let tiles = self.tiles.chunks(self.width);
        for line in tiles {
            let mut s = String::new();
            line.iter().for_each(|&c| s.push(c));
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
            self.tiles.push(c);
        });

        self.height += 1;
    }

    fn set_tile(&mut self, xy: XY, c: char) {
        let k = self.idx_from_xy(xy);
        self.tiles[k] = c;
    }

    fn get_tile(&self, xy: XY) -> char {
        let k = self.idx_from_xy(xy);
        self.tiles[k]
    }

    pub fn expand(&mut self) {
        let mut empty_y = vec![];
        // iterate through lines to find empty ones
        for y in 0..self.height {
            if (0..self.width).all(|x| self.get_tile(XY::new(x as isize, y as isize)) == '.') {
                empty_y.push(y);
            }
        }
        // iterate through columns to find empty ones
        let mut empty_x = vec![];
        for x in 0..self.width {
            if (0..self.height).all(|y| self.get_tile(XY::new(x as isize, y as isize)) == '.') {
                empty_x.push(x);
            }
        }

        let nheight = self.height + empty_y.len();
        let nwidth = self.width + empty_x.len();
        let mut ntiles: Vec<char> = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                let here = XY::new(x as isize, y as isize);
                // this column is an empty one, push one more space
                if empty_x.contains(&x) {
                    ntiles.push('.');
                    ntiles.push('.');
                } else {
                    ntiles.push(self.get_tile(here));
                }
            }

            // this line is an empty one push one more line
            if empty_y.contains(&y) {
                for _ in 0..nwidth {
                    ntiles.push('.');
                }
            }
        }

        self.height = nheight;
        self.width = nwidth;
        self.tiles = ntiles;
    }

    pub fn count_galaxies(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let here = XY::new(x as isize, y as isize);

                if self.get_tile(here) == '#' {
                    self.galaxies.push(here);
                }
            }
        }
        println!("{:?}", self.galaxies);
    }

    pub fn distance_between(&self, a: usize, b: usize) -> isize {
        grid_distance(self.galaxies[a], self.galaxies[b])
    }

    pub fn answer1(&self) -> isize {
        let mut sum = 0;
        let n = self.galaxies.len();
        // for each pair of galaxies
        for (a, b) in CombinePairs::new(n) {
            let distance = self.distance_between(a, b);
            sum += distance;
            println!("{a} {b} {distance}");
        }
        sum
    }
}

fn main() {
    assert_eq!(grid_distance(XY::new(0, 0), XY::new(1, 0)), 1);
    assert_eq!(grid_distance(XY::new(0, 0), XY::new(10, 0)), 10);
    assert_eq!(grid_distance(XY::new(0, 0), XY::new(0, 10)), 10);
    assert_eq!(grid_distance(XY::new(10, 0), XY::new(0, 0)), 10);

    /*
    let data = "...#......
    .......#..
    #.........
    ..........
    ......#...
    .#........
    .........#
    ..........
    .......#..
    #...#.....";
    */
    let mut data = String::new();
    let mut file = std::fs::File::open("input").unwrap();
    use std::io::Read;
    file.read_to_string(&mut data).unwrap();

    let mut f = Universe::new();
    for line in data.split('\n') {
        let line = line.trim();
        if line.len() == 0 {
            continue;
        }
        f.push_line(line);
    }

    f.print();
    f.expand();
    f.print();
    f.count_galaxies();

    let ans1 = f.answer1();
    println!("ans1 = {ans1}");
    // use first line to guess
}
