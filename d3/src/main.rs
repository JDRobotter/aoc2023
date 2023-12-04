use std::collections::VecDeque;

struct Schematic {
    matrix: Vec<Vec<char>>,
    width: isize,
    height: isize,
    sum: usize,
}

impl Schematic {
    pub fn from(s: &str) -> Self {
        let matrix: Vec<Vec<char>> = s.split('\n').map(|s| s.chars().collect()).collect();
        // NOTE we assume that every line as the same width
        let height = matrix.len() as isize;
        let width = matrix[0].len() as isize;

        Self {
            matrix,
            width,
            height,
            sum: 0,
        }
    }

    fn sum(&self) -> usize {
        self.sum
    }

    fn set(&mut self, x: isize, y: isize, nc: char) -> Option<()> {
        let x = x as usize;
        let y = y as usize;
        let line = self.matrix.get_mut(y)?;
        let c = line.get_mut(x)?;
        *c = nc;
        Some(())
    }

    fn get(&self, x: isize, y: isize) -> Option<char> {
        let x = x as usize;
        let y = y as usize;
        let line = self.matrix.get(y)?;
        let c = line.get(x)?;
        Some(*c)
    }

    fn is_symbol(&self, c: char) -> bool {
        match c {
            '0'..='9' => false,
            '.' => false,
            _ => true,
        }
    }

    pub fn print(&self) {
        for line in &self.matrix {
            let line: String = line.iter().collect();
            println!("{}", line);
        }
    }

    fn extract_part_number(&mut self, x: isize, y: isize) {
        let mut pn = VecDeque::new();
        // starting char
        pn.push_front(self.get(x, y).unwrap());
        // clear char
        self.set(x, y, '.');

        // search chars right of point
        let mut sx = x + 1;
        while let Some(c) = self.get(sx, y) {
            if !c.is_numeric() {
                break;
            }
            pn.push_back(c);
            self.set(sx, y, '.');
            sx += 1;
        }
        // search chars left of point
        let mut sx = x - 1;
        while let Some(c) = self.get(sx, y) {
            if !c.is_numeric() {
                break;
            }
            pn.push_front(c);
            self.set(sx, y, '.');
            sx -= 1;
        }

        let s: String = pn.iter().collect();
        if let Some(pn) = usize::from_str_radix(&s, 10).ok() {
            println!("PN {pn}");
            self.sum += pn;
        }
    }

    pub fn find_associated_tag(&mut self, x: isize, y: isize) {
        let positions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for (dx, dy) in positions {
            let x = x + dx;
            let y = y + dy;
            if let Some(c) = self.get(x, y) {
                if c.is_numeric() {
                    // we found a part number
                    self.extract_part_number(x, y);
                }
            }
        }
    }

    pub fn process(&mut self) {
        // iterate over each line and search for symbols
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(c) = self.get(x, y) {
                    if self.is_symbol(c) {
                        println!("{x}:{y} = {c}");
                        self.find_associated_tag(x, y);
                    }
                }
            }
        }
    }
}

use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("input").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let mut s = Schematic::from(&input);
    s.print();
    s.process();

    println!("sum={}", s.sum());
}
