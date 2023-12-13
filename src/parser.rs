use std::str::from_utf8;

pub use nom::IResult;
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_till},
    character::complete::digit1,
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
pub struct Parameter {
    attribute: Vec<u8>,
    value: Vec<u8>,
}

impl Parameter {
    pub fn get_content_type_text(&self) -> Vec<u8> {
        let mut result = self.attribute.clone();
        result.extend_from_slice(b"=\"");
        result.extend(self.value.clone().iter());
        result.extend_from_slice(b"\"");
        result
    }
}

#[derive(Debug, PartialEq)]
pub struct Parameters {
    list: Vec<Parameter>,
}

pub fn parameter(s: &[u8]) -> IResult<&[u8], Parameter> {
    map(
        tuple((double_quoted_string, tag(b" "), double_quoted_string)),
        |(attribute, _, value)| Parameter {
            attribute: attribute.to_vec(),
            value: value.to_vec(),
        },
    )(s)
}

pub fn parameters(s: &[u8]) -> IResult<&[u8], Parameters> {
    map(
        alt((
            map(tag_no_case("NIL"), |_| vec![]),
            delimited(tag(b"("), separated_list1(tag(b" "), parameter), tag(b")")),
        )),
        |list| Parameters { list: list },
    )(s)
}

#[derive(Debug, PartialEq)]
pub struct ContentTypeTypeAndSubType {
    ttype: Vec<u8>,
    subtype: Vec<u8>,
}

impl ContentTypeTypeAndSubType {
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
            ttype: ttype.to_vec(),
            subtype: subtype.to_vec(),
        },
    )(s)
}

#[derive(Debug, PartialEq)]
pub struct ContentTypeHeaderField {
    ttype: ContentTypeTypeAndSubType,
    parameters: Parameters,
}

impl ContentTypeHeaderField {
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
pub struct ContentIDHeaderField {
    value: Option<Vec<u8>>,
}

impl ContentIDHeaderField {
    pub fn get_text(&self) -> Option<Vec<u8>> {
        if let Some(value) = self.value.clone() {
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
            ContentIDHeaderField {
                value: Some(val.to_vec()),
            }
        }
    })(s)
}

/// RFC 2047
#[derive(Debug, PartialEq)]
pub struct ContentDescriptionHeaderField {
    value: Option<Vec<u8>>,
}

impl ContentDescriptionHeaderField {
    pub fn get_text(&self) -> Option<Vec<u8>> {
        if let Some(value) = self.value.clone() {
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
            ContentDescriptionHeaderField {
                value: Some(val.to_vec()),
            }
        }
    })(s)
}
#[derive(Debug, PartialEq)]
pub struct ContentTransferEncodingHeaderField {
    value: Vec<u8>,
}

impl ContentTransferEncodingHeaderField {
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
        ContentTransferEncodingHeaderField {
            value: val.to_vec(),
        }
    })(s)
}

#[derive(Debug, PartialEq)]
pub struct ContentSize(Option<usize>, Option<usize>);

impl ContentSize {
    pub fn get_text(&self) -> Vec<u8> {
        if let Some(value) = self.0 {
            let tmp_string = value.to_string();
            let mut result = tmp_string.as_str().as_bytes().to_vec();
            if let Some(value2) = self.1 {
                let value2 = value2.to_string();
                result.append(&mut vec![0x20]);
                result.append(&mut value2.as_str().as_bytes().to_vec());
            }
            result
        } else {
            vec![0x30]
        }
    }
}

pub fn content_size_parser(s: &[u8]) -> IResult<&[u8], ContentSize> {
    map(
        alt((
            map(tag_no_case("NIL"), |_| None),
            map(
                alt((
                    map(tuple((digit1, tag(b" "), digit1)), |(x, _, y)| (x, Some(y))),
                    map(digit1, |x| (x, None)),
                )),
                |x| Some(x),
            ),
        )),
        |val| {
            if let Some(size_tuple) = val {
                let (left, right) = size_tuple;
                let tmp_str = from_utf8(left).unwrap();
                let left_size = str::parse::<usize>(tmp_str).unwrap();
                let result = match right {
                    Some(right_val) => {
                        let tmp_str = from_utf8(right_val).unwrap();
                        let right_size = str::parse::<usize>(tmp_str).unwrap();
                        Some(right_size)
                    }
                    None => None,
                };
                ContentSize(Some(left_size), result)
            } else {
                ContentSize(None, None)
            }
        },
    )(s)
}

#[derive(Debug, PartialEq)]
pub struct ContentMD5HeaderField {
    value: Vec<u8>,
}

impl ContentMD5HeaderField {
    pub fn get_text(&self) -> Vec<u8> {
        let mut result = b"Content-MD5: ".to_vec();
        result.append(&mut self.value.to_vec());
        result.extend_from_slice(b"\r\n");
        result
    }
}

pub fn content_md5_header_field_parser(s: &[u8]) -> IResult<&[u8], ContentMD5HeaderField> {
    map(double_quoted_string, |val| ContentMD5HeaderField {
        value: val.to_vec(),
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
                    ttype: b"TEXT".to_vec(),
                    subtype: b"PLAIN".to_vec()
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
                    attribute: b"CHARSET".to_vec(),
                    value: b"ISO-8859-1".to_vec()
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
                            attribute: b"CHARSET".to_vec(),
                            value: b"ISO-8859-1".to_vec()
                        },
                        Parameter {
                            attribute: b"second".to_vec(),
                            value: b"2".to_vec()
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
                attribute: b"CHARSET".to_vec(),
                value: b"ISO-8859-1".to_vec()
            }
            .get_content_type_text(),
            br#"CHARSET="ISO-8859-1""#
        )
    }
    #[test]
    fn test_get_content_type_text_2() {
        assert_eq!(
            ContentTypeTypeAndSubType {
                ttype: b"text".to_vec(),
                subtype: b"plain".to_vec()
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
                    ttype: b"text".to_vec(),
                    subtype: b"html".to_vec()
                },
                parameters: Parameters {
                    list: vec![Parameter {
                        attribute: b"charset".to_vec(),
                        value: b"utf-8".to_vec()
                    }]
                }
            }
        );
        assert_eq!(
            content_type_header_field_parser(br#""application" "octet-stream" NIL"#)
                .unwrap()
                .1,
            ContentTypeHeaderField {
                ttype: ContentTypeTypeAndSubType {
                    ttype: b"application".to_vec(),
                    subtype: b"octet-stream".to_vec()
                },
                parameters: Parameters { list: vec![] }
            }
        );
    }
    #[test]
    fn test_content_type_header_field_get_text_1() {
        let text = content_type_header_field_parser(br#""text" "html" ("charset" "utf-8")"#)
            .unwrap()
            .1;
        assert_eq!(
            text.get_text(),
            b"Content-Type: text/html;\r\n        charset=\"utf-8\"\r\n"
        );
        let text = content_type_header_field_parser(br#""application" "octet-stream" NIL"#)
            .unwrap()
            .1;
        assert_eq!(
            text.get_text(),
            b"Content-Type: application/octet-stream\r\n"
        );
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
                value: Some(b"<id42@guppylake.bellcore.com>".to_vec())
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
                value: Some(b"Content-Description".to_vec())
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
        assert_eq!(
            res,
            ContentTransferEncodingHeaderField {
                value: b"base64".to_vec()
            }
        );
        assert_eq!(res.get_text(), b"Content-Transfer-Encoding: base64\r\n")
    }
    #[test]
    fn test_content_size_1() {
        assert_eq!(
            content_size_parser(b"1234").unwrap().1,
            ContentSize(Some(1234), None)
        );
        assert_eq!(
            content_size_parser(b"nil").unwrap().1,
            ContentSize(None, None)
        );
        assert_eq!(
            content_size_parser(b"1417 36").unwrap().1,
            ContentSize(Some(1417), Some(36))
        );
    }
    #[test]
    fn test_content_size_2() {
        assert_eq!(content_size_parser(b"1234").unwrap().1.get_text(), b"1234");
        assert_eq!(content_size_parser(b"nil").unwrap().1.get_text(), b"0");
        assert_eq!(
            content_size_parser(b"1417 36").unwrap().1.get_text(),
            b"1417 36"
        );
    }
    #[test]
    fn test_md5_header_field() {
        assert_eq!(
            content_md5_header_field_parser(b"\"Q2hlY2sgSW50ZWdyaXR5IQ==\"")
                .unwrap()
                .1
                .get_text(),
            b"Content-MD5: Q2hlY2sgSW50ZWdyaXR5IQ==\r\n".as_ref().to_vec()
        );
        assert_eq!(
            content_md5_header_field_parser(b"\"Q2hlY2sgSW50ZWdyaXR5IQ==\"")
                .unwrap()
                .1,
            ContentMD5HeaderField {
                value: b"Q2hlY2sgSW50ZWdyaXR5IQ==".to_vec()
            }
        );
    }
}
