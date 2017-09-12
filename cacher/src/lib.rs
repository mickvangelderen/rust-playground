use std::collections::HashMap;
use std::hash::Hash;

pub struct Cacher<T, A>
where T: Fn(&A) -> i32 {
    val: HashMap<A, i32>,
    fun: T,
}

impl<T, A> Cacher<T, A>
where T: Fn(&A) -> i32, A: Eq + PartialEq + Hash + Clone {
    pub fn new(fun: T) -> Cacher<T, A> {
        Cacher {
            val: HashMap::new(),
            fun,
        }
    }

    pub fn value(&mut self, arg: &A) -> i32 {
        match self.val.get(&arg) {
            Some(&val) => val,
            None => {
                let val = (self.fun)(arg);
                self.val.insert(arg.clone(), val);
                val
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Figure out how to check that the value is not recomputed.
    #[test]
    fn uncached() {
        let mut cached = Cacher::new(|x| x + 1);
        assert_eq!(2, cached.value(&1));
    }

    #[test]
    fn cached() {
        let mut cached = Cacher::new(|x| x + 1);
        assert_eq!(2, cached.value(&1));
        assert_eq!(2, cached.value(&1));
    }

    #[test]
    fn cached_different_arg() {
        let mut cached = Cacher::new(|x| x + 1);
        assert_eq!(2, cached.value(&1));
        assert_eq!(3, cached.value(&2));
    }
}
