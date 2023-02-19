use std::fs;

fn main() {
    let path = "./phonet";

    let file = fs::read_to_string(path).expect("Could not read file");

    let draft = phonet::Draft::from(&file).expect("Failed to parse file");

    println!("{:#?}", draft);

    let outcome = draft.run();

    println!("{:#?}", outcome);

    outcome.display(Default::default(), true);
}
