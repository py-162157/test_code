struct test {
    a: usize,
}

impl test {
    fn new() -> test {
        test {
            a:1,
        }
    }

    fn function1(&mut self) {
        self.a += 1;
    }

    fn function2(&mut self) {
        self.a += 1;
    }
}

fn main() {
    let mut x = test::new();
    x.function1();
    x.function2();
    print!("{}", x.a);
}
