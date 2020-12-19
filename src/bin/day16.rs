use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone)]
struct Field {
    name: String,
    ranges: Vec<(i32, i32)>,
}

impl Field {
    fn contains(&self, n: &i32) -> bool {
        self.ranges.iter().any(|(s, e)| (s..=e).contains(&n))
    }
}

fn parse_field(ln: &str) -> Field {
    let parts: Vec<_> = ln.split(": ").collect();
    let name = parts[0].to_string();
    let ranges: Vec<_> = parts[1].split(" or ").collect();
    let r1: Vec<_> = ranges[0].split('-').collect();
    let r2: Vec<_> = ranges[1].split('-').collect();
    Field {
        name,
        ranges: vec![
            (r1[0].parse().unwrap(), r1[1].parse().unwrap()),
            (r2[0].parse().unwrap(), r2[1].parse().unwrap()),
        ],
    }
}

fn invalid_fields(ticket: &[i32], fields: &[Field]) -> Vec<i32> {
    ticket
        .iter()
        .filter(|num| !fields.iter().any(|f| f.contains(*num)))
        .cloned()
        .collect()
}

fn is_valid_ticket(ticket: &[i32], fields: &[Field]) -> bool {
    invalid_fields(ticket, fields).is_empty()
}

fn main() {
    let contents = fs::read_to_string("day16.txt").expect("Something went wrong reading the file");

    let sections: Vec<_> = contents.split("\n\n").collect();

    let fields: Vec<Field> = sections[0].lines().map(parse_field).collect();

    let my_ticket: Vec<i32> = sections[1]
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let nearby_tickets: Vec<Vec<i32>> = sections[2]
        .lines()
        .skip(1)
        .map(|ln| ln.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    let ticket_scanning_error_rate: i32 = nearby_tickets
        .iter()
        .flat_map(|ticket| invalid_fields(ticket, &fields))
        .sum();

    println!(
        "Part 1, scanning error rate: {}",
        ticket_scanning_error_rate
    );

    assert!(is_valid_ticket(&my_ticket, &fields));

    let mut valid_tickets: Vec<Vec<i32>> = nearby_tickets
        .into_iter()
        .filter(|t| is_valid_ticket(t, &fields))
        .collect();
    valid_tickets.push(my_ticket.clone());
    let valid_tickets = valid_tickets;

    let mut mapping: HashMap<String, usize> = HashMap::new();
    let mut seen_rows: HashSet<usize> = HashSet::new();
    let mut fields = fields;

    while seen_rows.len() != my_ticket.len() {
        for ticket_row in 0..my_ticket.len() {
            if seen_rows.contains(&ticket_row) {
                continue;
            }
            let mut matches: Vec<Field> = fields
                .iter()
                .filter(|f| valid_tickets.iter().all(|t| f.contains(&t[ticket_row])))
                .cloned()
                .collect();
            if matches.len() == 1 {
                let field = matches.pop().unwrap();
                match field {
                    Field { name, .. } => {
                        fields = fields.into_iter().filter(|f| f.name != name).collect();
                        mapping.insert(name, ticket_row);
                        seen_rows.insert(ticket_row);
                    }
                }
            }
        }
    }

    println!(
        "Part 2, departure product: {}",
        mapping
            .into_iter()
            .filter(|(k, _)| k.starts_with("departure "))
            .map(|(_, v)| my_ticket[v] as i64)
            .product::<i64>()
    );
}
