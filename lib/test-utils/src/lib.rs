pub use assert_cmd::Command;
pub use ignore_macro::test_only_exists_binary;
pub use indoc::indoc;

#[macro_export]
macro_rules! assert_solve {
    (bin_name: $bin_name:expr, input: $input:expr, output: $expected:expr) => {
        // Ignore indent, new line at start and end
        let input = indoc! { $input };
        let expected = indoc! { $expected };

        // assert bin I/O
        let mut cmd = Command::cargo_bin($bin_name).unwrap();
        cmd.write_stdin(input);
        cmd.assert().stdout(expected).success();
    };
}

#[macro_export]
macro_rules! assert_test_cases {
    (bin: $bin_name:expr, $($case_name:ident: {input: $input:expr, output: $expected:expr}),*) => {
        use test_utils::*;

        $(
            #[test_only_exists_binary($bin_name)]
            fn $case_name() {
                assert_solve!(bin_name: $bin_name, input: $input, output: $expected);
            }
        )*
    };
}
