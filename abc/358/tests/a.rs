use test_utils::assert_test_cases;

assert_test_cases! {
    bin: "a",
    case1: {
        input: r#"
            AtCoder Land
        "#,
        output: r#"
            Yes
        "#
    },
    case2: {
        input: r#"
            CodeQUEEN Land
        "#,
        output: r#"
            No
        "#
    },
    case3: {
        input: r#"
            aTcodeR lANd
        "#,
        output: r#"
            No
        "#
    }
}
