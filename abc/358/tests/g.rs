use test_utils::assert_test_cases;

assert_test_cases! {
    bin: "g",
    case1: {
        input: r#"
            2 3 3
            1 2
            2 1 2
            3 4 5
        "#,
        output: r#"
            14
        "#
    },
    case2: {
        input: r#"
            2 2 1000000000
            2 1
            100 100
            100 99
        "#,
        output: r#"
            100000000000
        "#
    }
}
