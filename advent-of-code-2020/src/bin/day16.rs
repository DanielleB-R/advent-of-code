use advent_of_code_2020::day16;
use std::fs;

fn main() {
    let input = fs::read_to_string("day16-input.dat").expect("file system error");

    let (fields, my_ticket, nearby_tickets) = day16::parse_input(&input);

    let error_rate: usize = day16::find_invalid_values(&fields, &nearby_tickets)
        .into_iter()
        .sum();

    println!("{}", error_rate);

    let nearby_tickets = day16::discard_invalid_tickets(&fields, nearby_tickets);

    let field_definitions = day16::classify(&fields, &nearby_tickets);
    let my_ticket_product: usize = field_definitions
        .iter()
        .filter(|(k, _)| k.starts_with("departure"))
        .map(|(_, v)| my_ticket[*v])
        .product();

    println!("{}", my_ticket_product);
}
