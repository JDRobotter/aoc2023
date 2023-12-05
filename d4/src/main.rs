fn numbers_from_str(s: &str) -> Vec<u32> {
    s.split(' ')
        .filter_map(|s| u32::from_str_radix(s, 10).ok())
        .collect()
}

fn main() {
    /*
        let data = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    */
    let mut file = std::fs::File::open("input").unwrap();
    use std::io::Read;
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let mut sum: u32 = 0;
    for line in data.split('\n') {
        if line.len() == 0 {
            continue;
        }

        // -- extract card number
        let mut line = line.split(':');
        let card = line.next().unwrap();

        // -- extract winning numbers
        let line = line.next().unwrap();
        let mut line = line.split('|');
        let win = line.next().unwrap();
        let win = numbers_from_str(win);

        // -- extract own numbers
        let own = line.next().unwrap();
        let own = numbers_from_str(own);

        // -- compute card score
        let mut score = 0;
        for on in own {
            if win.contains(&on) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }
        println!("{card} {score}");
        sum += score;
    }
    println!("TOTAL {sum}");
}
