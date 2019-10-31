trait Shape {
    fn area(&self) -> f64;
    fn double(&mut self) -> ();
}

trait Nameable {
    fn name(&self) -> String;
}

#[derive(Copy,Clone,Debug)]
struct Circle {
    r: f64
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI*self.r*self.r
    }

    fn double(&mut self) -> () {
        self.r *= 2.0_f64.sqrt();
    }
}

impl Nameable for Circle {
    fn name(&self) -> String {
        "Circle".to_string()
    }
}

#[derive(Copy,Clone,Debug)]
struct Rectangle {
    l: f64,
    w: f64
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.l*self.w
    }
    
    fn double(&mut self) -> () {
        self.l *= 2.0_f64.sqrt();
        self.w *= 2.0_f64.sqrt();
    }
}

impl Nameable for Rectangle {
    fn name(&self) -> String {
        "Rectangle".to_string()
    }
}

fn print_shape<T: Shape + Nameable>(s: &T) {
    println!("{} of area {}mm^2", s.name(), s.area());
}

fn main() {
    let mut x = Circle { r: 1.0 };
    let mut y = Rectangle { l: 2.0, w: 3.0 };
    print_shape(&x);
    print_shape(&y);
    let v: Vec<&mut Shape> = vec![&mut x, &mut y];
    for i in v {
        i.double();
    } // v ends
    print_shape(&x);
    print_shape(&y);
}
