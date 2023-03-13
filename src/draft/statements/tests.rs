use super::*;

#[test]
fn split_statements_works() {
    let lhs = split_statements(
        &[
            "foo bar & abc 123;",
            "baz &",
            "123 456",
            " abc",
            ";",
            "a &",
            "",
            "what",
            "; hello;1; 2;3 & a;4",
            "pls",
            "# a comment",
            "hello & a & b",
            ";",
            "# comment no-multiline & ect",
            "some statement # with a hashtag",
            "",
            "# a comment; and a statement",
        ]
        .join("\n"),
    );

    let rhs = vec![
        ("foo bar  abc 123".to_string(), 1),
        ("baz 123 456 abc".to_string(), 2),
        ("a what".to_string(), 6),
        (" hello".to_string(), 9),
        ("1".to_string(), 9),
        (" 2".to_string(), 9),
        ("3  a".to_string(), 9),
        ("4".to_string(), 9),
        ("pls".to_string(), 10),
        ("hello  a & b".to_string(), 12),
        ("some statement # with a hashtag".to_string(), 15),
        (" and a statement".to_string(), 17),
    ];

    // Debugging
    for (i, stat) in rhs.iter().enumerate() {
        println!("\x1b[36;1m---\x1b[0m");
        println!(" \x1b[33m{}\x1b[0m {}", stat.1, stat.0);

        match lhs.get(i) {
            Some(stat) => println!(" \x1b[33m{}\x1b[0m {}", stat.1, stat.0),
            None => println!(" \x1b[31;1mNone\x1b[0m"),
        }

        if Some(stat) != lhs.get(i) {
            println!(" \x1b[31m^^^^\x1b[0m");
        }
    }
    println!("\x1b[36;1m---\x1b[0m");

    // assert_eq!(lhs, rhs);
}
