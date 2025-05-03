use docgen::generator::*;

fn main() {
    for binary in binary_list() {
        generate_binary_module_rs(&binary);
    }
    generate_module_rs();
    generate_lib_rs();
}
