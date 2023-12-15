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
            tag(" BODY"),
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

#[cfg(test)]
mod tests {
    use crate::{fetch::UidFetch, sequence::Sequence};

    use super::uid_fetch_body_parser;

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
        )
    }
}
