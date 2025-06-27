pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub trait Bar: Sized {
    const X: u64 = 0;

    fn default_impl1(self) -> u64 {
        Self::X
    }

    fn default_impl2() -> u64 {
        2
    }

    fn default_impl3() -> u64 {
        3
    }
}
