fn main() {
    println!("-----animal_type-----");
    animal_type();
    println!("-----japanese_food-----");
    japanese_food();
}

#[derive(Debug)]
enum AnimalType {
    Cat,
    Duck,
}

// How to return string or &str from enum.
impl AnimalType {
    fn as_str(&self) -> &str {
        match self {
            AnimalType::Cat => "🐈",
            AnimalType::Duck => "🐥",
        }
    }

    // How to use type as a parameters, but hey!👇 what's this? 😳
    fn say1(animal_type: AnimalType) -> &'static str {
        // To survive from fn {}, we need 👆 'static to let is has program's lifetime.
        match animal_type {
            AnimalType::Cat => "meaowww",
            AnimalType::Duck => "quackkk",
        }
    }

    // But why this didn't need to add static here? 👇 😳
    fn say2(&self, animal_type: AnimalType) -> &str {
        match animal_type {
            AnimalType::Cat => "meaowww",
            AnimalType::Duck => "quackkk",
        }
    }

    // That's because `elided lifetime rules` cover that for you already!
    // Then👇 you👇 don't need to write this loooooong👇
    fn say3<'a>(&'a self, animal_type: AnimalType) -> &'a str {
        match animal_type {
            AnimalType::Cat => "meaowww",
            AnimalType::Duck => "quackkk",
        }
    }

    // Remember this?
    fn static_say1(animal_type: &str) -> &str {
        match animal_type {
            "cat" => "meaowww",
            "duck" => "quackkk",
            _ => "wat!",
        }
    }

    // Actually the longer one look like this
    fn static_say2<'a>(animal_type: &'a str) -> &'a str {
        match animal_type {
            "cat" => "meaowww",
            "duck" => "quackkk",
            _ => "wat!",
        }
    }
}

fn animal_type() {
    println!(
        "{0:?} aka {1:?} say {2:?}, {3:?}",
        AnimalType::Cat,
        AnimalType::Cat.as_str(),
        AnimalType::say1(AnimalType::Cat),
        AnimalType::say2(&AnimalType::Cat, AnimalType::Cat),
    );

    println!(
        "{0:?} aka {1:?} say {2:?}, {3:?}",
        AnimalType::Duck,
        AnimalType::Duck.as_str(),
        AnimalType::say1(AnimalType::Duck),
        AnimalType::say2(&AnimalType::Duck, AnimalType::Duck),
    );

    AnimalType::static_say1("cat");
    AnimalType::static_say2("cat");
    println!(
        "AnimalType::say3: {:#?}",
        AnimalType::say3(&AnimalType::Cat, AnimalType::Cat)
    );
}

// Let's create a struct for those extra-special sushi rolls! 🍣
#[derive(Debug)]
struct SushiRollDetails {
    name: String,
    has_wasabi: bool,
    ingredients: Vec<String>,
}

// Our delightful Japanese food enum! 🍜🍣🍙
#[derive(Debug)]
enum JapaneseFood {
    // Simple and tasty ramen! 🍜
    Ramen,
    // A single piece of nigiri! 🍚
    #[allow(dead_code)]
    Nigiri {
        topping: String,
    },
    // A flavorful onigiri! 🍙 Holding the filling!
    Onigiri(String),
    // Our fancy sushi roll variant, holding the SushiRollDetails struct! ✨
    SpecialRoll(SushiRollDetails),
    // Delicious and crispy tempura! 🍤 Holding the type!
    #[allow(dead_code)]
    Tempura(String),
}

fn japanese_food() {
    // A comforting bowl of ramen! 😊
    let dinner = JapaneseFood::Ramen;
    // Salmon nigiri! 😋
    let my_sushi = JapaneseFood::Nigiri {
        topping: "Salmon".to_string(),
    };
    // A yummy tuna onigiri! 🍙
    let my_riceball = JapaneseFood::Onigiri("Tuna Mayo".to_string());
    // Our amazing special sushi roll! 🤩
    let dragon_roll = JapaneseFood::SpecialRoll(SushiRollDetails {
        name: "Dragon Roll".to_string(),
        has_wasabi: true,
        ingredients: vec![
            "Eel".to_string(),
            "Avocado".to_string(),
            "Cucumber".to_string(),
        ],
    });
    // Some crispy shrimp tempura! 🍤
    let fried_goodness = JapaneseFood::Tempura("Shrimp".to_string());

    // Let's see what's on the menu! 👀
    println!("Dinner: {:?}", dinner);
    println!("Sushi: {:?}", my_sushi);
    println!("Onigiri: {:?}", my_riceball);
    println!("Special Roll: {:?}", dragon_roll);
    println!("Tempura: {:?}", fried_goodness);

    // What's in our special Dragon Roll? 🐉 Let's check the details!
    match dragon_roll {
        JapaneseFood::SpecialRoll(details) => {
            println!(
                "The {} contains: {} and has {} wasabi!",
                details.name,
                details.ingredients.join(", "),
                if details.has_wasabi {
                    "wasabi"
                } else {
                    "no wasabi"
                }
            );
        }
        _ => {} // Not a special roll!
    }

    // What kind of onigiri did we get? 🤔
    match my_riceball {
        JapaneseFood::Onigiri(filling) => {
            println!("Enjoying an onigiri with {} filling! 🍙", filling);
        }
        _ => {} // Not an onigiri!
    }
}
