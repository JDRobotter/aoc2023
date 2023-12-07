fn race(duration: u128, distance: u128) -> u128 {
    let mut ways = 0u128;
    // test all button press durations
    for speed in 1..duration {
        let rem = duration - speed;
        let rdistance = rem * speed;
        if rdistance > distance {
            ways += 1;
        }
    }
    ways
}

fn main() {
    println!("Hello, world!");

    let wm = race(45977295, 305106211101695);
    println!("wm = {wm}");
}
