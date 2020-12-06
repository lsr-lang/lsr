use crate::tests::utils::parse;
use expect_test::expect;

#[test]
fn small_prog() {
    let token = r#"
        let a = 5;
        let b = 6;
        a - b
    "#;

    expect![[r#""#]].assert_debug_eq(&parse(token));
}
