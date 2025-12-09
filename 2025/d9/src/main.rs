use std::{collections::BinaryHeap, io::Read};

fn main() -> Result<(), std::io::Error> {
    let mut input = "".into();
    std::fs::File::open("input")?.read_to_string(&mut input)?;
    //dbg!(GP::new(&input).solve1(1000));
    //dbg!(GP::new(&input).solve2());
    Ok(())
}

/// Represent an horizontal or a vertical segment used in ray casting algorithm
#[derive(Debug)]
struct HVSegment {
    a: isize,
    b0: isize,
    b1: isize,
    winding: isize,
}

impl HVSegment {
    fn new(a: isize, b0: isize, b1: isize, winding: isize) -> Self {
        Self { a, b0, b1, winding }
    }

    /// check if y coordinate is inside of segment (exclude both ends)
    fn intersect(&self, b: isize) -> bool {
        self.b0 < b && b < self.b1
    }
}

/// A rectangle struct to be used in a BinaryHeap to classify possible rectangles by surface
#[derive(Debug)]
struct Rectangle {
    /// rectangle A corner tile index
    ka: usize,
    /// rectangle B corner tile index
    kb: usize,
    /// rectangle surface
    surface: isize,
}

impl Rectangle {
    fn new(ka: usize, kb: usize, surface: isize) -> Self {
        Self { ka, kb, surface }
    }
}

impl Eq for Rectangle {}

impl PartialEq for Rectangle {
    fn eq(&self, other: &Self) -> bool {
        self.surface == other.surface
    }
}

impl PartialOrd for Rectangle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.surface.partial_cmp(&other.surface)
    }
}

impl Ord for Rectangle {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.surface.cmp(&other.surface)
    }
}

struct TileFloor {
    /// red tile list
    reds: Vec<(isize, isize)>,
    /// vertical segments in area boundary, ordered by x position
    vsegments: Vec<HVSegment>,
    /// horizontal segments in area boundary, ordered by y position
    hsegments: Vec<HVSegment>,
}

impl TileFloor {
    fn new(mut reds: Vec<(isize, isize)>) -> Self {
        // -- iterate through plan do compute vertical segments
        // append first point to plan to complete boundary
        reds.push(reds[0]);

        // assert than there is no "flat" corner in our build list
        assert!(Self::are_some_corners_colinear(&reds));

        let mut lv = (0, 0);
        let mut vsegments = vec![];
        let mut hsegments = vec![];
        for k in 0..(reds.len() - 1) {
            // get current point and next point
            let p = reds[k];
            let np = reds[k + 1];

            if p.1 == np.1 {
                // -- segment is horizontal
                let xa = p.0.min(np.0);
                let xb = p.0.max(np.0);
                let winding = (np.0 - p.0).signum(); // winding positive if segment is going right
                let vs = HVSegment::new(p.1, xa, xb, winding);

                // append segment to list ordered by y position
                let idx = match hsegments.binary_search_by(|vs: &HVSegment| vs.a.cmp(&p.1)) {
                    Ok(idx) => idx,
                    Err(idx) => idx,
                };

                hsegments.insert(idx, vs);
            } else {
                // -- segment is vertical
                let ya = p.1.min(np.1);
                let yb = p.1.max(np.1);
                let winding = (np.1 - p.1).signum(); // winding positive if segment is going up
                let vs = HVSegment::new(p.0, ya, yb, winding);

                // append segment to list ordered by x position
                let idx = match vsegments.binary_search_by(|vs: &HVSegment| vs.a.cmp(&p.0)) {
                    Ok(idx) => idx,
                    Err(idx) => idx,
                };

                vsegments.insert(idx, vs);
            }
        }

        Self {
            reds,
            vsegments,
            hsegments,
        }
    }

    fn from_input(input: &str) -> Self {
        // puzzle input is the list of red tiles positions
        let reds: Vec<(isize, isize)> = input
            .trim()
            .split('\n')
            .map(|line| {
                let mut xy = line.split(',').map(|x| x.parse::<isize>().unwrap());
                (xy.next().unwrap(), xy.next().unwrap())
            })
            .collect();

        Self::new(reds)
    }

    /// Return surface of rectangle defined by both red tiles indices
    fn surface_of(&self, aidx: usize, bidx: usize) -> isize {
        let (ax, ay) = self.reds[aidx];
        let (bx, by) = self.reds[bidx];
        return ((bx - ax).abs() + 1) * ((by - ay).abs() + 1);
    }

    /// return true if provided three points are colinear
    fn is_colinear(a: (isize, isize), b: (isize, isize), c: (isize, isize)) -> bool {
        let ab = (b.0 - a.0, b.1 - a.1);
        let bc = (c.0 - b.0, c.1 - b.1);
        // check if cross product is zero
        (ab.1 * bc.0 - ab.0 * bc.1) == 0
    }

    /// return true if at least one triple of consecutive corners are colinears,
    /// return false otherwise
    fn are_some_corners_colinear(plan: &[(isize, isize)]) -> bool {
        for i in 0..(plan.len() - 2) {
            if Self::is_colinear(plan[i], plan[i + 1], plan[i + 2]) {
                return false;
            }
        }
        return true;
    }

    fn throw_ray(&self, segments: &Vec<HVSegment>, start: (isize, isize)) -> isize {
        self.throw_ray_distance(segments, start, isize::MAX)
    }

    /// throw a ray positive a sum windings of encountered segments
    fn throw_ray_distance(
        &self,
        segments: &Vec<HVSegment>,
        (a, b): (isize, isize),
        length: isize,
    ) -> isize {
        // find first vertical segment located at provided start position
        let sidx = match segments.binary_search_by(|vs: &HVSegment| vs.a.cmp(&a)) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };

        // if binary search returned last index this ray starts
        // after all of the segments
        if sidx == segments.len() {
            return 0;
        }

        // total winding
        let mut winding = 0;
        // ray height
        let ray = b;

        for idx in sidx..segments.len() {
            let vs = &segments[idx];
            // limit ray length
            if vs.a - a > length {
                break;
            }
            // check ray if intersecting
            if vs.intersect(ray) {
                winding += vs.winding;
            }
        }

        winding
    }

    /// return true if provided rectangle (defined by is two corners indices)
    /// is fully contained inside floor boundaries
    fn is_contained(&self, ka: usize, kb: usize) -> bool {
        // get points
        let a = self.reds[ka];
        let b = self.reds[kb];

        // our rectangle need to have is left side going up when going from a to b
        let (a, b) = ((a.0.min(b.0), a.1.min(b.1)), (a.0.max(b.0), a.1.max(b.1)));

        // special case were the rectangle is a line
        if a.0 == b.0 || a.1 == b.1 {
            // always contained
            return true;
        }
        // special case were the rectangle is two by two
        assert!(b.0 - a.0 != 1 && b.1 - a.1 != 1);

        // throw ray going x positive from rectangle bottom-left corner +(1,1)
        // if returned winding is even, ray started from inside the floor
        let inside = self.throw_ray(&self.vsegments, (a.0 + 1, a.1 + 1)) % 2 != 0;
        if !inside {
            // if this point is outside of the floor, the rectangle could not be fully contained inside
            return false;
        }
        //
        // b.1+-------+
        //    |      b|
        //    |a      |
        // a.1+-------+
        //    a.0     b.0
        // check if rectangle borders segments are crossed
        let vs = &self.vsegments;
        if self.throw_ray_distance(&vs, (a.0 + 1, a.1 + 1), isize::abs(b.0 - a.0) - 2) != 0 {
            return false;
        }
        if self.throw_ray_distance(&vs, (a.0 + 1, b.1 - 1), isize::abs(b.0 - a.0) - 2) != 0 {
            return false;
        }

        let hs = &self.hsegments;
        if self.throw_ray_distance(&hs, (a.1 + 1, a.0 + 1), isize::abs(b.1 - a.1) - 2) != 0 {
            return false;
        }
        if self.throw_ray_distance(&hs, (a.1 + 1, b.0 - 1), isize::abs(b.1 - a.1) - 2) != 0 {
            return false;
        }

        true
    }

    /// Return largest rectangle area without empty floor
    fn largest_rectangle_non_empty(&mut self) -> isize {
        let n = self.reds.len();

        // ---
        // create a pretty huge set of all possible rectangle and associated surfaces
        // ordered by surface
        let mut rectangles: BinaryHeap<Rectangle> = BinaryHeap::new();
        // do a strict triangular iteration through reds tiles
        // because rectangle(a,b) == rectangle(b,a) and rectangle(a,a) is empty
        for i in 0..n {
            for j in 0..i {
                rectangles.push(Rectangle::new(i, j, self.surface_of(i, j)));
            }
        }

        // ---
        // iterate through rectangles from largest surface to lowest
        for r in rectangles.iter() {
            // check if rectangle is fully contained
            if self.is_contained(r.ka, r.kb) {
                return r.surface;
            }
        }

        panic!("nothing found");
    }

    /// Return largest rectangle area formed between two opposite tiles
    fn largest_rectangle(&self) -> isize {
        let n = self.reds.len();
        let max_surface: isize = (0..n)
            // do a strict triangular iteration through reds tiles
            // because rectangle(a,b) == rectangle(b,a) and rectangle(a,a) is empty
            .map(|i| (0..i).map(move |j| self.surface_of(i, j)))
            // flatten nested levels
            .flatten()
            // compute maximum surface
            .max()
            .unwrap();

        max_surface
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use utils::asciimap::AsciiMap;

    fn ascii_map_to_plan(map: &str) -> String {
        let map = AsciiMap::from_multi_lines(map);
        let mut plan = String::new();
        let mut k = 0;
        while let Some((x, y)) = map.find(std::char::from_digit(k, 16).unwrap()) {
            plan.push_str(&format!("{x},{y}\n"));
            k += 1;
        }
        plan
    }

    #[test]
    fn example1() {
        let input = "
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"
        .trim();
        let tf = TileFloor::from_input(input);
        assert_eq!(tf.surface_of(4, 6), 24);
        assert_eq!(tf.surface_of(6, 4), 24);
        assert_eq!(tf.largest_rectangle(), 50);
    }

    #[test]
    fn example2() {
        let input = "
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"
        .trim();
        let mut tf = TileFloor::from_input(input);
        assert_eq!(tf.largest_rectangle_non_empty(), 24);
    }

    #[test]
    fn colinear_test() {
        assert_eq!(TileFloor::is_colinear((0, 0), (1, 1), (2, 2)), true);
        assert_eq!(TileFloor::is_colinear((0, 0), (0, 1), (0, 2)), true);
        assert_eq!(
            TileFloor::is_colinear((100, 100), (100, 1000), (100, 100000)),
            true
        );
        assert_eq!(TileFloor::is_colinear((0, 0), (0, 1), (1, 0)), false);
        assert_eq!(
            TileFloor::is_colinear((1000, 0), (1000, 1), (1001, 1000)),
            false
        );
    }

    #[test]
    fn contained() {
        let input = "
0,0
0,6
2,6
2,2
5,2
5,4
7,4
7,0";
        //  01234567
        //6 1-2
        //5 | |
        //4 | |  5-6
        //3 | |  | |
        //2 | 3--4 |
        //1 |      |
        //0 0------7

        let tf = TileFloor::from_input(input);
        assert_eq!(tf.is_contained(0, 4), true);
        assert_eq!(tf.is_contained(1, 4), false);
        assert_eq!(tf.is_contained(1, 7), false);
        assert_eq!(tf.is_contained(7, 1), false);
        assert_eq!(tf.is_contained(2, 4), false);
        assert_eq!(tf.is_contained(2, 5), false);
        assert_eq!(tf.is_contained(2, 6), false);
        assert_eq!(tf.is_contained(3, 7), true);
        assert_eq!(tf.is_contained(5, 7), true);
    }

    #[test]
    fn contained2() {
        let input = ascii_map_to_plan(
            "
...................
...................
...................
....7==6...3===2...
....|..|...|...|...
....|..|...|...|...
....|..|...|...|...
....0==5===4===1...
...................
...................
...................
        "
            .trim(),
        );
        eprintln!("{}", &input);
        let tf = TileFloor::from_input(&input);
        assert_eq!(tf.is_contained(0, 6), true);
        assert_eq!(tf.is_contained(7, 5), true);
        assert_eq!(tf.is_contained(1, 3), true);
        assert_eq!(tf.is_contained(2, 4), true);
        assert_eq!(tf.is_contained(7, 1), false);
        assert_eq!(tf.is_contained(0, 2), false);
    }

    #[test]
    fn contained_example2() {
        let input = "
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        //  0123456789012
        //7          3-2
        //6          | |
        //5   5------4 |
        //4   |        |
        //3   6----7   |
        //2        |   |
        //1        0---1
        //0
        //  0123456789012
        let tf = TileFloor::from_input(input);
        assert_eq!(tf.is_contained(1, 7), true);
        assert_eq!(tf.is_contained(3, 4), true);
        assert_eq!(tf.is_contained(4, 6), true);
        assert_eq!(tf.is_contained(3, 5), false);
        assert_eq!(tf.is_contained(5, 0), false);
        assert_eq!(tf.is_contained(7, 2), false);
        assert_eq!(tf.surface_of(4, 6), 24);
        assert_eq!(tf.surface_of(3, 5), 24);
    }

    #[test]
    fn good_part1() {
        let mut input = "".into();
        std::fs::File::open("input")
            .unwrap()
            .read_to_string(&mut input)
            .unwrap();
        let tf = TileFloor::from_input(&input);
        assert_eq!(tf.largest_rectangle(), 4763932976);
    }

    #[test]
    fn good_part2() {
        let mut input = "".into();
        std::fs::File::open("input")
            .unwrap()
            .read_to_string(&mut input)
            .unwrap();
        let mut tf = TileFloor::from_input(&input);
        assert_eq!(tf.largest_rectangle_non_empty(), 0);
    }
}
