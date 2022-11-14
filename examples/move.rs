fn main() {
    let ryan = Person {
        name: "Ryan".into(),
        hobbies: vec!["coding".into(), "cooking".into(), "hiking".into()],
        age: 33,
    };
    let name = ryan.name;
    println!("My name is {name}");
    print_hobbies(ryan);
    println!("My age is {}", ryan.age);
}

fn print_hobbies(person: Person) {
    println!("My hobbies are:");
    for hobby in person.hobbies {
        println!("{hobby}");
    }
}

struct Person {
    name: String,
    hobbies: Vec<String>,
    age: u16,
}
