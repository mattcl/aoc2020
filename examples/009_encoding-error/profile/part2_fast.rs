use aoc::util::load_input;
use aoc::xmas::Document;

fn main() {
    let lines = load_input("009").expect("could not load input");
    let document = Document::new(&lines).expect("could not load document");
    let outlier = document.find_outlier(25).unwrap();
    println!(
        "{}",
        document
            .find_weakness(outlier)
            .expect("could not find weakness")
    );
}
