use test_utils::assert_test_cases;

assert_test_cases! {
    bin: "d",
    case1: {
        input: r#"
            4
            1148-1210
            1323-1401
            1106-1123
            1129-1203
        "#,
        output: r#"
            1105-1210
            1320-1405
        "#
    },
    case2: {
        input: r#"
            1
            0000-2400
        "#,
        output: r#"
            0000-2400
        "#
    },
    case3: {
        input: r#"
            6
            1157-1306
            1159-1307
            1158-1259
            1230-1240
            1157-1306
            1315-1317
        "#,
        output: r#"
            1155-1310
            1315-1320
        "#
    }
}
