pub struct C15 {
    state: i32,
}

impl Iterator for C15 {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.state < 5 {
            let item = self.state;
            self.state += 1;
            Some(item)
        } else {
            None
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut c = C15 { state: 3 };
        assert_eq!(Some(3), c.next());
        assert_eq!(Some(4), c.next());
        assert_eq!(None, c.next());
        assert_eq!(None, c.next());
    }
}
