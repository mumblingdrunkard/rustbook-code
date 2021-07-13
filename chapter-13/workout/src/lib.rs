use std::collections::HashMap;

pub struct Cacher<T, U, V> {
    calculation: T,
    values: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V>
where
    T: Fn(&U) -> V,
    U: std::hash::Hash + Eq + Clone,
{
    pub fn from(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    pub fn value(&mut self, key: &U) -> &V {
        if self.values.contains_key(key) {
            return self.values.get(key).unwrap();
        }

        let value = (self.calculation)(key);
        self.values.entry(key.clone()).or_insert(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::from(|a: &i32| *a);
        let v1 = c.value(&1);
        assert_eq!(*v1, 1);

        let v2 = c.value(&2);
        assert_eq!(*v2, 2);
    }

    #[test]
    fn precalculated_str_len() {
        let mut c = Cacher::from(|s: &String| s.len());

        let l1 = c.value(&"Hello, World!".to_string());
        assert_eq!(*l1, 13);
    }
}
