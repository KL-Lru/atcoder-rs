use test_utils::assert_test_cases;

assert_test_cases! {
    bin: "b",
    case1: {
        input: r#"
            3
            1 2 1 3 2 3
        "#,
        output: r#"
            2
        "#
    },
    case2: {
        input: r#"
            2
            1 1 2 2
        "#,
        output: r#"
            0
        "#
    },
    case3: {
        input: r#"
            4
            4 3 2 3 2 1 4 1
        "#,
        output: r#"
            3
        "#
    }
}
