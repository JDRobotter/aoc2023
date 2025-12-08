use std::{
    collections::{BTreeSet, HashSet},
    io::Read,
    str::FromStr,
};

fn main() -> Result<(), std::io::Error> {
    let mut input = "".into();
    std::fs::File::open("input")?.read_to_string(&mut input)?;
    //dbg!(puzzle1(&input));
    //dbg!(puzzle2(&input));
    Ok(())
}

#[derive(Debug, Copy, Clone)]
struct JunctionBox {
    x: isize,
    y: isize,
    z: isize,
}

impl JunctionBox {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    /// Return squared distance between self and an other box
    fn sqdist_to(&self, other: &Self) -> isize {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

#[derive(Debug, Copy, Clone)]
struct Connection {
    // boxes indexes
    a: usize,
    b: usize,
    // sqdistance between boxes
    sqdist: usize,
}

impl Connection {
    fn normalize_indices(a: usize, b: usize) -> (usize, usize) {
        // a,b will be stored so that a < b
        if a > b { (b, a) } else { (a, b) }
    }

    fn new(a: usize, b: usize, sqdist: usize) -> Self {
        // (a,b) pair connection is identical to (b,a)
        // and will never be constructed
        let (a, b) = Self::normalize_indices(a, b);
        Self { a, b, sqdist }
    }

    /// return Some(idx) if given index match this connection,
    /// (idx will be the index of the other connected box)
    /// retur None otherwise
    fn matches(&self, idx: usize) -> Option<usize> {
        if idx == self.a {
            Some(self.b)
        } else if idx == self.b {
            Some(self.a)
        } else {
            None
        }
    }
}

impl FromStr for JunctionBox {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, &'static str> {
        let mut ns = s.split(',');

        let x = ns
            .next()
            .ok_or("not enough coordinates")?
            .parse::<isize>()
            .or_else(|_| Err("unable to parse integer"))?;

        let y = ns
            .next()
            .ok_or("not enough coordinates")?
            .parse::<isize>()
            .or_else(|_| Err("unable to parse integer"))?;

        let z = ns
            .next()
            .ok_or("not enough coordinates")?
            .parse::<isize>()
            .or_else(|_| Err("unable to parse integer"))?;

        Ok(JunctionBox::new(x, y, z))
    }
}

/// Giant Playground
struct GP {
    boxes: Vec<JunctionBox>,
    connected: Vec<Connection>,
}

impl GP {
    fn new(input: &str) -> Self {
        let boxes: Vec<JunctionBox> = input
            .trim()
            .split('\n')
            .map(|line| JunctionBox::from_str(line).unwrap())
            .collect();

        Self {
            boxes,
            connected: vec![],
        }
    }

    /// Return a list of unique connections between boxes, ordered by distance
    fn connect(&mut self, limit: usize, sqdist_limit: isize) {
        let n = self.boxes.len();

        // do a strict triangular iteration through boxes
        // (d(i,j) == d(j,i) and d(i,i) == 0)
        for i in 0..n {
            for j in 0..i {
                // compute distance and update minimum
                let sqdist = self.boxes[i].sqdist_to(&self.boxes[j]);
                if sqdist > sqdist_limit {
                    continue;
                }

                // insert connection ordered by distance between boxes (closer first)
                let conn = Connection::new(j, i, sqdist as usize);
                match self
                    .connected
                    .binary_search_by(|probe| conn.sqdist.cmp(&probe.sqdist).reverse())
                {
                    Ok(idx) => self.connected.insert(idx, conn),
                    Err(idx) => self.connected.insert(idx, conn),
                }

                // limit connected list length
                if self.connected.len() > limit {
                    self.connected.pop();
                }
            }
        }
    }

    /// find boxes connected to this box and remove them from rem
    /// return number of found boxes in this circuit
    fn follow_circuit(
        &self,
        bidx: usize,
        rem: &mut BTreeSet<usize>,
        last: &mut Option<Connection>,
    ) -> usize {
        // count boxes in circuit (this one included)
        let mut nboxes = 1;

        // recurse into each connected box ordered by proximity
        for conn in self.connected.iter() {
            // if
            if let Some(idx) = conn.matches(bidx)
                && rem.contains(&idx)
            {
                // remove connected box
                rem.remove(&idx);
                let _ = last.insert(conn.clone());
                nboxes += self.follow_circuit(idx, rem, last);
            }
        }

        nboxes
    }

    fn solve1(&mut self, nconnections: usize) -> u64 {
        // -- connect closest boxes --
        self.connect(nconnections, isize::MAX);

        // -- find circuits --
        // each boxes is EXACTLY part of ONE circuit,
        // let's make a set of boxes indices and remove them one by one
        // as we progress along circuits
        let mut bidxs: BTreeSet<usize> = (0..self.boxes.len()).collect();

        let mut last_connection = None;
        let mut counts = vec![];
        while let Some(bidx) = bidxs.pop_first() {
            let n = self.follow_circuit(bidx, &mut bidxs, &mut last_connection);
            counts.push(n);
            // find each box
        }

        // multiply together the size of the 3 largest circuits
        counts.sort();
        let sum: usize = counts.iter().rev().take(3).fold(1, |a, x| a * x);
        sum as u64
    }

    #[allow(unused)]
    fn solve2(&mut self) -> u64 {
        // -- connect closest boxes --
        self.connect(1_000_000, 1_000_000_000);

        // -- find circuits --
        // each boxes is EXACTLY part of ONE circuit,
        // let's make a set of boxes indices and remove them one by one
        // as we progress along circuits
        let mut bidxs: BTreeSet<usize> = (0..self.boxes.len()).collect();

        let mut last_connection = None;
        let n = self.follow_circuit(0, &mut bidxs, &mut last_connection);

        let last_connection = last_connection.unwrap();
        // expect for ex 1 : 9 & 11
        eprintln!("last conn = {last_connection:?}");
        let boxa = dbg!(self.boxes[last_connection.a]);
        let boxb = dbg!(self.boxes[last_connection.b]);

        (boxa.x * boxb.x) as u64
    }
}
#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn example1() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        assert_eq!(GP::new(input).solve1(10), 40);
    }

    #[test]
    fn example2() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        assert_eq!(GP::new(input).solve2(), 25272);
    }

    #[test]
    fn good_part1() {
        let mut input = "".into();
        std::fs::File::open("input")
            .unwrap()
            .read_to_string(&mut input)
            .unwrap();
        assert_eq!(GP::new(&input).solve1(1000), 75680);
    }

    #[test]
    fn good_part2() {
        let mut input = "".into();
        std::fs::File::open("input")
            .unwrap()
            .read_to_string(&mut input)
            .unwrap();
        assert_eq!(GP::new(&input).solve2(), 75680);
    }
}
