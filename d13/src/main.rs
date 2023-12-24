#[derive(Debug)]
enum Symmetry {
    None,
    Horizontal(usize),
    Vertical(usize),
}

struct Pattern {
    map: Vec<char>,
    width: usize,
    height: usize,
}

impl Pattern {
    pub fn new() -> Self {
        Self {
            map: vec![],
            width: 0,
            height: 0,
        }
    }

    pub fn empty(&self) -> bool {
        self.width == 0
    }

    pub fn push(&mut self, line: &str) {
        if self.width == 0 {
            self.width = line.len();
        }
        assert_eq!(self.width, line.len());

        for c in line.chars() {
            self.map.push(c);
        }

        self.height += 1;
    }

    pub fn print(&self) {
        println!("   0123456789ABCDEF");
        let lines = self.map.chunks(self.width).enumerate();
        for (y, line) in lines {
            let mut s = String::new();
            for (x, &c) in line.iter().enumerate() {
                s.push(c);
            }
            println!("{y:2} {s}");
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<&char> {
        if x >= self.width {
            return None;
        }
        let k = x + y * self.width;
        self.map.get(k)
    }

    pub fn check_horizontal_symetry(&self, k: usize) -> usize {
        // mirror
        // 0 ..
        // 1 .. k=1
        // 2 .. k=2
        // k ..
        // 4 .. 2*k-2 = 6 - 2 = 4
        // 5 .. 2*k-1 = 6 - 1= 5

        // count errors
        let mut errors = 0;
        // iterate over lines from 0 to k
        for y in 0..(k + 1) {
            // get mirror image
            let my = 2 * k - y + 1;
            for x in 0..self.width {
                // get base image
                let c = self.get(x, y).unwrap();
                if let Some(mc) = self.get(x, my) {
                    if c != mc {
                        // reflexion does not match
                        errors += 1;
                    }
                } else {
                    // nothing to do reflexion is outside of pattern
                }
            }
        }

        errors
    }

    pub fn check_vertical_symetry(&self, k: usize) -> usize {
        // mirror
        // 0 ..
        // 1 .. k=1
        // 2 .. k=2
        // k ..
        // 4 .. 2*k-2 = 6 - 2 = 4
        // 5 .. 2*k-1 = 6 - 1= 5

        // count errors
        let mut errors = 0;
        // iterate over lines from 0 to k
        for x in 0..(k + 1) {
            // get mirror image
            let mx = 2 * k - x + 1;
            for y in 0..self.height {
                // get base image
                let c = self.get(x, y).unwrap();
                if let Some(mc) = self.get(mx, y) {
                    if c != mc {
                        // reflexion does not match
                        errors += 1;
                    }
                } else {
                    // nothing to do reflexion is outside of pattern
                }
            }
        }

        errors
    }

    pub fn find_symetry(&self) -> Symmetry {
        for x in 0..(self.width - 1) {
            if self.check_vertical_symetry(x) == 0 {
                return Symmetry::Vertical(x + 1);
            }
        }

        for y in 0..(self.height - 1) {
            if self.check_horizontal_symetry(y) == 0 {
                return Symmetry::Horizontal(y + 1);
            }
        }
        Symmetry::None
    }

    pub fn find_smudge(&self) -> Symmetry {
        for x in 0..(self.width - 1) {
            if self.check_vertical_symetry(x) == 1 {
                return Symmetry::Vertical(x + 1);
            }
        }

        for y in 0..(self.height - 1) {
            if self.check_horizontal_symetry(y) == 1 {
                return Symmetry::Horizontal(y + 1);
            }
        }

        Symmetry::None
    }
}

fn main() {
    /*
    let mut data = "#.##..##.
    ..#.##.#.
    ##......#
    ##......#
    ..#.##.#.
    ..##..##.
    #.#.##.#.

    #...##..#
    #....#..#
    ..##..###
    #####.##.
    #####.##.
    ..##..###
    #....#..#";
    */
    let mut data = String::new();
    let mut file = std::fs::File::open("input").unwrap();
    use std::io::Read;
    file.read_to_string(&mut data).unwrap();

    let mut lines = data.split('\n');
    let mut patterns: Vec<Pattern> = vec![];
    let mut run = true;
    while run {
        let mut pattern = Pattern::new();
        loop {
            match lines.next() {
                Some(line) => {
                    let line = line.trim();
                    if line.len() == 0 {
                        break;
                    }
                    pattern.push(line);
                }
                None => {
                    run = false;
                    break;
                }
            }
        }
        if !pattern.empty() {
            patterns.push(pattern);
        }
    }

    let mut answer = 0usize;
    for (n, pat) in patterns.iter().enumerate() {
        println!("PATTERN {n}");
        pat.print();
        let x = pat.find_smudge();
        println!("{x:?}");
        match x {
            Symmetry::Horizontal(x) => answer += 100 * x,
            Symmetry::Vertical(x) => answer += x,
            Symmetry::None => unreachable!(),
        }
    }

    println!("answer = {answer}");
}
