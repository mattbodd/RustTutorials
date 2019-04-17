mod plant {
    pub struct Vegetable {
        pub name: String,
        id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
             Vegetable { 
                name: String::from(name),
                id: 1,
            }
        }

        pub fn get_info(&self) {
            println!("This is a(n) {} with id= {}"
                     , &self.name
                     , &self.id);
        }
    }
}

mod menu {
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub fn print_type(app : &Appetizer) -> String {
        match app {
            Appetizer::Soup => String::from("Soup"),
            Appetizer::Salad => String::from("Salad"),
        }
    }
}

fn main() {
    let mut v = plant::Vegetable::new("squash");

    v.name = String::from("butternut squash");
    println!("{} are delicious", v.name);

    v.get_info();

    let order1 = menu::Appetizer::Soup;
    let order2 = menu::Appetizer::Salad;
    println!("Type is: {}", menu::print_type(&order1));
    println!("Type is: {}", menu::print_type(&order2));
}
