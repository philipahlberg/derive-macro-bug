use trybuild::TestCases;

#[test]
fn tests() {
    let cases = TestCases::new();
    cases.compile_fail("tests/cases/generics.rs");
}
