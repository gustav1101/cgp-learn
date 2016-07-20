struct Fib {
    next: usize,
    cur: usize,
}

impl Fib {
    fn new() -> Fib {
        Fib { next: 1, cur: 0 }
    }
}



impl Iterator for Fib {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {

        let temp = self.cur;
        self.cur = self.next;
        self.next = temp + self.next;

        Some(temp)
    }
}


fn main() {
    for i in Fib::new().take(20) {
        println!("{}", i);
    }
}
