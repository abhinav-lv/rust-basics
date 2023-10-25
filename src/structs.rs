pub fn structs() {
    // struct_update();
    tuple_struct();
    let bleh = Rectangle {
        width: 450,
        length: 300
    };
    dbg!(calcRecArea(bleh));
}

fn calcRecArea(rectangle: Rectangle) -> u32 {
    rectangle.length * rectangle.width
}

// Tuple struct
struct Color(i32, i32, i32);

fn tuple_struct() {
    let white = Color(255, 255, 255);
    // dbg!(white.0, white.1, white.2);
    let Color(a, b, c) = white;
    dbg!(a, b, c);
}

// Struct update (rust version of js spread operator)
fn struct_update() {
    let user1 = Person {
        name: String::from("abhinav"),
        email: Some(String::from("abhinavlv@yahoo.com")),
        age: 20,
        phone: None,
    };
    display_person(&user1);
    let user2 = Person {
        phone: Some(String::from("9176376941")),
        ..user1
    };
    println!("------------");
    display_person(&user2);
}

// Display instance of Person
fn display_person(person: &Person) {
    println!("{}", person.name);
    match &person.email {
        Some(email) => println!("{email}"),
        None => {}
    };
    println!("{}", person.age);
    match &person.phone {
        Some(phone) => println!("{phone}"),
        None => {}
    };
}

// Person struct
struct Person {
    name: String,
    email: Option<String>,
    age: u8,
    phone: Option<String>,
}

struct Rectangle {
    width: u32,
    length: u32,
}
