pub use nom::IResult;
use nom::{
    bytes::complete::{tag, tag_no_case, take_till},
    combinator::map,
    multi::{many1, separated_list1},
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

#[derive(Debug, PartialEq)]
pub struct Parameter<'a> {
    attribute: &'a [u8],
    value: &'a [u8],
}

impl Parameter<'_> {
    pub fn get_content_type_text(&self) -> Vec<u8> {
        let mut result = self.attribute.to_vec();
        result.extend_from_slice(b"=\"");
        result.extend(self.value.to_vec().iter());
        result.extend_from_slice(b"\"");
        result
    }
}

#[derive(Debug, PartialEq)]
pub struct Parameters<'a> {
    list: Vec<Parameter<'a>>,
}

pub fn parameter(s: &[u8]) -> IResult<&[u8], Parameter> {
    map(
        tuple((double_quoted_string, tag(b" "), double_quoted_string)),
        |(attribute, _, value)| Parameter {
            attribute: attribute,
            value: value,
        },
    )(s)
}

pub fn parameters(s: &[u8]) -> IResult<&[u8], Parameters> {
    map(separated_list1(tag(b" "), parameter), |list| Parameters {
        list: list,
    })(s)
}

#[derive(Debug, PartialEq)]
pub struct ContentTypeTypeAndSubType<'a> {
    ttype: &'a [u8],
    subtype: &'a [u8],
}

impl ContentTypeTypeAndSubType<'_> {
    pub fn get_content_type_text(&self) -> Vec<u8> {
        let mut result = self.ttype.to_vec();
        result.extend_from_slice(b"/");
        result.extend(self.subtype.to_vec().iter());
        result
    }
}

pub fn content_type_main(s: &[u8]) -> IResult<&[u8], ContentTypeTypeAndSubType> {
    map(
        tuple((double_quoted_string, tag(b" "), double_quoted_string)),
        |(ttype, _, subtype)| ContentTypeTypeAndSubType {
            ttype: ttype,
            subtype: subtype,
        },
    )(s)
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
        assert_eq!(
            double_quoted_string(br#""something""#),
            Ok((b"".as_ref(), b"something".as_ref()))
        );
    }

    #[test]
    fn test_content_type_1() {
        assert_eq!(
            content_type_main(br#""TEXT" "PLAIN""#),
            Ok((
                b"".as_ref(),
                ContentTypeTypeAndSubType {
                    ttype: b"TEXT",
                    subtype: b"PLAIN"
                }
            ))
        );
    }
    #[test]
    fn test_parameter_1() {
        assert_eq!(
            parameter(br#""CHARSET" "ISO-8859-1""#),
            Ok((
                b"".as_ref(),
                Parameter {
                    attribute: b"CHARSET",
                    value: b"ISO-8859-1"
                }
            ))
        );
    }
    #[test]
    fn test_parameters_1() {
        assert_eq!(
            parameters(br#""CHARSET" "ISO-8859-1" "second" "2""#),
            Ok((
                b"".as_ref(),
                Parameters {
                    list: vec![
                        Parameter {
                            attribute: b"CHARSET",
                            value: b"ISO-8859-1"
                        },
                        Parameter {
                            attribute: b"second",
                            value: b"2"
                        }
                    ]
                }
            ))
        )
    }
    #[test]
    fn test_get_content_type_text_1() {
        assert_eq!(
            Parameter {
                attribute: b"CHARSET",
                value: b"ISO-8859-1"
            }
            .get_content_type_text(),
            br#"CHARSET="ISO-8859-1""#
        )
    }
    #[test]
    fn test_get_content_type_text_2() {
        assert_eq!(
            ContentTypeTypeAndSubType {
                ttype: b"text",
                subtype: b"plain"
            }
            .get_content_type_text(),
            br#"text/plain"#
        )
    }
}
