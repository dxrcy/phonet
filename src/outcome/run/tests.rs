use fancy_regex_macro::regex;

use super::*;
use crate::draft::Message;

/// Get example list of rules for testing
fn get_example_rules() -> Vec<Rule> {
    vec![
        Rule {
            pattern: regex!("a").clone(),
            intent: true,
            note: Some(Note("Should contain ⟨a⟩".to_string())),
        },
        Rule {
            pattern: regex!("x").clone(),
            intent: false,
            note: None,
        },
    ]
}

/// All tests should pass
#[test]
fn run_all_successful() {
    let draft = Draft {
        messages: vec![
            //s
            Info(Note("this is a note".to_string())),
            //
            Test(TestDraft {
                word: "abc".to_string(),
                intent: true,
            }),
            //
            Info(Note("another note".to_string())),
            //
            Test(TestDraft {
                word: "ax".to_string(),
                intent: false,
            }),
            //
            Test(TestDraft {
                word: "hello".to_string(),
                intent: false,
            }),
        ],
        //
        rules: get_example_rules(),
        mode: crate::Mode::Romanized,
        test_count: 3,
    };

    let outcome = draft.run();

    assert_eq!(outcome.fail_count, 0);

    let mut list = outcome.list.iter();

    assert_eq!(
        list.next(),
        Some(&Message::Info(Note("this is a note".to_string())))
    );

    assert_eq!(
        list.next(),
        Some(&Message::Test(TestOutcome {
            word: "abc".to_string(),
            intent: true,
            status: Pass
        }))
    );

    assert_eq!(
        list.next(),
        Some(&Message::Info(Note("another note".to_string())))
    );

    assert_eq!(
        list.next(),
        Some(&Message::Test(TestOutcome {
            word: "ax".to_string(),
            intent: false,
            status: Pass
        }))
    );

    assert_eq!(
        list.next(),
        Some(&Message::Test(TestOutcome {
            word: "hello".to_string(),
            intent: false,
            status: Pass
        }))
    );

    assert_eq!(list.next(), None);
}

/// All tests should fail
#[test]
fn run_all_failing() {
    let draft = Draft {
        messages: vec![
            //s
            Info(Note("this is a note".to_string())),
            //
            Test(TestDraft {
                word: "abc".to_string(),
                intent: false,
            }),
            //
            Info(Note("another note".to_string())),
            //
            Test(TestDraft {
                word: "ax".to_string(),
                intent: true,
            }),
            //
            Test(TestDraft {
                word: "hello".to_string(),
                intent: true,
            }),
        ],
        //
        rules: get_example_rules(),
        mode: crate::Mode::Romanized,
        test_count: 3,
    };

    let outcome = draft.run();

    assert_eq!(outcome.fail_count, 3);

    let mut list = outcome.list.iter();

    assert_eq!(
        list.next(),
        Some(&Message::Info(Note("this is a note".to_string())))
    );

    assert_eq!(
        list.next(),
        Some(&Message::Test(TestOutcome {
            word: "abc".to_string(),
            intent: false,
            status: Fail(ShouldBeInvalid)
        }))
    );

    assert_eq!(
        list.next(),
        Some(&Message::Info(Note("another note".to_string())))
    );

    assert_eq!(
        list.next(),
        Some(&Message::Test(TestOutcome {
            word: "ax".to_string(),
            intent: true,
            status: Fail(NoReasonGiven)
        }))
    );

    assert_eq!(
        list.next(),
        Some(&Message::Test(TestOutcome {
            word: "hello".to_string(),
            intent: true,
            status: Fail(CustomReason(Note("Should contain ⟨a⟩".to_string())))
        }))
    );

    assert_eq!(list.next(), None);
}

/// Tests that should match (valid)
#[test]
fn run_test_all_valid() {
    let rules = get_example_rules();

    assert_eq!(
        run_test(
            TestDraft {
                word: "abc".to_string(),
                intent: true,
            },
            &rules
        ),
        TestOutcome {
            word: "abc".to_string(),
            intent: true,
            status: Pass,
        }
    );

    assert_eq!(
        run_test(
            TestDraft {
                word: "hello".to_string(),
                intent: false,
            },
            &rules
        ),
        TestOutcome {
            word: "hello".to_string(),
            intent: false,
            status: Pass,
        }
    );

    assert_eq!(
        run_test(
            TestDraft {
                word: "ax".to_string(),
                intent: false,
            },
            &rules
        ),
        TestOutcome {
            word: "ax".to_string(),
            intent: false,
            status: Pass,
        }
    );
}

/// Tests that should not match (invalid)
#[test]
fn run_test_all_invalid() {
    let rules = get_example_rules();

    assert_eq!(
        run_test(
            TestDraft {
                word: "abc".to_string(),
                intent: false,
            },
            &rules
        ),
        TestOutcome {
            word: "abc".to_string(),
            intent: false,
            status: Fail(ShouldBeInvalid),
        }
    );

    assert_eq!(
        run_test(
            TestDraft {
                word: "hello".to_string(),
                intent: true,
            },
            &rules
        ),
        TestOutcome {
            word: "hello".to_string(),
            intent: true,
            status: Fail(CustomReason(Note("Should contain ⟨a⟩".to_string()))),
        }
    );

    assert_eq!(
        run_test(
            TestDraft {
                word: "ax".to_string(),
                intent: true,
            },
            &rules
        ),
        TestOutcome {
            word: "ax".to_string(),
            intent: true,
            status: Fail(NoReasonGiven),
        }
    );
}

#[test]
fn validate_test_works() {
    let rules = get_example_rules();

    // Test should be valid
    assert_eq!(validate_test("abc", &rules), Valid);

    // Test should be invalid - with custom reason
    assert_eq!(
        validate_test("boc", &rules),
        Invalid(Some(Note("Should contain ⟨a⟩".to_string())))
    );

    // Test should be invalid - with no reason given
    assert_eq!(validate_test("axe", &rules), Invalid(None));
}

/// Tests that should match (valid)
#[test]
fn get_status_all_valid() {
    let my_note = Note("Some reason".to_string());

    // Valid test matches (pass)
    assert_eq!(get_status(Valid, true), Pass);

    // Valid test does not match (fail)...
    // ...with custom reason
    assert_eq!(
        get_status(Invalid(Some(my_note)), true),
        Fail(CustomReason(Note("Some reason".to_string())))
    );
    // ...with no reason given
    assert_eq!(get_status(Invalid(None), true), Fail(NoReasonGiven));
}

/// Tests that should not match (invalid)
#[test]
fn get_status_all_invalid() {
    let my_note = Note("Some reason".to_string());

    // Invalid test does not match (pass)
    assert_eq!(get_status(Valid, false), Fail(ShouldBeInvalid));

    // Invalid test matches (fail)...
    // ...with custom reason
    assert_eq!(get_status(Invalid(Some(my_note)), false), Pass);
    // ...or no reason given (same status)
    assert_eq!(get_status(Invalid(None), false), Pass);
}
