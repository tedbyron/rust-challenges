#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod _2020;

use _2020::roboscript_interpreter::execute;

fn main() {
    macro_rules! expect_equal {
        ($actual:expr, $expected:expr $(,)*) => {{
            let actual = $actual;
            let expected = $expected;
            assert_eq!(
                actual, expected,
                "\ngot:\n{}\n\nexpected:\n{}\n",
                actual, expected
            );
        }};
    }
    expect_equal!(execute(""), "*");
    expect_equal!(execute("FFFFF"), "******");
    expect_equal!(
        execute("FFFFFLFFFFFLFFFFFLFFFFFL"),
        "******\r\n*    *\r\n*    *\r\n*    *\r\n*    *\r\n******",
    );
    expect_equal!(
        execute("LFFFFFRFFFRFFFRFFFFFFF"),
        "    ****\r\n    *  *\r\n    *  *\r\n********\r\n    *   \r\n    *   ",
    );
    expect_equal!(
        execute("LF5RF3RF3RF7"),
        "    ****\r\n    *  *\r\n    *  *\r\n********\r\n    *   \r\n    *   ",
    );
    expect_equal!(execute("RFLF7"), "*       \r\n********",);
}
