enum Fruit {
    Apple(String),
    Banana(String),
    Tomato(String),
}

struct Inventory {
    fruit: Vec<Fruit>,
}

impl Inventory {
    fn available_fruits(&self) {
        println!("Available fruits in store:");
        for (i, f) in self.fruit.iter().enumerate() {
            match f {
                Fruit::Apple(_) => println!("{}. Apple", i + 1),
                Fruit::Banana(_) => println!("{}. Banana", i + 1),
                Fruit::Tomato(_) => println!("{}. Tomato", i + 1),
            }
        }
        println!();
    }

    fn tell_me_joke(&self, fruit: &Fruit) {
        match fruit {
            Fruit::Apple(_) => println!("🍎 Why did the apple stop in the middle of the road? Because it ran out of juice!"),
            Fruit::Banana(_) => println!("🍌 What do you call two bananas? A pair of slippers!"),
            Fruit::Tomato(_) => println!("🍅 Why did the tomato blush? Because it saw the salad dressing!"),
        }
    }
}

fn main() {
    let a = "An apple a day keeps the doctor away.".to_string();
    let b = "A banana boosts energy in a peel!".to_string();
    let t = "A tomato a day keeps the sunburn away.".to_string();

    let fruits = vec![
        Fruit::Banana(b),
        Fruit::Apple(a),
        Fruit::Tomato(t),
    ];

    let grocery_store = Inventory { fruit: fruits };

    grocery_store.available_fruits();

    println!("Telling fruit jokes:\n");
    for f in &grocery_store.fruit {
        grocery_store.tell_me_joke(f);
    }
}
