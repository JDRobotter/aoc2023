use std::collections::BTreeMap;

struct MapJunction {
    pub left: String,
    pub right: String,
}

fn main() {
    /*
    let data = "LR

    11A = (11B, XXX)
    11B = (XXX, 11Z)
    11Z = (11B, XXX)
    22A = (22B, XXX)
    22B = (22C, 22C)
    22C = (22Z, 22Z)
    22Z = (22B, 22B)
    XXX = (XXX, XXX)";
    */
    let mut file = std::fs::File::open("input").unwrap();
    let mut data = String::new();
    use std::io::Read;
    file.read_to_string(&mut data).unwrap();

    let mut map = BTreeMap::new();

    let mut lines = data.split('\n');

    let path = lines.next().unwrap().trim();

    for line in lines {
        let line = line.trim();

        if line.len() == 0 {
            continue;
        }

        let mut line = line.split('=');

        // unpack junction name
        let name = line.next().unwrap().trim();
        // unpack left,right junctions
        let lr = line.next().unwrap().trim();
        let lr = lr.strip_prefix('(').unwrap();
        let lr = lr.strip_suffix(')').unwrap();

        let mut lr = lr.split(',');

        let left = lr.next().unwrap().trim();
        let right = lr.next().unwrap().trim();

        let mj = MapJunction {
            left: String::from(left),
            right: String::from(right),
        };

        map.insert(String::from(name), mj);
    }

    let mut positions = vec![];
    for location in map.keys() {
        println!("{location}");
        if location.ends_with("A") {
            positions.push(location);
        }
    }

    println!("{path:?}");
    println!("{positions:?}");
    let mut cycles = vec![];
    for pos in &mut positions {
        let mut t = 0i128;
        for instruction in path.chars().cycle() {
            // fetch junction
            let junction = map.get(*pos).unwrap();
            // apply instruction
            *pos = match instruction {
                'L' => &junction.left,
                'R' => &junction.right,
                _ => panic!(),
            };

            t += 1;

            if pos.ends_with('Z') {
                break;
            }
        }
        cycles.push(t);
    }

    println!("{cycles:?}");

    let r = cycles.iter().fold(1, |a, b| num::integer::lcm(a, *b));

    println!("{r:?}");

    /*
        for position in positions {
            println!("{start} {end}");
            let steps = walk_path(&path, start, &end, &map);
            println!(" --> {steps}");
        }
    */
}
