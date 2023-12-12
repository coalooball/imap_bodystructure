pub use nom::IResult;
use nom::{
    bytes::complete::{tag, tag_no_case, take_till},
    combinator::map,
    multi::many1,
    sequence::{delimited, tuple},
};

pub fn head_bodystructure(s: &[u8]) -> IResult<&[u8], &[u8]> {
    map(
        tuple((tag_no_case(b"BODYSTRUCTURE"), many1(tag(b" ")))),
        |(a, _)| a,
    )(s)
}

fn is_double_quote(s: u8) -> bool {
    s == 0x22
}

pub fn double_quoted_string(s: &[u8]) -> IResult<&[u8], &[u8]> {
    delimited(tag(b"\""), take_till(is_double_quote), tag(b"\""))(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_head_bodystructure_1() {
        assert_eq!(head_bodystructure(br#"BODYSTRUCTURE (("text" "html" ("charset" "utf-8") NIL NIL "base64" 1188 16 NIL NIL NIL NIL) "mixed" ("boundary" "===============1522363357941492443==") NIL NIL NIL)"#), 
        Ok((br#"(("text" "html" ("charset" "utf-8") NIL NIL "base64" 1188 16 NIL NIL NIL NIL) "mixed" ("boundary" "===============1522363357941492443==") NIL NIL NIL)"#.as_ref(), b"BODYSTRUCTURE".as_ref())));
    }

    #[test]
    fn test_double_quoted_string_1() {
        assert_eq!(double_quoted_string(br#""something""#), Ok((b"".as_ref(), b"something".as_ref())));
    }
}
