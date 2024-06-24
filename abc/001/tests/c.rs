use test_utils::assert_test_cases;

assert_test_cases! {
    bin: "c",
    case1: {
        input: r#"
            2750 628
        "#,
        output: r#"
            W 5
        "#
    },
    case2: {
        input: r#"
            161 8
        "#,
        output: r#"
            C 0
        "#
    },
    case3: {
        input: r#"
            3263 15
        "#,
        output: r#"
            NNW 1
        "#
    },
    case4: {
        input: r#"
            1462 1959
        "#,
        output: r#"
            SE 12
        "#
    },
    case5: {
        input: r#"
            1687 1029
        "#,
        output: r#"
            SSE 8
        "#
    },
    case6: {
        input: r#"
            2587 644
        "#,
        output: r#"
            WSW 5
        "#
    },
    case7: {
        input: r#"
            113 201
        "#,
        output: r#"
            NNE 3
        "#
    },
    case8: {
        input: r#"
            2048 16
        "#,
        output: r#"
            SSW 1
        "#
    }
}
