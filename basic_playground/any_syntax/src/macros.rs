// #[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String,
}

impl std::fmt::Debug for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ {}: `{}` in {} }}", self.id, self.name, self.addr)
    }
}


pub fn practice() {
    let a =  [1, 2, 3, 4, 5];
    dbg!(a);

    let p = Person {
        id: 100,
        name: String::from("hh"),
        age: 30,
        addr: String::from("Tokyo"),
    };
    dbg!(&p);
    println!("p is {:?}", &p);

}