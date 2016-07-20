use std::fmt;

struct Swagger<T: fmt::Display> {
    disp: T,
}

impl<T: fmt::Display> fmt::Display for Swagger<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#swag {} #yolo", self.disp)
    }
}

fn main() {
    let lulz = Swagger { disp: "ololol".to_string() };
    println!("Swagger says {}", lulz);
}
