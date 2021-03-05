#[macro_export]
macro_rules! assert_guard {
    (let $pattern:pat = $expression:expr) => {
        $crate::guard!(
            let $pattern = $expression else { panic!() }
        );
    };
}

#[cfg(test)]
mod test {
    // use super::*;

    #[test]
    fn should_match() {
        let val: Option<()> = None;
        assert_guard!(let Option::None = val);
        // todo: remove
        let _ = val;
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
        expected = "assertion failed: `let Some(_) = bar(foo)`\n  matched value: `None`"
    )]
    #[ignore]
    fn should_have_nice_panic_message() {
        let bar = true;
        fn foo(_: bool) -> Option<()> {
            None
        }
        assert_guard!(let Some(_) = foo(bar));
    }

    #[test]
    #[ignore]
    fn should_work_for_non_debug_types() {}

    // todo: figure out different guard synteces
    // todo: negation syntax?
    // todo: add comment to PR about using $crate, which only works since 1.30
}
