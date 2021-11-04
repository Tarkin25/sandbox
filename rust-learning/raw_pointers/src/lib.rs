mod ref_cell;

struct IntWrapper {
    ptr: *mut i32,
}

impl IntWrapper {
    fn new(mut value: i32) -> Self {
        let ptr = &mut value as *mut i32;

        Self {
            ptr,
        }
    }

    fn get(&self) -> i32 {
        unsafe {
            *self.ptr
        }
    }

    fn add_one(&self) {
        unsafe {
            *self.ptr += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IntWrapper;

    #[test]
    fn add_one() {
        let w = IntWrapper::new(0);

        println!("initial value = {}", w.get());

        w.add_one();

        println!("value after add_one = {}", w.get());
    }

}
