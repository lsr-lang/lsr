use crate::tests::utils::parse;
use expect_test::expect;

#[test]
fn literals() {
    expect![[r#"
            Num(
                "4",
            )
        "#]]
    .assert_debug_eq(&parse("4"));

    expect![[r#"
            Dec(
                "4.",
            )
        "#]]
    .assert_debug_eq(&parse("4."));

    expect![[r#"
            Float(
                "4.f",
            )
        "#]]
    .assert_debug_eq(&parse("4.f"));

    expect![[r#"
            Operator(
                "`==>`",
            )
        "#]]
    .assert_debug_eq(&parse("`==>`"));
    expect![[r#"
            Atom(
                ":Atom",
            )
        "#]]
    .assert_debug_eq(&parse(":Atom"));

    expect![[r#"
            Identifier(
                "identifier",
            )
        "#]]
    .assert_debug_eq(&parse("identifier"));
}
