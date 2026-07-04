#[lang = "complex"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[unstable(feature = "complex_numbers", issue = "154023")]
#[repr(C)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

#[must_use]
#[unstable(feature = "complex_numbers", issue = "154023")]
impl<T> Complex<T> {
    pub fn new(re: T, im: T);
}
