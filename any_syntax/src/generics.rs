struct Rectange {
    width: f32,
    height: f32,
}

struct Triangle {
    base: f32,
    height: f32,
}

struct Circle {
    radius: f32
}

trait CalcArea {
    fn calc_area(&self) -> f32;
}

impl CalcArea for Rectange {
    fn calc_area(&self) -> f32 {
        self.width * self.height
    }
}

impl CalcArea for Triangle {
    fn calc_area(&self) -> f32 {
        self.base * self.height * 0.5
    }
}

impl CalcArea for Circle {
    fn calc_area(&self) -> f32 {
        self.radius * self.radius * 3.14
    }
}

pub fn practice() {
    let v = [10, 20, 30, 40, 50];
    print(&v);
    let v = ['A', 'B', 'C', 'D', 'E'];
    print(&v);
    let v = ["one", "two", "three", "four", "five"];
    print(&v);

    let rect = Rectange {
        width: 10.0,
        height: 20.0,
    };
    let tri = Triangle {
        base: 10.0,
        height: 20.0,
    };
    let cir = Circle {
        radius: 10.0
    };
    println!("rect area is {}", rect.calc_area());
    println!("tri area is {}", tri.calc_area());
    println!("cir area is {}", cir.calc_area());
}

fn print<T>(a: &[T]) where T: std::fmt::Debug {
    print!("a is ");
    for i in a {
        print!("{:?} ", i);
    }
    println!("");
}
