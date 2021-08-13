trait Pet {
    fn is_happy(&self) -> bool;
}

#[derive(Debug)]
pub struct Dog {
    tail_wagging: bool,
}

impl Pet for Dog {
    fn is_happy(&self) -> bool {
        self.tail_wagging
    }
}

#[derive(Debug)]
pub struct Cat {
    sit_and_stare: bool,
}

impl Pet for Cat {
    fn is_happy(&self) -> bool {
        self.sit_and_stare
    }
}

enum AnimalType {}

pub enum AnimalTrait {
    TailWagging { happy: bool },
    SitAndStare { happy: bool },
}

pub struct PetManager {
    dogs: Vec<Dog>, // private
    cats: Vec<Cat>, // private
}

impl PetManager {
    pub fn new_dog(animal_trait: AnimalTrait) -> Option<Dog> {
        match animal_trait {
            AnimalTrait::TailWagging { happy } => Option::from(Dog {
                tail_wagging: happy,
            }),
            _ => Option::None,
        }
    }

    pub fn new_cat(animal_trait: AnimalTrait) -> Option<Cat> {
        match animal_trait {
            AnimalTrait::SitAndStare { happy } => Option::from(Cat {
                sit_and_stare: happy,
            }),
            _ => Option::None,
        }
    }
}

trait Manager<S> {
    fn add(&mut self, pet: S);
}

impl Manager<Dog> for PetManager {
    fn add(&mut self, pet: Dog) {
        self.dogs.push(pet);
    }
}

impl Manager<Cat> for PetManager {
    fn add(&mut self, pet: Cat) {
        self.cats.push(pet);
    }
}

pub fn pets() {
    // dogs
    let dog1 = PetManager::new_dog(AnimalTrait::TailWagging { happy: true });
    let dog2 = PetManager::new_dog(AnimalTrait::TailWagging { happy: false });

    // cats
    let cat1 = PetManager::new_cat(AnimalTrait::SitAndStare { happy: true });
    let cat2 = PetManager::new_cat(AnimalTrait::SitAndStare { happy: false });

    let mut pet_manager = PetManager {
        dogs: vec![dog1.unwrap()],
        cats: vec![cat1.unwrap()],
    };

    pet_manager.add(dog2.unwrap());
    pet_manager.add(cat2.unwrap());

    println!("dogs: {:?}", pet_manager.dogs);
    println!("cats: {:?}", pet_manager.cats);
}
