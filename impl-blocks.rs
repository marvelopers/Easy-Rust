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
  fn create(age: u8, animal_type: AnimalType) -> Self {
    Self { age, animal_type }
  }

  // fn change_to_cat(&mut self) {
  //     self.animal_type = AnimalType::Cat;
  //     println!("Changed to dog! Now I am: {:?}", self);
  // }

  // fn change_to_dog(&mut self) {
  //     self.animal_type = AnimalType::Dog;
  //     println!("Changed to cat! Now I am: {:?}", self);
  // }

  fn check_type(&self) {
    use AnimalType::*;

    match self.animal_type {
      Cat => pringln!("Animal type is cat"),
      Dog => pringln!("Animal type is dog"),
    }
  }

  fn print(&self) {
    println!("I am a: {:?}", self);
  }
}

fn main() {
  // let my_vec = vec![7, 8];
  // println!("The Vec length is: {}", my_vec.len()); //method

  // let my_cat = Animal::new_cat(1);
  // let my_dog = Animal::new_dog(2);
  // my_dog.print();
  // my_dog.change_to_cat();
  // my_dog.change_to_dog();

  use AnimalType::*;
  let my_cat = Animal.create(1, Cat);
  let my_dog = Animal.create(1, Dog);
}
