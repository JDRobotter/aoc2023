#[derive(Debug)]
struct Set {
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

impl Set {
    pub fn check(&self, br: u32, bg: u32, bb: u32) -> bool {
        (self.r <= br) && (self.g <= bg) && (self.b <= bb)
    }

    pub fn power(&self) -> u32 {
        self.r * self.g * self.b
    }
}

pub fn minimum_set(vset: &Vec<Set>) -> Set {
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    for set in vset {
        r = u32::max(r, set.r);
        g = u32::max(g, set.g);
        b = u32::max(b, set.b);
    }

    Set { r, g, b }
}

fn main() {
    /*
    let data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    */
    let mut data = String::new();
    use std::io::Read;
    std::fs::File::open("input")
        .unwrap()
        .read_to_string(&mut data)
        .unwrap();

    let mut sum = 0;
    for line in data.split('\n') {
        if line.len() == 0 {
            continue;
        }
        let mut line = line.split(':');

        // extract game number
        let game = line.next().unwrap().trim();
        let game = game.strip_prefix("Game ").unwrap().trim();
        let game = u32::from_str_radix(game, 10).unwrap();
        println!("G {game}");

        // extract sets
        let sets = line.next().unwrap();
        let mut csets = vec![];
        for set in sets.split(';') {
            let mut cset = Set { r: 0, g: 0, b: 0 };
            for color in set.split(',') {
                let color = color.trim();
                let mut color = color.split(' ');

                let n = color.next().unwrap();
                let n = u32::from_str_radix(n, 10).unwrap();
                let color = color.next().unwrap();

                match color {
                    "blue" => cset.b = n,
                    "red" => cset.r = n,
                    "green" => cset.g = n,
                    _ => panic!("unknown color {color}"),
                }
            }
            csets.push(cset);
        }
        let mset = minimum_set(&csets);
        println!("G {game} {mset:?} {}", mset.power());
        sum += mset.power();
    }

    println!("sum = {sum}");
}
