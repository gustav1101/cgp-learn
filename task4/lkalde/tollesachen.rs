use std::ops;

fn main() {
    let d = tollefunktion(4, 3);
    println!("{:?}", d);
}

fn tollefunktion<T: ops::Mul + ops::Add + Copy>
    (v1: T,
     v2: T)
     -> (<T as ops::Add>::Output, <T as ops::Mul>::Output) {
    (v1 + v2, v1 * v2)
}
