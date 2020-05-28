pub use percent_encoding::{AsciiSet, CONTROLS, NON_ALPHANUMERIC};

pub const FRAGMENT: AsciiSet = NON_ALPHANUMERIC
    .remove(b'*')
    .remove(b'-')
    .remove(b'.')
    .remove(b'_');

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {}
}
