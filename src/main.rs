use std::env;
fn main() {
    for item in env::args().skip(1) {
        println!("{}", mock(item))
    }
}

fn mock(input: String) -> String {
    let mut make_uppercase = false;
    input
        .chars()
        .map(|c| {
            if c.is_alphabetic() {
                make_uppercase = !make_uppercase;
            }
            match c {
                c if make_uppercase && c.is_lowercase() => c.to_uppercase().to_string(),
                c if !make_uppercase && c.is_uppercase() => c.to_lowercase().to_string(),
                _ => c.to_string(),
            }
        })
        .collect::<String>()
}
