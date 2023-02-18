use phonet::Draft;

#[test]
fn tokipona_should_pass() {
    let file = include_str!("../examples/tokipona.phonet");

    assert_eq!(
        Draft::from(file).expect("Failed to parse").run().fail_count,
        0
    );
}
