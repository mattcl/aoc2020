use aoc::error::Result;
use aoc::ticket::{Ticket, TicketValidator};
use aoc::util::load_input;
use std::str::FromStr;

fn main() {
    let lines = load_input("016").expect("could not load input");
    let mut parts = lines.split(|line| line.is_empty());

    let rules = parts.next().expect("invalid input");

    let mut validator = TicketValidator::from_input(&rules).expect("could not make validator");

    let our_ticket = Ticket::from_str(
        parts
            .next()
            .expect("invalid input, missing our ticket")
            .get(1)
            .expect("invalid input, missing our ticket values")
        ).expect("could not make our ticket");

    let mut other_tickets = parts
        .next()
        .expect("invalid input, no nearby tickets")
        [1..]
        .iter()
        .map(|ticket| Ticket::from_str(ticket))
        .collect::<Result<Vec<Ticket>>>()
        .expect("could not make other tickets");


    let mut sum = 0;
    for ticket in other_tickets.iter_mut() {
        if let Some(vals) = validator.validate(ticket) {
            sum += vals.iter().sum::<usize>();
        }
    }

    println!("part 1: {}", sum);

    let valid_tickets = other_tickets.into_iter().filter(|ticket| ticket.is_valid).collect::<Vec<Ticket>>();

    // println!("{:#?}", validator.make_col_map(&valid_tickets).iter().map(|col| col.len()).collect::<Vec<usize>>());

    validator.determine_rule_order_fast(&valid_tickets).expect("could not determine rule order");

    let res = validator.rules
        .iter()
        .enumerate()
        .filter(|(_, r)| r.name.starts_with("departure"))
        .map(|(index, _)| our_ticket.values[index])
        .product::<usize>();

    println!("part 2: {}", res);
}
