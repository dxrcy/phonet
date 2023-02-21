use phonet::{
    draft::{Message, Note},
    outcome::{FailKind::*, PassStatus::*, TestOutcome},
    Draft,
};

#[test]
fn example_outcome_works() {
    let file = include_str!("../examples/example.phonet");

    let outcome = Draft::from(file).expect("Failed to parse").run();

    assert_eq!(outcome.fail_count, 2);

    let mut list = outcome.messages.iter();

    assert_eq!(
        list.next().unwrap(),
        &Message::Info(Note("Invalid letters".to_string()))
    );
    assert_eq!(
        list.next().unwrap(),
        &Message::Test(TestOutcome {
            word: "taso".to_string(),
            intent: true,
            status: Pass,
        })
    );
    assert_eq!(
        list.next().unwrap(),
        &Message::Test(TestOutcome {
            word: "tyxo".to_string(),
            intent: false,
            status: Pass,
        })
    );

    assert_eq!(
        list.next().unwrap(),
        &Message::Info(Note("Examples of failing tests".to_string()))
    );
    assert_eq!(
        list.next().unwrap(),
        &Message::Test(TestOutcome {
            word: "tyxo".to_string(),
            intent: true,
            status: Fail(CustomReason(Note("Invalid letters".to_string()))),
        })
    );
    assert_eq!(
        list.next().unwrap(),
        &Message::Test(TestOutcome {
            word: "taso".to_string(),
            intent: false,
            status: Fail(ShouldBeInvalid),
        })
    );

    assert_eq!(
        list.next().unwrap(),
        &Message::Info(Note("Syllable structure".to_string()))
    );
    assert_eq!(
        list.next().unwrap(),
        &Message::Test(TestOutcome {
            word: "taso".to_string(),
            intent: true,
            status: Pass,
        })
    );
    assert_eq!(
        list.next().unwrap(),
        &Message::Test(TestOutcome {
            word: "kili".to_string(),
            intent: true,
            status: Pass,
        })
    );
    assert_eq!(
        list.next().unwrap(),
        &Message::Test(TestOutcome {
            word: "ano".to_string(),
            intent: false,
            status: Pass,
        })
    );
    assert_eq!(
        list.next().unwrap(),
        &Message::Test(TestOutcome {
            word: "taaso".to_string(),
            intent: false,
            status: Pass,
        })
    );

    assert_eq!(
        list.next().unwrap(),
        &Message::Info(Note("Some more tests".to_string()))
    );
    assert_eq!(
        list.next().unwrap(),
        &Message::Test(TestOutcome {
            word: "silo".to_string(),
            intent: true,
            status: Pass,
        })
    );
    assert_eq!(
        list.next().unwrap(),
        &Message::Test(TestOutcome {
            word: "tila".to_string(),
            intent: true,
            status: Pass,
        })
    );
    assert_eq!(
        list.next().unwrap(),
        &Message::Test(TestOutcome {
            word: "aka".to_string(),
            intent: false,
            status: Pass,
        })
    );
    assert_eq!(
        list.next().unwrap(),
        &Message::Test(TestOutcome {
            word: "axe".to_string(),
            intent: false,
            status: Pass,
        })
    );

    assert_eq!(
        list.next().unwrap(),
        &Message::Info(Note("No repeated letters".to_string()))
    );
    assert_eq!(
        list.next().unwrap(),
        &Message::Test(TestOutcome {
            word: "taso".to_string(),
            intent: true,
            status: Pass,
        })
    );
    assert_eq!(
        list.next().unwrap(),
        &Message::Test(TestOutcome {
            word: "taaso".to_string(),
            intent: false,
            status: Pass,
        })
    );
    assert_eq!(
        list.next().unwrap(),
        &Message::Test(TestOutcome {
            word: "ttaso".to_string(),
            intent: false,
            status: Pass,
        })
    );

    assert_eq!(
        list.next().unwrap(),
        &Message::Info(Note("2 tests *should* have failed!".to_string()))
    );

    assert_eq!(list.next(), None);
}
