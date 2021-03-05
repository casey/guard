#[macro_export]
macro_rules! assert_guard {
    ($($input:tt)*) => {
        $crate::guard!(
            $($input)* else { $crate::assert_guard_panic!($($input)*) }
        );
    };
}

#[macro_export]
macro_rules! assert_guard_panic {
    (let $pattern:pat = $expression:expr) => {
        panic!(
            "assertion failed: `let {} = {}`\n  matched value: `{:?}`",
            stringify!($pattern),
            stringify!($expression),
            $expression,
        )
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn should_match() {
        let val: Option<()> = None;
        assert_guard!(let Option::None = val);
    }

    #[test]
    fn should_bind() {
        let val = Some(42);
        assert_guard!(let Some(n) = val);
        assert_eq!(n, 42);
    }

    #[test]
    #[should_panic]
    fn should_panic() {
        let val: Option<()> = None;
        assert_guard!(let Some(_) = val);
    }

    #[test]
    #[should_panic(expected = "Some(_)")]
    fn panic_message_should_include_pattern() {
        let val: Option<()> = None;
        assert_guard!(let Some(_) = val);
    }

    #[test]
    #[should_panic(expected = "val")]
    fn panic_message_should_include_matched_expression() {
        let val: Option<()> = None;
        assert_guard!(let Some(_) = val);
    }

    #[test]
    #[should_panic(expected = "None")]
    fn panic_message_should_include_mismatching_value() {
        let val: Option<()> = None;
        assert_guard!(let Some(_) = val);
    }

    #[test]
    #[should_panic(
        expected = "assertion failed: `let Some(_) = foo(bar)`\n  matched value: `None`"
    )]
    fn should_have_nice_panic_message() {
        let bar = true;
        fn foo(_: bool) -> Option<()> {
            None
        }
        assert_guard!(let Some(_) = foo(bar));
    }

    #[test]
    #[ignore]
    fn should_not_evaluate_matched_expression_twice() {
        use std::sync::{Arc, Mutex};
        let n = Arc::new(Mutex::new(0));
        let result = std::panic::catch_unwind(|| {
            assert_guard!(let Option::None = {
            let mut value = n.lock().unwrap();
            *value += 1;
            Some(*value)
          });
        });
        assert!(result.is_err());
        assert_eq!(*n.lock().unwrap(), 1);
    }

    // todo: add comment to PR about using $crate, which only works since 1.30
    // todo: worry about double evaluation of expression
    // todo:
    // - worry about things that don't implement Debug
    // - http://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html#quick-recap-method-resolution
    // todo: document that only one guard! macro syntax is supported
}
