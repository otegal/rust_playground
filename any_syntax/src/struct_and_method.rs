struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String,
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
