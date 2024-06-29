use test_utils::assert_test_cases;

assert_test_cases! {
    bin: "c",
    case1: {
        input: r#"
            5 0
            2 5
        "#,
        output: r#"
            5
        "#
    },
    case2: {
        input: r#"
            3 1
            4 1
        "#,
        output: r#"
            0
        "#
    },
    case3: {
        input: r#"
            2552608206527595 5411232866732612
            771856005518028 7206210729152763
        "#,
        output: r#"
            1794977862420151
        "#
    }
}
