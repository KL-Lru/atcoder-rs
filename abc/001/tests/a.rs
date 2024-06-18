use test_utils::assert_test_cases;

assert_test_cases! {
    bin: "a",
    case1: {
        input: r#"
            15
            10
        "#,
        output: r#"
            5
        "#
    },
    case2: {
        input: r#"
            0
            0
        "#,
        output: r#"
            0
        "#
    },
    case3: {
        input: r#"
            5
            20
        "#,
        output: r#"
            -15
        "#
    }
}
