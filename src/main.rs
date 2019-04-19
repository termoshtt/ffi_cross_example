extern "C" {
    fn foo_c() -> i32;
}

fn main() {
    println!("Fooo: {}", unsafe { foo_c() });
}
