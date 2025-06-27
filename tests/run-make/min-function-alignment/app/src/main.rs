pub fn sub(left: u64, right: u64) -> u64 {
    left - right
}

struct X;

impl dep::Bar for X {
    const X: u64 = 42;

    fn default_impl3() -> u64 {
        4
    }
}

fn main() {
    println!("main {}", main as usize);
    println!("sub {}", sub as usize);
    println!("dep::add {}", dep::add as usize);
    println!("X::default_impl1 {}", <X as dep::Bar>::default_impl1 as usize);
    println!("X::default_impl2 {}", <X as dep::Bar>::default_impl2 as usize);
    println!("X::default_impl3 {}", <X as dep::Bar>::default_impl3 as usize);
}
