struct Philosopher {
    name: String,
}

impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
        }
    }
}

fn main() {
    let p1 = Philosopher::new("hikakin");
    let p2 = Philosopher::new("seikin");
    let p3 = Philosopher::new("dekakin");
    let p4 = Philosopher::new("chibikin");
    let p5 = Philosopher::new("kin");
}