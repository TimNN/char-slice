//! This crate provides the `CharSlice` trait which is implemented for `&str`'s
//! and provides the `char_slice` method, which allows extracting a sequence of
//! unicode codepoints from a slice.

use std::{ops, usize};

/// This trait provides the `char_slice` method.
///
/// `R` should be one of the ranges in `std::ops`.
pub trait CharSlice<R> {
    /// Returns a substring of self specified by the given range.
    ///
    /// In case of invalid input, this method will return the empty string.
    fn char_slice(&self, r: R) -> &str;
}

impl CharSlice<ops::RangeFull> for str {
    #[inline]
    fn char_slice(&self, _: ops::RangeFull) -> &str {
        self
    }
}

impl CharSlice<ops::RangeTo<usize>> for str {
    #[inline]
    fn char_slice(&self, r: ops::RangeTo<usize>) -> &str {
        self.char_slice(0..r.end)
    }
}

impl CharSlice<ops::RangeFrom<usize>> for str {
    #[inline]
    fn char_slice(&self, r: ops::RangeFrom<usize>) -> &str {
        self.char_slice(r.start..usize::MAX)
    }
}

impl CharSlice<ops::Range<usize>> for str {
    #[inline]
    fn char_slice(&self, r: ops::Range<usize>) -> &str {
        char_slice(self, r.start, r.end)
    }
}

#[inline(always)]
// Returns `true` if `b` is the start of (or a complete) utf8 codepoint
fn utf8_start_byte(b: u8) -> bool {
    b < 128 || b >= 192
}

fn char_slice(s: &str, start: usize, end: usize) -> &str {
    if end <= start {
        return "";
    }

    let mut bidx = 0; // byte index
    let mut cidx = 0; // char index

    let mut start_idx = 0;

    for b in s.bytes() {
        if utf8_start_byte(b) {
            if cidx == start {
                start_idx = bidx;
            }

            if cidx == end {
                return &s[start_idx..bidx];
            }

            cidx += 1;
        }

        bidx += 1;
    }

    // did not find start
    if start >= cidx {
        return "";
    }

    // did find start but not end
    &s[start_idx..]
}

#[test]
fn substr_test() {
    assert_eq!("".char_slice(0..0), "");
    assert_eq!("".char_slice(0..1), "");
    assert_eq!("a".char_slice(1..2), "");
    assert_eq!("a".char_slice(0..1), "a");
    assert_eq!("a".char_slice(0..2), "a");
    assert_eq!("a".char_slice(0..0), "");
    assert_eq!("ab".char_slice(0..1), "a");
    assert_eq!("ab".char_slice(1..2), "b");
    assert_eq!("ab".char_slice(0..2), "ab");

    assert_eq!("äöü".char_slice(0..0), "");
    assert_eq!("äöü".char_slice(4..5), "");
    assert_eq!("äöü".char_slice(0..1), "ä");
    assert_eq!("äöü".char_slice(1..2), "ö");
    assert_eq!("äöü".char_slice(2..3), "ü");
    assert_eq!("äöü".char_slice(0..2), "äö");
    assert_eq!("äöü".char_slice(1..3), "öü");
    assert_eq!("äöü".char_slice(0..3), "äöü");
    assert_eq!("äöü".char_slice(0..4), "äöü");
}
