trait Run {
}
struct Human {
}
impl Run for Human {
}
struct Cat {
}
impl Run for Cat {
}
fn demo(x: i32) -> impl Run {
    if x > 0 {
        Human {}
    } else {
        Cat {}
    }
}
fn main() {
}