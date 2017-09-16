pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

pub struct ListIterator<T> {
    head: Box<List<T>>,
}

impl<T> Iterator for ListIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match *self.head {
            List::Cons(item, tail) => {
                self.head = tail;
                Some(item)
            },
            List::Nil => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let end = List::<i32>::new();
        let e2 = List::Nil;

        assert_eq!(Nil, end);
    }
}
