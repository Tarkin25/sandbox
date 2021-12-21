pub fn memoize<F, A, R>(mut f: F) -> impl FnMut(A) -> R
where 
F: FnMut(A) -> R,
A: PartialEq + Clone,
R: Clone
{
    let mut prev: Option<(A, R)> = None;

    move |arg: A| {
        if let Some((prev_arg, prev_result)) = &prev {
            if prev_arg == &arg {
                return prev_result.clone();
            }
        }

        let result = f(arg.clone());
        
        prev = Some((arg, result.clone()));

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn memoize_works() {
        let mut call_count = 0;
        let call_count_mut = &mut call_count;

        let expensive = |a: i32| {
            *call_count_mut += 1;
            a * 2
        };

        let mut memoized = memoize(expensive);

        assert_eq!(memoized(2), 4);
        assert_eq!(memoized(2), 4);
        assert_eq!(memoized(3), 6);

        drop(memoized);

        assert_eq!(call_count, 2);
    }
}
