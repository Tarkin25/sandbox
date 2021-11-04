use std::alloc::{alloc, dealloc, Layout};
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};

fn put_on_heap<T>(value: T) -> *mut T {
    unsafe {
        let ptr = alloc(Layout::new::<T>()) as *mut T;
        std::ptr::write(ptr, value);

        ptr
    }
}

struct RefCell<T> {
    value_ptr: *mut T,
    borrows_ptr: *mut usize,
    is_borrowed_mut_ptr: *mut bool,
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        let value_ptr = put_on_heap(value);
        let borrows_ptr = put_on_heap(0);
        let is_borrowed_mut_ptr = put_on_heap(false);

        Self {
            value_ptr,
            borrows_ptr,
            is_borrowed_mut_ptr,
        }
    }

    pub fn borrow(&self) -> Ref<'_, T> {
        if *self.is_borrowed_mut() {
            panic!("unable to borrow - already borrowed mutably")
        }

        unsafe {
            let value = &*self.value_ptr;
            *self.borrows_ptr += 1;

            Ref {
                value,
                borrows_ptr: self.borrows_ptr,
            }
        }
    }

    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        if self.borrows() > &0 || *self.is_borrowed_mut() {
            panic!("unable to borrow mutably - there are borrows already")
        }

        unsafe {
            let value = &mut *self.value_ptr;
            *self.is_borrowed_mut_ptr = true;

            RefMut {
                value,
                is_borrowed_mut_ptr: self.is_borrowed_mut_ptr,
            }
        }
    }

    fn borrows(&self) -> &usize {
        unsafe {
            &*self.borrows_ptr
        }
    }

    fn value(&self) -> &T {
        unsafe {
            &*self.value_ptr
        }
    }

    fn is_borrowed_mut(&self) -> &bool {
        unsafe {
            &*self.is_borrowed_mut_ptr
        }
    }
}

impl<T> Drop for RefCell<T> {
    fn drop(&mut self) {
        use std::ptr::drop_in_place;

        unsafe {
            drop_in_place(self.borrows_ptr);
            drop_in_place(self.is_borrowed_mut_ptr);
            drop_in_place(self.value_ptr);
        }
    }
}

impl<T> Debug for RefCell<T> where T: Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RefCell")
            .field("borrows", self.borrows())
            .field("is_borrowed_mut", self.is_borrowed_mut())
            .field("value", self.value())
            .finish()
    }
}

struct Ref<'a, T> {
    value: &'a T,
    borrows_ptr: *mut usize,
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        unsafe {
            *self.borrows_ptr -= 1;
        }
    }
}

impl<T> Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

struct RefMut<'a, T> {
    value: &'a mut T,
    is_borrowed_mut_ptr: *mut bool,
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        unsafe {
            *self.is_borrowed_mut_ptr = false;
        }
    }
}

impl<T> Deref for RefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<T> DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct NotifyWhenDropped<'a> {
        is_dropped: &'a mut bool,
    }

    impl<'a> NotifyWhenDropped<'a> {
        fn new(is_dropped: &'a mut bool) -> Self {
            *is_dropped = false;

            Self {
                is_dropped,
            }
        }
    }

    impl Drop for NotifyWhenDropped<'_> {
        fn drop(&mut self) {
            *self.is_dropped = true;
        }
    }

    #[test]
    fn borrow() {
        let x = RefCell::new(5);

        let y = x.borrow();
        assert_eq!(5, *y);

        let z = x.borrow();
        assert_eq!(5, *z);
    }

    #[test]
    fn borrow_mut() {
        let x = RefCell::new(5);

        assert_eq!(5, *x.borrow());

        {
            *x.borrow_mut() = 10;
        }

        assert_eq!(10, *x.borrow());
    }

    #[test]
    #[should_panic]
    fn borrow_mut_while_borrowed_panics() {
        let x = RefCell::new(5);

        let _y = x.borrow();

        let _z = x.borrow_mut();
    }

    #[test]
    #[should_panic]
    fn borrow_while_borrowed_mutably_panics() {
        let x = RefCell::new(5);

        let _y = x.borrow_mut();

        let _z = x.borrow();
    }

    #[test]
    fn content_is_dropped_correctly() {
        let mut is_dropped = false;

        {
            let notify = NotifyWhenDropped::new(&mut is_dropped);
            let _x = RefCell::new(notify);
        }

        assert!(is_dropped);
    }

}

