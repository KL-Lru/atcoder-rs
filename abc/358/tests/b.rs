use test_utils::assert_test_cases;

assert_test_cases! {
    bin: "b",
    case1: {
        input: r#"
            3 4
            0 2 10
        "#,
        output: r#"
            4
            8
            14
        "#
    },
    case2: {
        input: r#"
            3 3
            1 4 7
        "#,
        output: r#"
            4
            7
            10
        "#
    },
    case3: {
        input: r#"
            10 50000
            120190 165111 196897 456895 540000 552614 561627 743796 757613 991216
        "#,
        output: r#"
            170190
            220190
            270190
            506895
            590000
            640000
            690000
            793796
            843796
            1041216
        "#
    }
}
