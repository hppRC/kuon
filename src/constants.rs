pub use percent_encoding::{AsciiSet, CONTROLS, NON_ALPHANUMERIC};

pub const FRAGMENT: AsciiSet = NON_ALPHANUMERIC
    .remove(b'*')
    .remove(b'-')
    .remove(b'.')
    .remove(b'_');
