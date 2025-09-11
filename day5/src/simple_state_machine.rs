use std::marker::PhantomData;

// --- Marker Types for States ---
// These don't hold data, they just act like labels for our states.
#[derive(Debug, Clone, Copy)] // So we can print and copy them easily
struct Sleeping;

#[derive(Debug, Clone, Copy)]
struct Awake;

// --- Our Cat Struct ---
// It's generic! The `State` tells us what condition the cat is in.
// PhantomData tells Rust "we care about this `State` type, even if we don't store data of that type".
#[derive(Debug)]
struct Cat<State> {
    name: String,
    _state: PhantomData<State>, // We use PhantomData because State is just a marker
}

// --- Transitions ---

// Implementation block ONLY for Cats that are Sleeping
impl Cat<Sleeping> {
    // This function takes a Cat<Sleeping>...
    fn wake_up(self) -> Cat<Awake> {
        // ...and returns a NEW Cat<Awake>!
        println!("{} stretches and yawns... now awake! ☀️", self.name);
        Cat {
            name: self.name,     // Keep the same name
            _state: PhantomData, // Mark it as Awake now
        }
    }
    // Notice: There's no `lull_to_sleep` or `eat` function here! Sleeping cats don't eat!

    fn new_sleeping_cat(name: String) -> Self {
        println!("A new cat, {}, appears, already asleep...", name);
        Cat {
            name,
            _state: PhantomData,
        }
    }
}

// Implementation block ONLY for Cats that are Awake
impl Cat<Awake> {
    // This function takes a Cat<Awake>...
    fn lull_to_sleep(self) -> Cat<Sleeping> {
        // ...and returns a NEW Cat<Sleeping>!
        println!("{} curls up and starts snoozing... 😴", self.name);
        Cat {
            name: self.name,     // Keep the same name
            _state: PhantomData, // Mark it as Sleeping now
        }
    }

    // *** NEW *** Let the awake cat eat!
    // It takes any type that can be turned 'Into' a String for the food.
    // It consumes the Cat<Awake> and returns it, still Awake.
    fn eat(self, food: impl Into<String>) -> Self {
        // Here we convert the input 'food' into a real String
        let food_item: String = food.into();
        println!(
            "{} happily munches on some {}! Yum! 🐟",
            self.name, food_item
        );
        // The cat ate, but is still awake, so we return the same cat (type).
        self
    }
    // Notice: No `wake_up` function here! Can't wake up an awake cat!
}

// --- How to create our first Cat ---
// impl Cat<Sleeping> {
//     // A function to create a new cat, starting in the Sleeping state.
//     fn new_sleeping_cat(name: String) -> Self {
//         println!("A new cat, {}, appears, already asleep...", name);
//         Cat {
//             name,
//             _state: PhantomData,
//         }
//     }
// }

pub fn main() {
    // Mittens starts asleep
    let mittens = Cat::new_sleeping_cat("Mittens".to_string());
    println!("*️⃣ Initial state: {:?}", mittens);

    // We can ONLY call wake_up because mittens is Cat<Sleeping>
    let mittens = mittens.wake_up();
    println!("➡️ After waking up: {:?}", mittens);

    // *** NEW *** Now that Mittens is Awake, she can eat!
    // We pass a &str, but `eat` turns it into a String using .into()
    let mittens = mittens.eat("delicious tuna");
    println!("➡️ After eating: {:?}", mittens); // Still Awake

    // We could even pass a String directly if we wanted!
    let mittens = mittens.eat("fancy salmon".to_string());
    println!("➡️ After eating again: {:?}", mittens); // Still Awake

    // Now we can ONLY call lull_to_sleep because mittens is Cat<Awake>
    let mittens = mittens.lull_to_sleep();
    println!("➡️ After falling asleep again: {:?}", mittens);

    // Try uncommenting this - it won't compile! Mittens is Sleeping!
    // let mittens = mittens.eat("a midnight snack");

    // Try uncommenting this - it won't compile! Mittens is Sleeping!
    // let mittens = mittens.lull_to_sleep();
}
