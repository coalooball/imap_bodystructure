use std::vec;

#[derive(Debug)]
pub struct SequenceNumbers(Vec<u8>, usize, usize);

impl SequenceNumbers {
    pub fn new() -> SequenceNumbers {
        SequenceNumbers(vec![], 0, 0)
    }

    fn get_current_level(&self) -> usize {
        self.1
    }

    fn get_capacity_level(&self) -> usize {
        self.2
    }

    fn add_current_level(&mut self) {
        self.1 += 1;
    }

    fn add_capacity_level(&mut self) {
        self.2 += 1;
    }

    fn minus_current_level(&mut self) {
        self.1 -= 1;
    }

    fn minus_capacity_level(&mut self) {
        self.2 -= 1;
    }

    fn is_current_level_less_then_capacity_level(&self) -> bool {
        self.get_current_level() < self.get_capacity_level()
    }

    fn need_to_add_new_level(&mut self) -> bool {
        if self.is_current_level_less_then_capacity_level() {
            false
        } else {
            true
        }
    }

    fn need_to_remove_latest_level(&mut self) -> bool {
        if self.is_current_level_less_then_capacity_level() {
            true
        } else {
            false
        }
    }

    fn push_new_level(&mut self) {
        self.add_current_level();
        self.add_capacity_level();
        self.0.push(1);
    }

    fn upgrade_the_capacity_level(&mut self) {
        self.add_current_level();
        let capacity_level_index = self.get_capacity_level() - 1;
        self.0[capacity_level_index] += 1;
    }

    fn get_string(&self) -> String {
        let end = self.get_current_level();
        self.0[0..end]
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(".")
    }

    pub fn get_b_string(&self) -> Vec<u8> {
        self.get_string().into_bytes()
    }

    fn return_previous_level(&mut self) {
        self.minus_current_level();
    }

    fn return_previous_level_with_removing_capacity_item(&mut self) {
        self.minus_current_level();
        self.minus_capacity_level();
        self.0.pop();
    }

    pub fn next_level(&mut self) {
        if self.need_to_add_new_level() {
            self.push_new_level();
        } else {
            self.upgrade_the_capacity_level();
        }
    }

    pub fn previous_level(&mut self) {
        if self.need_to_remove_latest_level() {
            self.return_previous_level_with_removing_capacity_item();
        } else {
            self.return_previous_level();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let test_str = b"test((ewffe(";
        let mut sn = SequenceNumbers::new();
        for &i in test_str {
            if i == b'(' {
                sn.next_level();
            } else if i == b')' {
                sn.previous_level();
            } else {
            }
        }
        assert_eq!(sn.get_b_string(), b"1.1.1");
    }

    #[test]
    fn test_2() {
        let test_str = b"test((ewffe()(";
        let mut sn = SequenceNumbers::new();
        for &i in test_str {
            if i == b'(' {
                sn.next_level();
            } else if i == b')' {
                sn.previous_level();
            } else {
            }
        }
        assert_eq!(sn.get_b_string(), b"1.1.2");
    }

    #[test]
    fn test_3() {
        let test_str = b"(((test)))";
        let mut sn = SequenceNumbers::new();
        for &i in test_str {
            if i == b'(' {
                sn.next_level();
            } else if i == b')' {
                sn.previous_level();
            } else {
            }
        }
        assert_eq!(sn.get_b_string(), b"");
    }
}
