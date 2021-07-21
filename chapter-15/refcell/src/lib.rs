pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("ERROR: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("URGENT WARNING: You have used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("WARNING: You have used up over 75% of your quota!");
        }
    }
}

#[cfg(any(test, doctest))]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(message.to_string());
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock = MockMessenger::new();
        let mut tracker = LimitTracker::new(&mock, 100);

        tracker.set_value(80);

        assert_eq!(mock.sent_messages.borrow().len(), 1);
    }

    #[test]
    #[should_panic]
    fn multiple_borrow_panic() {
        let mock = MockMessenger::new();

        let mut one_borrow = mock.sent_messages.borrow_mut();
        let mut two_borrow = mock.sent_messages.borrow_mut();

        one_borrow.push("hello".to_string());
        two_borrow.push(" world".to_string());
    }
}
