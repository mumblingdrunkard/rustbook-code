#[allow(unused)]
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

#[allow(unused)]
fn shoes_in_my_size<'a>(shoes: &'a [Shoe], shoe_size: u32) -> Vec<&Shoe> {
    shoes.iter().filter(|s| s.size == shoe_size).collect()
}

struct Counter {
    count: u32,
}

#[allow(unused)]
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();

        let v2 = vec![1, 2, 3];
        let mut v2_iter = v2.iter();

        assert_eq!(v1_iter.next(), v2_iter.next());
        assert_eq!(v1_iter.next(), v2_iter.next());
        assert_eq!(v1_iter.next(), v2_iter.next());
    }

    #[test]
    fn iterator_sum() {
        let v = vec![1, 2, 3];
        let total: i64 = v.iter().sum();
        assert_eq!(total, 6);
    }

    #[test]
    fn iterator_adaptor() {
        let v1 = vec![1, 2, 3];
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

        assert_eq!(v1, vec![1, 2, 3]);
        assert_eq!(v2, vec![2, 3, 4]);
    }

    #[test]
    fn iterator_consume() {
        let v1 = vec![1, 2, 3];
        // Consumes v1, taking ownership
        let v2: Vec<i64> = v1.into_iter().map(|x| x + 1).collect();

        // Uncommenting the line below makes the code not compile
        // assert_eq!(v1, vec![1, 2, 3]);

        assert_eq!(v2, vec![2, 3, 4]);
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: "sneaker".to_string(),
            },
            Shoe {
                size: 13,
                style: "sandal".to_string(),
            },
            Shoe {
                size: 10,
                style: "boot".to_string(),
            },
        ];

        let in_my_size = shoes_in_my_size(&shoes, 10);
        let expected = vec![&shoes[0], &shoes[2]];

        assert_eq!(in_my_size, expected);
    }

    #[test]
    fn testing_next() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn other_iterator_methods() {
        // Creates the tuples:
        // 1 2 3 4
        // 2 3 4 5
        // Then maps them to:
        // 2 6 12 20
        // Which is filtered to
        // 6 12
        // and then summed to
        // 18
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();

        assert_eq!(sum, 18);
    }
}
