use test_utils::assert_test_cases;

assert_test_cases! {
    bin: "b",
    case1: {
        input: r#"
            15000
        "#,
        output: r#"
            65
        "#
    },
    case2: {
        input: r#"
            75000
        "#,
        output: r#"
            89
        "#
    },
    case3: {
        input: r#"
            200
        "#,
        output: r#"
            2
        "#
    }
}
