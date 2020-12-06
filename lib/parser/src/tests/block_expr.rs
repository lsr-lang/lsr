use crate::tests::utils::parse;
use expect_test::expect;

#[test]
fn if_statements() {
    expect![[r#"
            IfStatement(
                (
                    Num(
                        "1",
                    ),
                    Num(
                        "2",
                    ),
                ),
            )
        "#]]
    .assert_debug_eq(&parse("if 1 then 2 end"));

    expect![[r#"
            IfElseStatement(
                (
                    Num(
                        "1",
                    ),
                    Num(
                        "2",
                    ),
                    Num(
                        "3",
                    ),
                ),
            )
        "#]]
    .assert_debug_eq(&parse("if 1 then 2 else 3 end"));

    expect![[r#"
            IfStatement(
                (
                    Num(
                        "1",
                    ),
                    BinOp(
                        (
                            Num(
                                "1",
                            ),
                            Subtraction(
                                "-",
                            ),
                            IfElseStatement(
                                (
                                    Identifier(
                                        "a",
                                    ),
                                    Float(
                                        "4f",
                                    ),
                                    Num(
                                        "6",
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            )
        "#]]
    .assert_debug_eq(&parse("if 1 then 1 - if a then 4f else 6 end end"));
}
