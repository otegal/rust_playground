use std::thread;
use std::time::Duration;

struct Philosopher {
    name: String,
}

impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
        }
    }

    fn eat(&self) {
        println!("{} is eating.", &self.name);
        thread::sleep(Duration::from_millis(1000));
        println!("{} is done eating.", &self.name);
    }
}

fn main() {
    let philosophers = vec![
        Philosopher::new("hikakin"),
        Philosopher::new("seikin"),
        Philosopher::new("dekakin"),
        Philosopher::new("chibikin"),
        Philosopher::new("kin"),
    ];

    for p in philosophers {
        p.eat();
    }
}