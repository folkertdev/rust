#[lang = "complex"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[unstable(feature = "complex_numbers", issue = "154023")]
#[repr(complex)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

#[unstable(feature = "complex_numbers", issue = "154023")]
impl<T> Complex<T> {
    #[must_use]
    pub fn new(re: T, im: T) -> Complex<T> {
        Complex { re, im }
    }
}
