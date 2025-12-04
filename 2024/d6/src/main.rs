use std::thread;
use std::time::Duration;
use utils::asciimap::AsciiMap;

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn xy(&self) -> (isize, isize) {
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
        }
    }

    fn rotate_cw(&self) -> Self {
        match self {
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
        }
    }

    fn mark(&self) -> char {
        match self {
            Direction::Left => '<',
            Direction::Right => '>',
            Direction::Up => '^',
            Direction::Down => 'v',
        }
    }
}

fn any_in_ray(
    map: &AsciiMap,
    pos: (isize, isize),
    velocity: (isize, isize),
    c: char,
) -> Option<(isize, isize)> {
    let (mut xp, mut yp) = pos;
    let (xv, yv) = velocity;
    while let Some(&x) = map.iget(xp, yp) {
        // wall
        if x == '#' {
            return None;
        }

        // searched char
        if x == c {
            return Some((xp, yp));
        }

        xp += xv;
        yp += yv;
    }
    None
}

fn throw_ray(
    map: &mut AsciiMap,
    marks: &mut AsciiMap,
    pos: &mut (isize, isize),
    velocity: Direction,
) -> bool {
    let (xv, yv) = velocity.xy();
    let (mut xp, mut yp) = pos;
    loop {
        let x = map.iget(xp, yp);
        match x {
            Some('#') => {
                // wall reached, end ray, continue throwing
                return true;
            }
            Some(_) => {
                // advance
            }
            None => {
                // OOB
                return false;
            }
        }
        // throw rays around to find blockage
        let nv = velocity.rotate_cw();
        if let Some((_, _)) = any_in_ray(&map, (xp, yp), nv.xy(), nv.mark()) {
            let (xv, yv) = velocity.xy();
            marks.iset(xp + xv, yp + yv, 'O');
        }
        // update position if terrain is clear
        *pos = (xp, yp);
        // mark position based on current velocity
        let mark = velocity.mark();
        map.iset(xp, yp, mark);
        // advance vector
        xp += xv;
        yp += yv;
    }
}

fn main() -> std::io::Result<()> {
    let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    let input = std::fs::read_to_string("input")?;

    let mut map = AsciiMap::from_multi_lines(input);
    println!("{} {}", map.width(), map.height());
    let mut marks = map.clone();

    let mut guard = map.ifind('^').unwrap();
    let mut velocity = Direction::Up;
    while throw_ray(&mut map, &mut marks, &mut guard, velocity) {
        // obstacle reached, turn right
        velocity = velocity.rotate_cw();
        //map.print();
        //marks.print();
        //thread::sleep(Duration::from_secs(1));
    }
    //map.print();

    let count: usize = ['>', '<', '^', 'v'].iter().map(|c| map.count(*c)).sum();
    println!("P1 = {}", count);
    let count: usize = marks.count('O');
    println!("P2 = {}", count);

    //marks.print();

    Ok(())
}
