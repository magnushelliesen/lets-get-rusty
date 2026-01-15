fn main() {
    let x = f();
    x(2);
}

fn g(x: i32) {
    println!("{}", x);
}

fn f() -> fn(i32) {
    g
}
