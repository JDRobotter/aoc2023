use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    io::Read,
    time::Duration,
};
use utils::asciimap::AsciiMap;

fn main() -> Result<(), std::io::Error> {
    let mut input = "".into();
    std::fs::File::open("input")?.read_to_string(&mut input)?;
    //dbg!(puzzle1(&input));
    //dbg!(puzzle2(&input));
    Ok(())
}

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
enum Dir {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct CrucibleState {
    pub x: isize,
    pub y: isize,
    pub dir: Dir,
}

impl CrucibleState {
    fn new(x: isize, y: isize, dir: Dir) -> Self {
        Self { x, y, dir }
    }

    /// Return grid distance to provided position
    fn distance(&self, x: isize, y: isize) -> usize {
        (isize::abs(x - self.x) + isize::abs(y - self.y)) as usize
    }

    /// turn crucible by x increments of 90° (+1) to turn right (-1) to turn left
    fn rotate(mut self, x: isize) -> Self {
        let x = x as i8;
        // enum are declared in order of positive rotations
        let n = if x >= 0 {
            u8::wrapping_add(self.dir as u8, x as u8) % 4
        } else {
            u8::wrapping_sub(self.dir as u8, -x as u8) % 4
        };
        // SAFETY: n is ensured to be in range of enum
        self.dir = unsafe { std::mem::transmute(n) };

        self
    }

    /// advance forward by 1
    fn advance(mut self) -> Self {
        match self.dir {
            Dir::Left => self.x -= 1,
            Dir::Right => self.x += 1,
            Dir::Up => self.y -= 1,
            Dir::Down => self.y += 1,
        }
        self
    }

    /// return heat at current state
    fn heat(&self, map: &AsciiMap) -> Option<usize> {
        let c = map.iget(self.x, self.y)?;
        Some(c.to_digit(10).unwrap() as usize)
    }
}

#[derive(Debug, Copy, Clone)]
enum CrucibleType {
    Normal,
    Ultra,
}

#[derive(Debug, Copy, Clone)]
struct Node {
    pub state: CrucibleState,
    pub heat: usize,
}

impl Node {
    fn new(state: CrucibleState, heat: usize) -> Self {
        Self { state, heat }
    }

    /// return a node advanced with updated heat, return None if node is out of bounds
    fn advance(&self, map: &AsciiMap) -> Option<Self> {
        let ns = self.state.advance();
        Some(Self::new(ns, self.heat + ns.heat(&map)?))
    }

    /// return a node rotated then advanced with updated heat, return None if node is out of bounds
    fn rotate_advance(&self, r: isize, map: &AsciiMap) -> Option<Self> {
        let ns = self.state.rotate(r).advance();
        Some(Self::new(ns, self.heat + ns.heat(&map)?))
    }

    /// return a vec of reachable nodes
    fn reachable(&self, map: &AsciiMap, ctype: CrucibleType) -> Vec<Node> {
        match ctype {
            CrucibleType::Normal => self.reachable_normal(map),
            CrucibleType::Ultra => self.reachable_ultra(map),
        }
    }

    fn reachable_normal(&self, map: &AsciiMap) -> Vec<Node> {
        let mut nodes = vec![];
        let mut next_node = self.clone();
        for _ in 0..3 {
            if let Some(node) = next_node.rotate_advance(-1, &map) {
                nodes.push(node);
            }
            if let Some(node) = next_node.rotate_advance(1, &map) {
                nodes.push(node);
            }
            if let Some(node) = next_node.advance(&map) {
                next_node = node;
            } else {
                break;
            }
        }

        nodes
    }

    fn reachable_ultra(&self, map: &AsciiMap) -> Vec<Node> {
        let mut nodes = vec![];
        let mut next_node = self.clone();

        for _ in 0..4 {
            if let Some(node) = next_node.advance(&map) {
                next_node = node;
            } else {
                return vec![];
            }
        }
        for _ in 3..10 {
            if let Some(node) = next_node.rotate_advance(-1, &map) {
                nodes.push(node);
            }
            if let Some(node) = next_node.rotate_advance(1, &map) {
                nodes.push(node);
            }
            if let Some(node) = next_node.advance(&map) {
                next_node = node;
            } else {
                break;
            }
        }

        nodes
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.heat.partial_cmp(&other.heat).map(|o| o.reverse())
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.heat.cmp(&other.heat).reverse()
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.heat.eq(&other.heat)
    }
}

impl Eq for Node {}

struct LavaMap {
    map: AsciiMap,
}

impl LavaMap {
    fn new(map: AsciiMap) -> Self {
        Self { map }
    }

    fn from_input(input: &str) -> Self {
        let map = AsciiMap::from_multi_lines(input);
        Self::new(map)
    }

    fn solve(&mut self, ctype: CrucibleType) -> usize {
        let mut visited: HashSet<CrucibleState> = HashSet::new();

        // create a priority queue based on heat value
        let mut open: BinaryHeap<Node> = BinaryHeap::new();

        let target = (
            self.map.width() as isize - 1,
            self.map.height() as isize - 1,
        );

        // crucible starts in
        // top-left corner facing right
        // or top-left corner facing down
        // NOTE: starting tile heat is not taken into account
        open.push(Node::new(CrucibleState::new(0, 0, Dir::Right), 0));
        open.push(Node::new(CrucibleState::new(0, 0, Dir::Down), 0));
        self.map.print();

        // iterate as long there is open nodes to explore
        while !open.is_empty() {
            let node = open.pop().unwrap();
            eprintln!("{node:?}");
            // check if we reached target
            if node.state.distance(target.0, target.1) == 0 {
                // return accumulated heat + heat of current state
                return node.heat;
            }

            // check if node was already visited
            if visited.contains(&node.state) {
                continue;
            }

            // iterate through each reachable node
            for next in node.reachable(&self.map, ctype) {
                // insert node in open list
                open.push(next);
            }

            visited.insert(node.state);
        }

        todo!()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn rotations() {
        let mut node = CrucibleState::new(0, 0, Dir::Right);
        node = node.rotate(1);
        assert_eq!(node.dir, Dir::Down);
        node = node.rotate(1);
        assert_eq!(node.dir, Dir::Left);
        node = node.rotate(1);
        assert_eq!(node.dir, Dir::Up);
        node = node.rotate(1);
        assert_eq!(node.dir, Dir::Right);

        let mut node = CrucibleState::new(0, 0, Dir::Right);
        node = node.rotate(4);
        assert_eq!(node.dir, Dir::Right);

        let mut node = CrucibleState::new(0, 0, Dir::Right);
        node = node.rotate(-1);
        assert_eq!(node.dir, Dir::Up);
        node = node.rotate(-1);
        assert_eq!(node.dir, Dir::Left);
        node = node.rotate(-1);
        assert_eq!(node.dir, Dir::Down);
        node = node.rotate(-1);
        assert_eq!(node.dir, Dir::Right);
    }

    #[test]
    fn advances() {
        let mut node = CrucibleState::new(0, 0, Dir::Right);
        node = node.advance();
        assert_eq!(node, CrucibleState::new(1, 0, Dir::Right));
        node = node.advance();
        assert_eq!(node, CrucibleState::new(2, 0, Dir::Right));

        let mut node = CrucibleState::new(0, 0, Dir::Down);
        node = node.advance();
        assert_eq!(node, CrucibleState::new(0, 1, Dir::Down));
    }

    #[test]
    fn reachable() {
        let input = "111\n111\n111\n111\n111";
        let map = AsciiMap::from_multi_lines(input);

        let node = Node::new(CrucibleState::new(1, 1, Dir::Down), 10);
        let rs = node.reachable(&map, CrucibleType::Normal);
        assert_eq!(rs[0], Node::new(CrucibleState::new(2, 1, Dir::Right), 11));
        assert_eq!(rs[1], Node::new(CrucibleState::new(0, 1, Dir::Left), 11));
        assert_eq!(rs[2], Node::new(CrucibleState::new(2, 2, Dir::Right), 12));
        assert_eq!(rs[3], Node::new(CrucibleState::new(0, 2, Dir::Left), 12));
        assert_eq!(rs[4], Node::new(CrucibleState::new(2, 3, Dir::Right), 13));
        assert_eq!(rs[5], Node::new(CrucibleState::new(0, 3, Dir::Left), 13));
    }

    #[test]
    fn heat() {
        let input = "123\n456";
        let map = AsciiMap::from_multi_lines(input);
        assert_eq!(CrucibleState::new(0, 0, Dir::Down).heat(&map), Some(1));
        assert_eq!(CrucibleState::new(1, 0, Dir::Down).heat(&map), Some(2));
        assert_eq!(CrucibleState::new(0, 1, Dir::Down).heat(&map), Some(4));
        assert_eq!(CrucibleState::new(2, 1, Dir::Down).heat(&map), Some(6));
        assert_eq!(CrucibleState::new(3, 1, Dir::Down).heat(&map), None);
        assert_eq!(CrucibleState::new(0, 2, Dir::Down).heat(&map), None);
    }

    #[test]
    fn example1() {
        let input = "
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        let loss = LavaMap::from_input(input.trim()).solve(CrucibleType::Normal);
        assert_eq!(loss, 102);
    }

    #[test]
    fn example1_easy() {
        let input = "
1199999999999
9111991119999
9991111919999
9999999119999
9999999199999
9999999199999
9999999119999
9999999919999
9999999919999
9999999911119
9999999999919
9999999999919
9999999999911";
        let loss = LavaMap::from_input(input.trim()).solve(CrucibleType::Normal);
        assert_eq!(loss, 28);
    }

    #[test]
    fn example2() {
        let input = "
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        let loss = LavaMap::from_input(input.trim()).solve(CrucibleType::Ultra);
        assert_eq!(loss, 94);
    }

    #[test]
    fn example2_part2() {
        let input = "
111111111111
999999999991
999999999991
999999999991
999999999991";
        let loss = LavaMap::from_input(input.trim()).solve(CrucibleType::Ultra);
        assert_eq!(loss, 71);
    }

    #[test]
    fn good_part1() {
        let mut input = "".into();
        std::fs::File::open("input")
            .unwrap()
            .read_to_string(&mut input)
            .unwrap();
        let loss = LavaMap::from_input(&input.trim()).solve(CrucibleType::Normal);
        assert_eq!(loss, 928);
    }

    #[test]
    fn good_part2() {
        let mut input = "".into();
        std::fs::File::open("input")
            .unwrap()
            .read_to_string(&mut input)
            .unwrap();
        let loss = LavaMap::from_input(&input.trim()).solve(CrucibleType::Ultra);
        assert_eq!(loss, 1104);
    }
}
