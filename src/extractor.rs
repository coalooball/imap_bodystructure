use crate::sequence::{self, Sequence};
use nom::{
    bytes::complete::{tag, tag_no_case, take_until, take_while},
    character::{
        complete::{alphanumeric1, digit1},
        is_digit,
    },
    combinator::{map, opt},
    multi::many1,
    sequence::{delimited, terminated, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct UidFetch {
    pub uid: Vec<u8>,
    pub sequence: sequence::Sequence,
}

pub fn fetch_all_body_parser(s: &[u8]) -> IResult<&[u8], ()> {
    map(
        tuple((
            alphanumeric1,
            opt(tag_no_case(b" UID")),
            tag_no_case(b" FETCH "),
            digit1,
            tag_no_case(" BODY"),
            opt(tag_no_case(b".PEEK")),
            tag(b"[]"),
        )),
        |_| (),
    )(s)
}

pub fn uid_fetch_body_parser(s: &[u8]) -> IResult<&[u8], UidFetch> {
    map(
        tuple((
            alphanumeric1,
            tag(b" "),
            tag_no_case(b"UID FETCH "),
            digit1,
            tag_no_case(" BODY"),
            opt(tag_no_case(b".PEEK")),
            delimited(
                tag(b"["),
                take_while(|x| is_digit(x) || x == b'.' as u8),
                tag(b"]"),
            ),
        )),
        |(_, _, _, uid, _, _, seq)| UidFetch {
            sequence: Sequence::new(seq).unwrap(),
            uid: uid.to_vec(),
        },
    )(s)
}

pub fn find_uid_in_response(origin_vec: &Vec<u8>) -> Vec<u8> {
    let mut token: Vec<u8> = Vec::new();
    let mut uid: Vec<u8> = Vec::new();
    let mut recording = false;

    for &i in origin_vec {
        if recording {
            if i == b' ' {
                break;
            }
            uid.push(i);
        } else {
            if i.is_ascii_alphabetic() {
                token.push(i);
            } else {
                if ascii_lowercase_equal(&token, b"UID") {
                    recording = true;
                }
                token.clear();
            }
        }
    }

    uid
}

pub fn split_multi_fetch_response_parser(
    s: &[u8],
    include_first_line: bool,
) -> IResult<&[u8], Vec<Vec<u8>>> {
    match include_first_line {
        true => map(
            many1(map(extract_fetch_respone_all_context_parser, |x| {
                x.to_vec()
            })),
            |x| x,
        )(s),
        false => map(
            many1(map(extract_fetch_respone_main_context_parser, |x| {
                x.to_vec()
            })),
            |x| x,
        )(s),
    }
}

pub fn ascii_lowercase_equal(vec1: &[u8], vec2: &[u8]) -> bool {
    vec1.iter()
        .map(|&b| b.to_ascii_lowercase())
        .eq(vec2.iter().map(|&b| b.to_ascii_lowercase()))
}

pub fn extract_bodystructure(origin_vec: &Vec<u8>) -> Vec<u8> {
    let mut token: Vec<u8> = Vec::new();
    let mut bodystructure: Vec<u8> = Vec::new();
    let mut recording = false;
    let mut brackets_count = 0;

    for &i in origin_vec {
        if recording {
            bodystructure.push(i);
            if i == b'(' {
                brackets_count += 1;
            } else if i == b')' {
                brackets_count -= 1;
                if brackets_count == 0 {
                    break;
                }
            }
        } else {
            if i.is_ascii_alphabetic() {
                token.push(i);
            } else {
                if ascii_lowercase_equal(&token, b"BODYSTRUCTURE") {
                    recording = true;
                    bodystructure.extend_from_slice(b"BODYSTRUCTURE");
                    bodystructure.push(i);
                    if i == b'(' {
                        brackets_count += 1;
                    }
                }
                token.clear();
            }
        }
    }

    bodystructure
}

pub fn extract_fetch_respone_all_context_parser(s: &[u8]) -> IResult<&[u8], &[u8]> {
    delimited(tag(b"*"), take_until("\r\n)\r\n"), tag(b"\r\n)\r\n"))(s)
}

pub fn extract_fetch_respone_all_context(s: &[u8]) -> Option<&[u8]> {
    match extract_fetch_respone_all_context_parser(s) {
        Ok((_, x)) => Some(x),
        Err(_) => None,
    }
}

pub fn extract_fetch_respone_main_context_parser(s: &[u8]) -> IResult<&[u8], &[u8]> {
    delimited(
        tag(b"*"),
        extract_fetch_respone_main_context_parser_0,
        tag(b"\r\n)\r\n"),
    )(s)
}

fn extract_fetch_respone_main_context_parser_0(s: &[u8]) -> IResult<&[u8], &[u8]> {
    map(
        tuple((
            terminated(take_until("\r\n"), tag(b"\r\n")),
            take_until("\r\n)\r\n"),
        )),
        |(_, x)| x,
    )(s)
}

pub fn extract_fetch_respone_main_context(s: &[u8]) -> Option<&[u8]> {
    match extract_fetch_respone_main_context_parser(s) {
        Ok((_, x)) => Some(x),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uid_fetch_body_parser() {
        let result1 = uid_fetch_body_parser(b"22 UID FETCH 696 BODY.PEEK[1.1]")
            .unwrap()
            .1;
        assert_eq!(
            result1,
            UidFetch {
                sequence: Sequence::new(b"1.1").unwrap(),
                uid: b"696".to_vec()
            }
        );
        let result2 = uid_fetch_body_parser(b"a5 uid fetch 303416 body.peek[1.1]")
            .unwrap()
            .1;
        assert_eq!(
            result2,
            UidFetch {
                sequence: Sequence::new(b"1.1").unwrap(),
                uid: b"303416".to_vec()
            }
        );
        let result3 = uid_fetch_body_parser(b"a5 uid fetch 303416 body[1.1]")
            .unwrap()
            .1;
        assert_eq!(
            result3,
            UidFetch {
                sequence: Sequence::new(b"1.1").unwrap(),
                uid: b"303416".to_vec()
            }
        );
    }
    #[test]
    fn test_find_uid_in_response() {
        let text = b"* 170 FETCH (UID 665 FLAGS () RFC822.SIZE 13473 INTERNALDATE
         \"05-Dec-2023 06:17:00 +0000\" BODYSTRUCTURE ((\"text\" \"html\" (\"charset\" \"utf-8\") NIL NIL \"base64\" 4914 63 NIL NIL NIL NIL)
         (\"application\" \"octet-stream\" NIL NIL NIL \"base64\" 7160 NIL (\"attachment\" (\"filename*\" \"utf-8\'\'05.APC_bind_shell-%E5%AF%86%E7%A0%81.rar\")) NIL NIL) 
         \"mixed\" (\"boundary\" \"===============0638663467655325798==\") NIL NIL NIL) 
         BODY[HEADER.FIELDS (DATE SUBJECT FROM SENDER REPLY-TO TO CC BCC MESSAGE-ID REFERENCES IN-REPLY-TO X-MAILMASTER-SHOWONERCPT 
            X-CUSTOM-MAIL-MASTER-SENT-ID DISPOSITION-NOTIFICATION-TO X-CM-CTRLMSGS)] {149}\r\n
            Subject: =?utf-8?b?5L2g5aW9IDA15rWL6K+V?=\r\nFrom: liutianyu@nextcloud.games\r\n
            To: shenzongxu@nextcloud.games\r\nDate: Tue, 05 Dec 2023 06:17:00 -0000\r\n\r\n)".to_vec();
        let uid1 = find_uid_in_response(&text);
        assert_eq!(uid1, b"665");
    }
    #[test]
    fn test_split_multi_fetch_response() {
        let text = b"* fetch0 something\r\n t1 \r\n)\r\n* fetch1 \r\n t2 \r\n)\r\n* fetch2\r\n t3 \r\n)\r\n Ok ...";
        let split_text1 = split_multi_fetch_response_parser(text, false).unwrap().1;
        assert_eq!(
            split_text1,
            vec![b" t1 ".to_vec(), b" t2 ".to_vec(), b" t3 ".to_vec()]
        )
    }

    #[test]
    fn extract_bodystructure_test_1() {
        let target = br#"BODYSTRUCTURE ((("TEXT" "HTML" ("charset" "gbk") NIL NIL "BASE64" 140 2 NIL NIL NIL) "RELATED" ("BOUNDARY" "----=_Part_28035_897908132.1699414214660") NIL NIL) "MIXED" ("BOUNDARY" "----=_Part_28034_578039922.1699414214660") NIL NIL)"#;
        let text = br#"* 10 FETCH (BODYSTRUCTURE ((("TEXT" "HTML" ("charset" "gbk") NIL NIL "BASE64" 140 2 NIL NIL NIL) "RELATED" ("BOUNDARY" "----=_Part_28035_897908132.1699414214660") NIL NIL) "MIXED" ("BOUNDARY" "----=_Part_28034_578039922.1699414214660") NIL NIL))"#;
        let bodystructure = extract_bodystructure(&text.to_vec());
        assert_eq!(bodystructure, target);
    }
    #[test]
    fn extract_bodystructure_test_2() {
        let target = br#"BODYSTRUCTURE (("text" "html" ("charset" "utf-8") NIL NIL "base64" 1188 16 NIL NIL NIL NIL) "mixed" ("boundary" "===============1522363357941492443==") NIL NIL NIL)"#;
        let text = br#"* 154 FETCH (UID 649 FLAGS () RFC822.SIZE 2394 INTERNALDATE "05-Dec-2023 06:16:58 +0000" BODYSTRUCTURE (("text" "html" ("charset" "utf-8") NIL NIL "base64" 1188 16 NIL NIL NIL NIL) "mixed" ("boundary" "===============1522363357941492443==") NIL NIL NIL) BODY[HEADER.FIELDS (DATE SUBJECT FROM SENDER REPLY-TO TO CC BCC MESSAGE-ID REFERENCES IN-REPLY-TO X-MAILMASTER-SHOWONERCPT X-CUSTOM-MAIL-MASTER-SENT-ID DISPOSITION-NOTIFICATION-TO X-CM-CTRLMSGS)] {181}
        Subject: =?utf-8?b?5L2g5aW9IDBiMGZiYjZkYmFmM2FmYmIgenFhLWVtYWls5rWL6K+V?=
        From: liutianyu@nextcloud.games
        To: shenzongxu@nextcloud.games
        Date: Tue, 05 Dec 2023 06:16:58 -0000"#;
        let bodystructure = extract_bodystructure(&text.to_vec());
        assert_eq!(bodystructure, target);
    }
    #[test]
    fn ascii_lowercase_equal_test_1() {
        let str1 = b"Hello";
        let str2 = b"hello";
        let str3 = b"HELLO";
        assert_ne!(str1, str2);
        assert_eq!(str1.to_ascii_lowercase(), str2.to_ascii_lowercase());
        assert!(ascii_lowercase_equal(str2, str3));
    }
    #[test]
    fn test_extract_fetch_respone_main_context_parser() {
        let text = b"* 174 FETCH (UID 669 BODY[1] {78} \r\nNzU5YjI1NmExYjRjNTkwYyA8YnI+5L2g5aW977yaPGJyPiAgICDor7fmn6XmlLbpmYTku7bjgII=\r\n)\r\n66 OK Fetch completed (0.004 + 0.000 + 0.003 secs).";
        assert_eq!(
            extract_fetch_respone_main_context_parser(text).unwrap().1,
            b"NzU5YjI1NmExYjRjNTkwYyA8YnI+5L2g5aW977yaPGJyPiAgICDor7fmn6XmlLbpmYTku7bjgII="
        )
    }
    #[test]
    fn test_extract_fetch_respone_main_context_parser_0() {
        let text = b"174 FETCH (UID 669 BODY[1] {78} \r\nNzU5YjI1NmExYjRjNTkwYyA8\r\n)\r\n";
        assert_eq!(
            extract_fetch_respone_main_context_parser_0(text).unwrap().1,
            b"NzU5YjI1NmExYjRjNTkwYyA8"
        )
    }
    #[test]
    fn test_extract_fetch_respone_all_context_parser() {
        let text = b"* 174 FETCH (UID 669 BODY[1] {78} \r\nNzU5YjI1NmExYjRjNTkwYyA8\r\n)\r\n";
        assert_eq!(
            extract_fetch_respone_all_context_parser(text).unwrap().1,
            b" 174 FETCH (UID 669 BODY[1] {78} \r\nNzU5YjI1NmExYjRjNTkwYyA8"
        )
    }
    #[test]
    fn test_fetch_all_body() {
        assert_eq!(
            fetch_all_body_parser(b"123 FETCH 3456 body[]").unwrap().1,
            ()
        );
        assert_eq!(
            fetch_all_body_parser(b"123 UID FETCH 3456 body[]")
                .unwrap()
                .1,
            ()
        );
        assert_eq!(
            fetch_all_body_parser(b"123 FETCH 3456 body.peek[]")
                .unwrap()
                .1,
            ()
        );
        assert_eq!(
            fetch_all_body_parser(b"123 UID FETCH 3456 body.peek[]")
                .unwrap()
                .1,
            ()
        );
    }
}
