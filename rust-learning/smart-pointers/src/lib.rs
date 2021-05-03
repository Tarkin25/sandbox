use std::cell::RefCell;

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
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
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

pub trait EnhancedRefCell<T> {
    fn read<R: Fn(&T)>(&self, reader: R);

    fn map<M: Fn(&T) -> T>(&self, mapper: M);

    fn write<W: Fn(&mut T)>(&self, writer: W);
}

impl<T> EnhancedRefCell<T> for RefCell<T> {
    fn read<R: Fn(&T)>(&self, reader: R) {
        reader(&*self.borrow());
    }

    fn map<M: Fn(&T) -> T>(&self, mapper: M) {
        let new = mapper(&*self.borrow());

        *self.borrow_mut() = new;
    }

    fn write<W: Fn(&mut T)>(&self, writer: W) {
        writer(&mut *self.borrow_mut());
    }
}

#[cfg(test)]
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
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }

    #[test]
    fn read() {
        let ref_cell = RefCell::new(5);

        ref_cell.read(|value| assert_eq!(5, *value));
    }

    #[test]
    fn map() {
        let ref_cell = RefCell::new(5);

        ref_cell.map(|v| *v * 2);

        assert_eq!(10, *ref_cell.borrow());
    }

    #[test]
    fn write() {
        let ref_cell = RefCell::new(5);

        let writer = |v: &mut i32| *v = 10;

        ref_cell.write(writer);

        assert_eq!(10, *ref_cell.borrow());
    }
}