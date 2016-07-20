use std::ops;

fn main() {
    let b = 4;
    let c = 3;
    let d = tollefunktion(b, c);
    println!("{:?}", d);
}

fn tollefunktion<T: ops::Mul + ops::Add + Copy>
    (v1: T,
     v2: T)
     -> (<T as ops::Add>::Output, <T as ops::Mul>::Output) {
    (v1 + v2, v1 * v2)
}
