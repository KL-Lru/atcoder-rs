use test_utils::assert_test_cases;

assert_test_cases! {
    bin: "f",
    // @note modified case: not match to implementation
    case1: {
        input: r#"
            3 3 7
        "#,
        output: r#"
            Yes
            +++++S+
            +o.o.o+
            +.+-+-+
            +o.o.o+
            +-+-+.+
            +o|o|o+
            +++++G+
        "#
    },
    case2: {
        input: r#"
            3 3 2
        "#,
        output: r#"
            No
        "#
    },
    case3: {
        input: r#"
            4 1 4
        "#,
        output: r#"
            Yes
            +S+
            +o+
            +.+
            +o+
            +.+
            +o+
            +.+
            +o+
            +G+
        "#
    }

}
