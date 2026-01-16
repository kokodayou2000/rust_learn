use std::{
    cell::Cell,
    collections::{HashMap, hash_map},
};

struct Calculator<'ctx> {
    f: Box<dyn Fn(i64) -> i64 + 'ctx>,
    cache: Cell<HashMap<i64, i64>>,
}

impl<'ctx> Calculator<'ctx> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(i64) -> i64 + 'ctx,
    {
        Self {
            f: Box::new(f),
            cache: Cell::new(HashMap::new()),
        }
    }

    pub fn eval(&self, input: i64) -> i64 {
        let mut cache = self.cache.take();
        let value = match cache.entry(input) {
            hash_map::Entry::Occupied(entry) => *entry.get(),
            hash_map::Entry::Vacant(vacant_entry) => {
                let value = (self.f)(input);
                vacant_entry.insert(value);
                value
            }
        };

        self.cache.set(cache);

        value
    }
}

#[cfg(test)]
mod tests {
    use crate::modules::examples::fib_calc::{self, Calculator};

    fn fib(n: i64) -> i64 {
        if (n <= 2) {
            return n;
        }
        fib(n - 1) + fib(n - 2)
    }

    #[test]
    fn it_works() {
        let fib_calc = Calculator::new(|n| {
            println!("cal fib({})", n);
            fib(n)
        });

        assert_eq!(fib_calc.eval(2), 2);
        assert_eq!(fib_calc.eval(3), 3);
        assert_eq!(fib_calc.eval(3), 3);
        assert_eq!(fib_calc.eval(8), 34);
        assert_eq!(fib_calc.eval(8), 34);
    }
}
