use std::{collections::VecDeque, str::from_utf8};

use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map, multi::separated_list0,
    IResult,
};

pub fn sequence_parser(s: &[u8]) -> IResult<&[u8], Vec<usize>> {
    map(
        separated_list0(
            tag(b"."),
            map(digit1, |x| {
                let d = from_utf8(x).unwrap();
                let num = str::parse::<usize>(d).unwrap();
                num
            }),
        ),
        |x| x,
    )(s)
}

#[derive(Debug, PartialEq)]
pub struct Sequence(pub VecDeque<usize>);

impl Sequence {
    pub fn new(s: &[u8]) -> Result<Sequence, String> {
        if let Ok((_, sequence)) = sequence_parser(s) {
            let mut tmp: VecDeque<usize> = VecDeque::new();
            tmp.extend(sequence);
            Ok(Sequence(tmp))
        } else {
            Err(String::from("Parsing sequence unscessfully."))
        }
    }

    pub fn pop(&mut self) -> Option<usize> {
        self.0.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut tmp_vec_deque: VecDeque<usize> = VecDeque::new();
        tmp_vec_deque.extend(vec![1, 1]);
        let target = Sequence(tmp_vec_deque);
        let seq = Sequence::new(b"1.1").unwrap();
        assert_eq!(seq, target);
    }
}
