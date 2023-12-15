use crate::preparser;
use crate::sequence::{self, Sequence};
use nom::{
    bytes::complete::{tag, tag_no_case, take_while},
    character::{
        complete::{alphanumeric1, digit1},
        is_digit,
    },
    combinator::{map, opt},
    sequence::{delimited, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct UidFetch {
    pub uid: Vec<u8>,
    pub sequence: sequence::Sequence,
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
                if preparser::ascii_lowercase_equal(&token, b"UID") {
                    recording = true;
                }
                token.clear();
            }
        }
    }

    uid
}
#[cfg(test)]
mod tests {
    use crate::{response_parser::UidFetch, sequence::Sequence};

    use super::{uid_fetch_body_parser, find_uid_in_response};

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
        let text = b"* 170 FETCH (UID 665 FLAGS () RFC822.SIZE 13473 INTERNALDATE \"05-Dec-2023 06:17:00 +0000\" BODYSTRUCTURE ((\"text\" \"html\" (\"charset\" \"utf-8\") NIL NIL \"base64\" 4914 63 NIL NIL NIL NIL)(\"application\" \"octet-stream\" NIL NIL NIL \"base64\" 7160 NIL (\"attachment\" (\"filename*\" \"utf-8\'\'05.APC_bind_shell-%E5%AF%86%E7%A0%81.rar\")) NIL NIL) \"mixed\" (\"boundary\" \"===============0638663467655325798==\") NIL NIL NIL) BODY[HEADER.FIELDS (DATE SUBJECT FROM SENDER REPLY-TO TO CC BCC MESSAGE-ID REFERENCES IN-REPLY-TO X-MAILMASTER-SHOWONERCPT X-CUSTOM-MAIL-MASTER-SENT-ID DISPOSITION-NOTIFICATION-TO X-CM-CTRLMSGS)] {149}\r\nSubject: =?utf-8?b?5L2g5aW9IDA15rWL6K+V?=\r\nFrom: liutianyu@nextcloud.games\r\nTo: shenzongxu@nextcloud.games\r\nDate: Tue, 05 Dec 2023 06:17:00 -0000\r\n\r\n)".to_vec();
        let uid1 = find_uid_in_response(&text);
        assert_eq!(uid1, b"665");
    }
}
