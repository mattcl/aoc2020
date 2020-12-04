use aoc::passport::Passport;
use aoc::util::load_input;

fn main() {
    let lines = load_input("004").expect("could not load input");
    let res = Passport::from_input(&lines);

    let valid_passports = res
        .into_iter()
        .filter(|passport| passport.is_ok())
        .map(|passport| passport.unwrap())
        .collect::<Vec<Passport>>();

    println!("part 1: {}", valid_passports.len());

    let count = valid_passports
        .into_iter()
        .filter(|passport| passport.validate().is_ok())
        .count();

    println!("part 2: {}", count);
}
