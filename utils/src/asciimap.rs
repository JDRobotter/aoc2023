#[derive(Clone, Copy)]
pub enum Rotation {
    R0,
    R90,
    R180,
    R270,
}

pub struct AsciiMap {
    map: Vec<char>,
    width: usize,
    height: usize,
    rotation: Rotation,
}

impl AsciiMap {
    pub fn new() -> Self {
        Self {
            map: vec![],
            width: 0,
            height: 0,
            rotation: Rotation::R0,
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            map: self.map.clone(),
            width: self.width,
            height: self.height,
            rotation: self.rotation,
        }
    }

    pub fn from_multi_lines(lines: impl AsRef<str>) -> Self {
        let lines: &str = lines.as_ref();
        let mut _self = AsciiMap::new();
        for line in lines.trim().split('\n') {
            let line = line.trim();
            _self.push(line);
        }
        _self
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn set_rotation(&mut self, rotation: Rotation) {
        self.rotation = rotation;
    }

    fn rotate(&self, x: usize, y: usize) -> (usize, usize) {
        // 0     90    180  270
        // ABC   GDA   IHG  CFI
        // DEF   HEB   FED  BEH
        // GHI   IFC   CBA  ADG
        // 2,2 1,2 0,2
        // 2,0 2,1 2,2
        let w = self.width;
        let h = self.height;
        match self.rotation {
            Rotation::R0 => (x, y),
            Rotation::R90 => (w - 1 - y, x),
            Rotation::R180 => (w - 1 - x, h - 1 - y),
            Rotation::R270 => (y, w - 1 - x),
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
        let mut sd = String::new();
        let mut su = String::new();
        for i in 0..self.width {
            let i = i as u32;
            sd.push(std::char::from_digit(i / 10, 10).unwrap());
            su.push(std::char::from_digit(i % 10, 10).unwrap());
        }

        if self.width > 10 {
            println!("   {sd}");
        }
        println!("   {su}");
        for y in 0..self.height() {
            let mut s = String::new();
            for x in 0..self.width() {
                let c = self.get(x, y).unwrap();
                s.push(*c);
            }
            println!("{y:2} {s}");
        }
    }

    fn index_to_xy(&self, k: usize) -> (usize, usize) {
        (k % self.width, k / self.width)
    }

    fn xy_to_index(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    pub fn iset(&mut self, x: isize, y: isize, c: char) -> Option<()> {
        if x < 0 || y < 0 {
            return None;
        }
        self.set(x as usize, y as usize, c)
    }

    pub fn set(&mut self, x: usize, y: usize, c: char) -> Option<()> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let (x, y) = self.rotate(x, y);
        let k = self.xy_to_index(x, y);
        self.map[k] = c;
        Some(())
    }

    pub fn iget(&self, x: isize, y: isize) -> Option<&char> {
        if x < 0 || y < 0 {
            return None;
        }
        self.get(x as usize, y as usize)
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&char> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let (x, y) = self.rotate(x, y);
        let k = self.xy_to_index(x, y);
        self.map.get(k)
    }

    pub fn swap(&mut self, xa: usize, ya: usize, xb: usize, yb: usize) {
        let a = *self.get(xa, ya).unwrap();
        let b = *self.get(xb, yb).unwrap();
        self.set(xa, ya, b);
        self.set(xb, yb, a);
    }

    // return position of first occurence of char, None if not found
    pub fn find(&self, sc: char) -> Option<(usize, usize)> {
        for (k, &mc) in self.map.iter().enumerate() {
            if mc == sc {
                return Some(self.index_to_xy(k));
            }
        }
        None
    }

    pub fn ifind(&self, sc: char) -> Option<(isize, isize)> {
        let (x, y) = self.find(sc)?;
        Some((x as isize, y as isize))
    }

    // count occurences of char in map
    pub fn count(&self, sc: char) -> usize {
        self.map.iter().filter(|c| sc == **c).count()
    }

    /// iterate through elements of map return element and position
    pub fn iter(&self) -> impl Iterator<Item = (char, (usize, usize))> + use<'_> {
        self.map.iter().enumerate().map(|(index, &c)| {
            // convert index to 2D position
            let xy = self.index_to_xy(index);
            // return a (char, position) tuple
            (c, xy)
        })
    }
}
