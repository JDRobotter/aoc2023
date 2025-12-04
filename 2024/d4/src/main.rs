use utils::asciimap::AsciiMap;

fn check_ray(map: &AsciiMap, center: (isize, isize), increment: (isize, isize)) -> bool {
    let (mut x, mut y) = center;
    let (xi, yi) = increment;
    // look at most for 3 letters : MAS
    for c in "MAS".chars() {
        // start by moving index
        x += xi;
        y += yi;
        // if word does not match, it's not XMAS, return immediately
        if Some(&c) != map.iget(x, y) {
            return false;
        }
    }
    return true;
}

fn check_x_mas(map: &AsciiMap, center: (usize, usize)) -> Option<()> {
    let (x, y) = center;
    let x = x as isize;
    let y = y as isize;
    let ul = *map.iget(x - 1, y - 1)?;
    let ur = *map.iget(x + 1, y - 1)?;
    let dl = *map.iget(x - 1, y + 1)?;
    let dr = *map.iget(x + 1, y + 1)?;

    (((ul == 'M' && dr == 'S') || (ul == 'S' && dr == 'M'))
        && ((dl == 'M' && ur == 'S') || (dl == 'S' && ur == 'M')))
        .then_some(())
}

fn main() {
    let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    let input = std::fs::read_to_string("./input").unwrap();

    let map = AsciiMap::from_multi_lines(&input);

    // -- part one --
    // iterate through map pixels
    let mut found = 0;
    for y in 0..map.height() {
        for x in 0..map.width() {
            let c = map.get(x, y).unwrap();
            if *c == 'X' {
                // only 'X' may start 'XMAS'
                // throw rays around looking for XMAS
                for increment in [
                    (1, 0),
                    (1, 1),
                    (0, 1),
                    (-1, 1),
                    (-1, 0),
                    (-1, -1),
                    (0, -1),
                    (1, -1),
                ] {
                    if check_ray(&map, (x as isize, y as isize), increment) {
                        found += 1;
                    }
                }
            }
        }
    }
    println!("P1: {found} XMAS found");

    // -- part two --
    let mut found = 0;
    for y in 0..map.height() {
        for x in 0..map.width() {
            // search for X-MAS
            // X-MAS top-left pixel is a 'M'
            let c = map.get(x, y).unwrap();
            if *c == 'A' {
                if check_x_mas(&map, (x, y)).is_some() {
                    found += 1;
                }
            }
        }
    }
    println!("P2: {found} XMAS found");
}
