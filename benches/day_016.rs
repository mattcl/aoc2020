use aoc::error::Result;
use aoc::ticket::{Ticket, TicketValidator};
use aoc::util::load_input;
use criterion::{criterion_group, BenchmarkId, Criterion};
use std::str::FromStr;

pub fn bench(c: &mut Criterion) {
    let lines = load_input("016").expect("could not load input");
    let mut parts = lines.split(|line| line.is_empty());

    let rules = parts.next().expect("invalid input");

    let validator = TicketValidator::from_input(&rules).expect("could not make validator");

    let _our_ticket = Ticket::from_str(
        parts
            .next()
            .expect("invalid input, missing our ticket")
            .get(1)
            .expect("invalid input, missing our ticket values"),
    )
    .expect("could not make our ticket");

    let other_tickets = parts.next().expect("invalid input, no nearby tickets")[1..]
        .iter()
        .map(|ticket| Ticket::from_str(ticket))
        .collect::<Result<Vec<Ticket>>>()
        .expect("could not make other tickets");

    let mut group = c.benchmark_group("016 ticket translation part 1");
    group.bench_function(BenchmarkId::new("find invalid tickets", "normal"), |b| {
        b.iter(|| {
            let mut other_tickets = other_tickets.clone();
            let mut _sum = 0;
            for ticket in other_tickets.iter_mut() {
                if let Some(vals) = validator.validate(ticket) {
                    _sum += vals.iter().sum::<usize>();
                }
            }
        })
    });

    group.finish();

    let mut group = c.benchmark_group("016 ticket translation part 2");
    group.bench_function(BenchmarkId::new("determine_rule_order", "fast"), |b| {
        let mut _sum = 0;
        let mut other_tickets = other_tickets.clone();
        for ticket in other_tickets.iter_mut() {
            if let Some(vals) = validator.validate(ticket) {
                _sum += vals.iter().sum::<usize>();
            }
        }

        let valid_tickets = other_tickets
            .into_iter()
            .filter(|ticket| ticket.is_valid)
            .collect::<Vec<Ticket>>();

        b.iter(|| {
            let mut validator = validator.clone();
            validator
                .determine_rule_order_fast(&valid_tickets)
                .expect("could not determine rule order");
        })
    });

    group.finish();
}

criterion_group!(benches, bench);
