struct Schematic {
    matrix: Vec<Vec<char>>,
}

impl Schematic {
    pub fn from(s: &str) -> Self {
        let matrix = s.split('\n').map(|s| s.chars().collect()).collect();
        Self { matrix }
    }

    pub fn print(&self) {
        println!("{:?}", self.matrix);
    }

    pub fn process(&mut self:
}

fn main() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    let s = Schematic::from(input);
    s.print();
}
