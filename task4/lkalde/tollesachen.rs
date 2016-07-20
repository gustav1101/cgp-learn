fn main() {
    let b = 4;
    let c = 3;
    let d = tollefunktion(b, c);
    println!("{:?}", d);
}

fn tollefunktion<T: std::ops::Mul + std::ops::Add + Copy>
    (v1: T,
     v2: T)
     -> (<T as std::ops::Add>::Output, <T as std::ops::Mul>::Output) {
    (v1 + v2, v1 * v2)
}
