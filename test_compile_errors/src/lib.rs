#[test]
fn compile_test() {
    let t = trybuild::TestCases::new();
    t.compile_fail("src/compile-fail/*.rs");
}
