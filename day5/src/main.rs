mod simple_state_machine;

#[tokio::main]
async fn main() {
    println!("-----trait_impl-----");
    trait_impl();
    println!("-----async_with_tokio-----");
    async_with_tokio().await;
    println!("-----async_with_async_trait-----");
    async_with_async_trait().await;
    println!("-----simple_state_machine-----");
    simple_state_machine::main();
}

// Just boring struct.
#[derive(Debug, Clone)]
struct Animal {}
struct Human {}

// New skill. Wanna to say something?
trait Sayable {
    // This nearly like interface.
    fn say(&self) -> String; // We use String instead of &str here for no reason.
}

// Implement `Sayable` skill for `Animal`.
impl Sayable for Animal {
    // All animal will say meow for now. 😆
    fn say(&self) -> String {
        "meow!".to_owned() // convert &str to String.
    }
}

// Implement `Sayable` skill for `Human`.
impl Sayable for Human {
    // All human kind say hi! 🤘
    fn say(&self) -> String {
        "hi!".to_owned() // convert &str to String.
    }
}

fn trait_impl() {
    let animal = Animal {};

    // So we can call like this.
    println!("{:?}", animal.say());

    // Or this.
    println!("{:?}", Animal::say(&animal));

    // Now human turn (with shorthand).
    println!("{:?}", Human::say(&Human {}));
}

use std::thread::sleep;
use std::time::{Duration, SystemTime};

async fn sleep_2secs() {
    // Will sleep for 2 seconds.
    sleep(Duration::new(2, 0));
}

async fn async_with_tokio() {
    let now = SystemTime::now();
    sleep_2secs().await;

    let now_sec = now.elapsed().ok().unwrap().as_secs();
    assert_eq!(now_sec, 2);
    println!("We have been asleep for {} seconds.", now_sec);
}

use async_trait::async_trait;

// 👇 We need this.
#[async_trait]
trait AnimalTrait {
    // 👇 Because of this async.
    async fn sleep(&self);
}

struct Cat;

// 👇 Also here.
#[async_trait]
impl AnimalTrait for Cat {
    async fn sleep(&self) {
        // Will sleep for 2 seconds.
        sleep(Duration::new(2, 0));
    }
}

// This `async fn main` need `tokio::main`.
async fn async_with_async_trait() {
    // Wait for sleepy cat for 2 sec.
    let now = SystemTime::now();
    Cat {}.sleep().await;

    // Ensure it's 2 sec.
    let now_sec = now.elapsed().ok().unwrap().as_secs();
    assert_eq!(now_sec, 2);
    println!("Cat has been asleep for {} seconds.", now_sec);
}
