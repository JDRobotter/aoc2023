fn hash(s: &str) -> usize {
    let mut sum = 0;
    for c in s.chars() {
        let ic = c as usize;
        sum += ic;
        sum = 17 * sum;
        sum = sum % 256;
    }
    sum
}

fn main() {
    /*
        let data = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    */
    let mut data = String::new();
    let mut file = std::fs::File::open("input").unwrap();
    use std::io::Read;
    file.read_to_string(&mut data).unwrap();

    let mut sum = 0;
    let steps = data.trim().split(',');
    for step in steps {
        println!("#{step}#");
        let h = hash(step);
        sum += h;
    }

    println!("{}", sum);
}
