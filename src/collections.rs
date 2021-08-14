use std::borrow::Borrow;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::fmt::Formatter;

pub fn vector() {
    let mut v1: Vec<&str> = Vec::new();

    v1.push("Hello");
    v1.push("World");

    let v2 = vec!["What", "is", "up", "?"];

    for word in v2 {
        println!("{}", word)
    }
}

pub fn queue() {
    let mut queue: VecDeque<u16> = VecDeque::with_capacity(3);

    queue.push_back(5);
    queue.push_front(3);
    queue.push_back(8);
    // queue.push_front(2); // allocation

    println!("queue capacity: {}", queue.capacity());
    for (index, value) in queue.iter().enumerate() {
        println!("i: {}, v: {}", index, value)
    }

    queue.swap(0, 2);

    println!("queue: {:?}", queue);

    while !queue.is_empty() {
        println!("queue pop: {}", queue.pop_front().unwrap())
    }

    queue.extend(0..15);
    // queue.extend(0..=15);
    println!("queue capacity: {}", queue.capacity());
    println!("queue: {:?}", queue);

    queue.truncate(5);
    queue.shrink_to_fit();
    println!("queue capacity: {}", queue.capacity());
    println!("queue: {:?}", queue);
}

pub fn hash_set() {
    let mut albums: HashSet<&str> = HashSet::new();

    albums.insert("Stranger in the Alps");
    albums.insert("The Rearview Mirror");
    albums.insert("Faded Touch");
    albums.insert("Lagrange Points");

    albums.remove("Stranger in the Alps");

    if !albums.contains("Stranger in the Alps") {
        println!(
            "We have {} albums but '{}' is not one of them.",
            albums.len(),
            "Stranger in the Alps"
        )
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum CarType {
    Sedan,
    Coupe,
    Hatchback,
    Suv,
}

#[derive(Debug)]
struct Car {
    manufacturer: String,
    model: String,
}

impl fmt::Display for Car {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.manufacturer, self.model)
    }
}

pub fn hash_map() {
    let mut cars = HashMap::new();

    cars.insert(
        CarType::Sedan,
        Car {
            manufacturer: "BMW".to_string(),
            model: "3".to_string(),
        },
    );

    cars.insert(
        CarType::Coupe,
        Car {
            manufacturer: "Porsche".to_string(),
            model: "911".to_string(),
        },
    );

    cars.insert(
        CarType::Hatchback,
        Car {
            manufacturer: "Volvo".to_string(),
            model: "V40".to_string(),
        },
    );

    cars.insert(
        CarType::Suv,
        Car {
            manufacturer: "Subaru".to_string(),
            model: "Ascent".to_string(),
        },
    );

    // for (car_type, car) in cars {
    for (car_type, car) in cars.borrow() {
        println!("{:?} -> {}", car_type, car)
        // println!("{:?} -> {:?}", car_type, car) // debug
    }

    println!("{:?}", &cars)
}
