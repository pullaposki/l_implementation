#[derive(Debug)]
struct Animal {
    age:u8,
    animal_type:AnimalType
}

#[derive(Debug)]
enum AnimalType {
    Dog,
    Cat,
    Bird
}

impl Animal {
    // Implementations often have new() method to create a new instance of the struct
    fn new(age:u8, animal_type:AnimalType) -> Self {
        Self {
            age,
            animal_type
        }
    }

    fn cnange_to_dog(&mut self) {
        println!("Changing animal type to Dog");
        self.animal_type = AnimalType::Dog;
    }

    fn cnange_to_cat(&mut self) {
        println!("Changing animal type to Cat");
        self.animal_type = AnimalType::Cat;
    }

    fn cnange_to_bird(&mut self) {
        println!("Changing animal type to Bird");
        self.animal_type = AnimalType::Bird;
    }

    fn check_type(&self) {
        match self.animal_type {
            AnimalType::Dog => println!("This is a Dog"),
            AnimalType::Cat => println!("This is a Cat"),
            AnimalType::Bird => println!("This is a Bird")
        }
    }

}



fn main() {
    
}
