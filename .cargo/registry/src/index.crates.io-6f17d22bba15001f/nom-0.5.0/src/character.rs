/// Character level parsers

use internal::{IResult,Needed,Err};
use util::ErrorCode;

/// matches one of the provided characters
#[macro_export]
macro_rules! one_of (
  ($i:expr, $inp: expr) => (
    {
      if $i.is_empty() {
        $crate::IResult::Incomplete($crate::Needed::Size(1))
      } else {
        #[inline(always)]
        fn as_bytes<T: $crate::AsBytes>(b: &T) -> &[u8] {
          b.as_bytes()
        }

        let expected = $inp;
        let bytes = as_bytes(&expected);
        let mut found = false;

        for &i in bytes {
          if i == $i[0] {
            found = true;
            break;
          }
        }

        if found {
          $crate::IResult::Done(&$i[1..], $i[0] as char)
        } else {
          $crate::IResult::Error($crate::Err::Position($crate::ErrorCode::OneOf as u32, $i))
        }
      }
    }
  );
);

/// matches anything but the provided characters
#[macro_export]
macro_rules! none_of (
  ($i:expr, $inp: expr) => (
    {
      if $i.is_empty() {
        $crate::IResult::Incomplete($crate::Needed::Size(1))
      } else {
        #[inline(always)]
        fn as_bytes<T: $crate::AsBytes>(b: &T) -> &[u8] {
          b.as_bytes()
        }

        let expected = $inp;
        let bytes = as_bytes(&expected);
        let mut found = false;

        for &i in bytes {
          if i == $i[0] {
            found = true;
            break;
          }
        }

        if !found {
          $crate::IResult::Done(&$i[1..], $i[0] as char)
        } else {
          $crate::IResult::Error($crate::Err::Position($crate::ErrorCode::NoneOf as u32, $i))
        }
      }
    }
  );
);

/// matches one character: `char!(char) => &[u8] -> IResult<&[u8], char>
#[macro_export]
macro_rules! char (
  ($i:expr, $c: expr) => (
    {
      if $i.is_empty() {
        $crate::IResult::Incomplete($crate::Needed::Size(1))
      } else {
        if $i[0] == $c as u8 {
          $crate::IResult::Done(&$i[1..], $i[0] as char)
        } else {
          $crate::IResult::Error($crate::Err::Position($crate::ErrorCode::Char as u32, $i))
        }
      }
    }
  );
);

named!(pub newline<char>, char!('\n'));

pub fn crlf(input:&[u8]) -> IResult<&[u8], char> {
  if input.len() < 2 {
    IResult::Incomplete(Needed::Size(2))
  } else {
    if &input[0..2] == &b"\r\n"[..] {
      IResult::Done(&input[2..], '\n')
    } else {
      IResult::Error(Err::Position(ErrorCode::CrLf as u32, input))
    }
  }
}

named!(pub eol<char>, alt!(crlf | newline));
named!(pub tab<char>, char!('\t'));

pub fn anychar(input:&[u8]) -> IResult<&[u8], char> {
  if input.is_empty() {
    IResult::Incomplete(Needed::Size(1))
  } else {
    IResult::Done(&input[1..], input[0] as char)
  }
}

#[cfg(test)]
mod tests {
  use internal::IResult::*;
  use internal::Err::*;
  use util::ErrorCode;

  #[test]
  fn one_of() {
    named!(f<char>, one_of!("ab"));

    let a = &b"abcd"[..];
    assert_eq!(f(a), Done(&b"bcd"[..], 'a'));

    let b = &b"cde"[..];
    assert_eq!(f(b), Error(Position(ErrorCode::OneOf as u32, b)));
  }

  #[test]
  fn none_of() {
    named!(f<char>, none_of!("ab"));

    let a = &b"abcd"[..];
    assert_eq!(f(a), Error(Position(ErrorCode::NoneOf as u32, a)));

    let b = &b"cde"[..];
    assert_eq!(f(b), Done(&b"de"[..], 'c'));
  }

  #[test]
  fn char() {
    named!(f<char>, char!('c'));

    let a = &b"abcd"[..];
    assert_eq!(f(a), Error(Position(ErrorCode::Char as u32, a)));

    let b = &b"cde"[..];
    assert_eq!(f(b), Done(&b"de"[..], 'c'));
  }
}
