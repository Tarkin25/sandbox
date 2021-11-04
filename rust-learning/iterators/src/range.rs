use std::ops::{Add, AddAssign};

struct Range<T> {
    /// The lowest value, inclusive
    low: T,
    /// The highest value, exclusive
    high: T,
}

impl<T> Range<T> {
    pub fn new(low: T, high: T) -> Self {
        Self {
            low,
            high,
        }
    }
}

impl<T> Iterator for Range<T>
where T: Copy + PartialOrd + AddAssign + From<i32>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.low < self.high {
            let value = self.low;

            self.low += 1.into();

            Some(value)
        } else {
            None
        }
    }
}

impl<T> DoubleEndedIterator for Range<T>
where T: Copy + PartialOrd + AddAssign + From<i32> + Add<Output=T>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.high > self.low {
            self.high += (-1).into();

            Some(self.high)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator() {
        let mut range = Range::new(0.0, 2.0);

        assert_eq!(range.next(), Some(0.0));
        assert_eq!(range.next(), Some(1.0));
        assert_eq!(range.next(), None);
    }

    #[test]
    fn double_ended_iterator() {
        let mut range = Range::new(0, 2);

        assert_eq!(range.next_back(), Some(1));
        assert_eq!(range.next_back(), Some(0));
        assert_eq!(range.next_back(), None);
    }

    #[test]
    fn print() {
        for i in Range::new(0, 2).rev() {
            println!("{}", i);
        }
    }
}