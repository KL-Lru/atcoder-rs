use test_utils::assert_test_cases;

assert_test_cases! {
    bin: "c",
    case1: {
        input: r#"
            3 5
            oooxx
            xooox
            xxooo
        "#,
        output: r#"
            2
        "#
    },
    case2: {
        input: r#"
            3 2
            oo
            ox
            xo
        "#,
        output: r#"
            1
        "#
    },
    case3: {
        input: r#"
            8 6
            xxoxxo
            xxoxxx
            xoxxxx
            xxxoxx
            xxoooo
            xxxxox
            xoxxox
            oxoxxo
        "#,
        output: r#"
            3
        "#
    }
}
