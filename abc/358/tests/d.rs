use test_utils::assert_test_cases;

assert_test_cases! {
    bin: "d",
    case1: {
        input: r#"
            4 2
            3 4 5 4
            1 4
        "#,
        output: r#"
            7
        "#
    },
    case2: {
        input: r#"
            3 3
            1 1 1
            1000000000 1000000000 1000000000
        "#,
        output: r#"
            -1
        "#
    },
    case3: {
        input: r#"
            7 3
            2 6 8 9 5 1 11
            3 5 7
        "#,
        output: r#"
            19
        "#
    }

}
