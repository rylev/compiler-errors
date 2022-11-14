fn main() {
    let mut pantry = vec![Food::Avocados(3)];
    stock(&mut pantry);
}

fn stock(pantry: &mut Vec<Food>) {
    match pantry.iter_mut().find(|f| matches!(f, Food::Avocados(_))) {
        Some(Food::Avocados(n)) if *n > 0 => {
            pantry.push(Food::Chips);
            *n = *n * 2;
        }
        None => pantry.push(Food::Cake),
    };
}

#[derive(PartialEq)]
enum Food {
    Avocados(u16),
    Chips,
    Cake,
}
