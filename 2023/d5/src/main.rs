use std::collections::HashMap;

fn numbers_from_str(s: &str) -> Vec<i64> {
    s.split(' ')
        .filter_map(|s| i64::from_str_radix(s, 10).ok())
        .collect()
}

#[derive(Debug)]
struct SourceDestinationMap {
    source: String,
    destination: String,
    converters: Vec<ConverterRange>,
}

impl SourceDestinationMap {
    pub fn new(source: String, destination: String) -> Self {
        Self {
            source,
            destination,
            converters: Vec::new(),
        }
    }
    pub fn source(&self) -> &String {
        &self.source
    }

    pub fn destination(&self) -> &String {
        &self.destination
    }

    pub fn add_range(&mut self, didx: i64, sidx: i64, range: i64) {
        self.converters.push(ConverterRange::new(didx, sidx, range))
    }

    pub fn convert(&self, idx: i64) -> i64 {
        for conv in &self.converters {
            if let Some(idx) = conv.convert(idx) {
                return idx;
            }
        }
        return idx;
    }
}

#[derive(Debug)]
struct ConverterRange {
    didx: i64,
    sidx: i64,
    range: i64,
}

impl ConverterRange {
    pub fn new(didx: i64, sidx: i64, range: i64) -> Self {
        Self { didx, sidx, range }
    }

    pub fn convert(&self, idx: i64) -> Option<i64> {
        let delta = idx - self.sidx;
        if delta >= 0 && delta < self.range {
            Some(self.didx + delta)
        } else {
            None
        }
    }
}

fn main() {
    /*
        let data = "seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48

    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15

    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4

    water-to-light map:
    88 18 7
    18 25 70

    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13

    temperature-to-humidity map:
    0 69 1
    1 0 69

    humidity-to-location map:
    60 56 37
    56 93 4";
        */

    let mut data = String::from("");
    use std::io::Read;
    std::fs::File::open("input")
        .unwrap()
        .read_to_string(&mut data)
        .unwrap();

    let mut lines = data.split('\n');

    // -- extract seeds
    let seeds = lines.next().unwrap();

    let seeds = seeds.strip_prefix("seeds:").unwrap();
    let seeds = numbers_from_str(seeds);

    // pop an empty line
    lines.next().unwrap();

    let mut maps = HashMap::new();

    // -- extract maps
    let mut run = true;
    while run {
        // A-to-B map:
        let line = lines.next();

        if line.is_none() {
            run = false;
            break;
        }
        let line = line.unwrap();

        println!("L {line}");
        let line = line.strip_suffix(" map:").unwrap();
        let mut ab = line.split("-to-");
        let source = ab.next().unwrap();
        let destination = ab.next().unwrap();

        let mut sdmap = SourceDestinationMap::new(String::from(source), String::from(destination));

        // range lines
        loop {
            let line = lines.next();
            println!("{line:?}");
            if let Some(line) = line {
                if line.len() == 0 {
                    // empty line poped
                    break;
                }
                let vs = numbers_from_str(line);
                println!("{line:?} {vs:?}");
                sdmap.add_range(vs[0], vs[1], vs[2]);
            } else {
                // EOF
                run = false;
                break;
            }
        }

        maps.insert(String::from(source), sdmap);
    }
    println!("{maps:?}");

    // for each initial seed
    let mut locs = vec![];
    for seedidx in seeds.iter() {
        let mut idx = *seedidx;
        let mut source = String::from("seed");
        while source != "location" {
            let sdmap = maps.get(&source).unwrap();
            idx = sdmap.convert(idx);
            source = sdmap.destination().clone();
        }
        println!("{seedidx} -> {idx}");
        locs.push(idx);
    }

    let minloc = locs.iter().min().unwrap();
    dbg!(minloc);
}
