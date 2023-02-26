use phonet::{DisplayLevel, Draft};

#[test]
fn tokipona_should_pass() {
    let file = include_str!("../examples/tokipona.phonet");

    let draft = Draft::from(file).expect("Failed to parse");

    assert_eq!(draft.name, Some("Toki Pona".to_string()));

    let outcome = draft.run();

    assert_eq!(outcome.fail_count, 0);

    // Should only panic if there is a stdout problem
    outcome.display(DisplayLevel::OnlyFails, true);
}
