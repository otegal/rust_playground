struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String,
}

impl Person {
    fn print(&self) {
        println!("{}: {} ({}) in {}", self.id, self.name, self.age, self.addr);
    }
    fn print_t(&self, private: bool) {
        if private == true {
            println!("{}: {}", self.id, self.name);
        } else {
            println!("{}: {} ({}) in {}", self.id, self.name, self.age, self.addr);
        }
    }
    fn to_str(&self) -> String {
        let s = format!("{}: {} ({}) in {}",  self.id, self.name, self.age, self.addr);
        s
    }
    fn add_age(&mut self, n: i32) {
        self.age += n;
    }
}

pub fn practice() {
    let mut pa = Person {
        id: 100,
        name: String::from("hh"),
        age: 50,
        addr: String::from("tokyo"),
    };
    print_person(&pa);
    add_age(&mut pa);
    print_person(&pa);

    let ps2 = new_person(200, "ii");
    print_person(&pa);

    let pa = new_person(100, "hh");
    let pa2 = new_person(200, "ii");
    let mut people = vec![pa, pa2];
    people.push(new_person(200, "jj"));
    people.push(new_person(200, "kk"));
    for p in &people {
        print_person(p);
    }

    // method section
    let mut pa = Person {
        id: 100,
        name: String::from("hh"),
        age: 50,
        addr: String::from("tokyo"),
    };
    pa.print();
    pa.print_t(true);
    pa.print_t(false);
    let s = pa.to_str();
    println!("s is {}", s);

    pa.print();
    pa.add_age(1);
    pa.print();
}

fn new_person(id: i32, name:&str) -> Person {
    let pa = Person {
        id: id,
        name: name.to_string(),
        age: -1,
        addr: String::from("unknown"),
    };
    pa
}

fn print_person(pa: &Person) {
    println!("{}: {} ({}) in {}", pa.id, pa.name, pa.age, pa.addr);
}

fn add_age(pa: &mut Person) {
    pa.age += 1;
}
