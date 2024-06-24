use test_utils::assert_test_cases;

assert_test_cases! {
    bin: "a",
    case1: {
        input: r#"
            3
            Aoki
            Takahashi
            Takahashi
        "#,
        output: r#"
            2
        "#
    },
    case2: {
        input: r#"
            2
            Aoki
            Aoki
        "#,
        output: r#"
            0
        "#
    },
    case3: {
        input: r#"
            20
            Aoki
            Takahashi
            Takahashi
            Aoki
            Aoki
            Aoki
            Aoki
            Takahashi
            Aoki
            Aoki
            Aoki
            Takahashi
            Takahashi
            Aoki
            Takahashi
            Aoki
            Aoki
            Aoki
            Aoki
            Takahashi
        "#,
        output: r#"
            7
        "#
    }
}
