#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type: AnimalType,
}

#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog,
}

impl Animal {
    fn new_cat(age) -> Self {
        Self {
            age,
            animal_type: AnimalType::Cat,
        }
    }
}

fn main() {
    let my_vec = vec![7, 8];
    println!("The Vec length is: {}", my_vec.len()); //method

    let my_cat = Animal::new_cat(1);
    println!("I made my cat {:?}", my_cat);
}
