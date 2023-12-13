pub use nom::IResult;
use nom::{
    branch::alt,
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
    map(
        delimited(tag(b"("), separated_list1(tag(b" "), parameter), tag(b")")),
        |list| Parameters { list: list },
    )(s)
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

#[derive(Debug, PartialEq)]
pub struct ContentTypeHeaderField<'a> {
    ttype: ContentTypeTypeAndSubType<'a>,
    parameters: Parameters<'a>,
}

impl ContentTypeHeaderField<'_> {
    pub fn get_text(&self) -> Vec<u8> {
        let mut result = b"Content-Type: ".to_vec();
        result.append(&mut self.ttype.get_content_type_text());
        for param in &self.parameters.list {
            result.extend_from_slice(b";\r\n");
            result.extend_from_slice(b"        ");
            result.extend(param.get_content_type_text().iter());
        }
        result.extend_from_slice(b"\r\n");
        result
    }
}

pub fn content_type_header_field_parser(s: &[u8]) -> IResult<&[u8], ContentTypeHeaderField> {
    map(
        tuple((content_type_main, tag(b" "), parameters)),
        // Initialism: cttast for ContentTypeTypeAndSubType
        |(cttast, _, params)| ContentTypeHeaderField {
            ttype: cttast,
            parameters: params,
        },
    )(s)
}
/// RFC 2046
#[derive(Debug, PartialEq)]
pub struct ContentIDHeaderField<'a> {
    value: Option<&'a [u8]>,
}

impl ContentIDHeaderField<'_> {
    pub fn get_text(&self) -> Option<Vec<u8>> {
        if let Some(value) = self.value {
            let mut result = b"Content-ID: ".to_vec();
            result.append(&mut value.to_vec());
            result.extend_from_slice(b"\r\n");
            return Some(result);
        }
        None
    }
}

pub fn content_id_header_field_parser(s: &[u8]) -> IResult<&[u8], ContentIDHeaderField> {
    map(alt((tag_no_case(b"NIL"), double_quoted_string)), |val| {
        if val.to_ascii_lowercase() == b"nil" {
            ContentIDHeaderField { value: None }
        } else {
            ContentIDHeaderField { value: Some(val) }
        }
    })(s)
}

/// RFC 2047
#[derive(Debug, PartialEq)]
pub struct ContentDescriptionHeaderField<'a> {
    value: Option<&'a [u8]>,
}

impl ContentDescriptionHeaderField<'_> {
    pub fn get_text(&self) -> Option<Vec<u8>> {
        if let Some(value) = self.value {
            let mut result = b"Content-Description: ".to_vec();
            result.append(&mut value.to_vec());
            result.extend_from_slice(b"\r\n");
            return Some(result);
        }
        None
    }
}

pub fn content_description_header_field_parser(
    s: &[u8],
) -> IResult<&[u8], ContentDescriptionHeaderField> {
    map(alt((tag_no_case(b"NIL"), double_quoted_string)), |val| {
        if val.to_ascii_lowercase() == b"nil" {
            ContentDescriptionHeaderField { value: None }
        } else {
            ContentDescriptionHeaderField { value: Some(val) }
        }
    })(s)
}
#[derive(Debug, PartialEq)]
pub struct ContentTransferEncodingHeaderField<'a> {
    value: &'a [u8],
}

impl ContentTransferEncodingHeaderField<'_> {
    pub fn get_text(&self) -> Vec<u8> {
        let mut result = b"Content-Transfer-Encoding: ".to_vec();
        result.append(&mut self.value.to_vec());
        result.extend_from_slice(b"\r\n");
        result
    }
}

pub fn content_transfer_encoding_header_field_parser(
    s: &[u8],
) -> IResult<&[u8], ContentTransferEncodingHeaderField> {
    map(double_quoted_string, |val| {
        ContentTransferEncodingHeaderField { value: val }
    })(s)
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
            parameters(br#"("CHARSET" "ISO-8859-1" "second" "2")"#),
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
    #[test]
    fn test_content_type_header_field_parser_1() {
        assert_eq!(
            content_type_header_field_parser(br#""text" "html" ("charset" "utf-8")"#)
                .unwrap()
                .1,
            ContentTypeHeaderField {
                ttype: ContentTypeTypeAndSubType {
                    ttype: b"text",
                    subtype: b"html"
                },
                parameters: Parameters {
                    list: vec![Parameter {
                        attribute: b"charset",
                        value: b"utf-8"
                    }]
                }
            }
        )
    }
    #[test]
    fn test_get_text_1() {
        let text = content_type_header_field_parser(br#""text" "html" ("charset" "utf-8")"#)
            .unwrap()
            .1;
        assert_eq!(
            text.get_text(),
            b"Content-Type: text/html;\r\n        charset=\"utf-8\"\r\n"
        )
    }
    #[test]
    fn test_content_id_header_field_parser_1() {
        assert_eq!(
            content_id_header_field_parser(b"NIL").unwrap().1,
            ContentIDHeaderField { value: None }
        );
        assert_eq!(
            content_id_header_field_parser(br#""<id42@guppylake.bellcore.com>""#)
                .unwrap()
                .1,
            ContentIDHeaderField {
                value: Some(b"<id42@guppylake.bellcore.com>")
            }
        );
    }
    #[test]
    fn test_content_description_header_field_parser_1() {
        assert_eq!(
            content_description_header_field_parser(b"NIL").unwrap().1,
            ContentDescriptionHeaderField { value: None }
        );
        assert_eq!(
            content_description_header_field_parser(br#""Content-Description""#)
                .unwrap()
                .1,
            ContentDescriptionHeaderField {
                value: Some(b"Content-Description")
            }
        );
    }
    #[test]
    fn test_content_id_get_text_1() {
        let ci = content_id_header_field_parser(br#""<id42@guppylake.bellcore.com>""#)
            .unwrap()
            .1;
        assert_eq!(
            ci.get_text(),
            Some(b"Content-ID: <id42@guppylake.bellcore.com>\r\n".to_vec())
        );
    }
    #[test]
    fn test_content_desc_get_text_1() {
        let cd = content_description_header_field_parser(br#""This is a description""#)
            .unwrap()
            .1;
        assert_eq!(
            cd.get_text(),
            Some(b"Content-Description: This is a description\r\n".to_vec())
        );
    }
    #[test]
    fn test_content_transfer_encoding_header_field_parser_1() {
        let res = content_transfer_encoding_header_field_parser(b"\"base64\"")
            .unwrap()
            .1;
        assert_eq!(res, ContentTransferEncodingHeaderField { value: b"base64" });
        assert_eq!(res.get_text(), b"Content-Transfer-Encoding: base64\r\n")
    }
}
