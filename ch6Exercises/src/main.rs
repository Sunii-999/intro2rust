// 1. Define the struct 'Character' with fields: name (String), health (i32)
struct Character {
    name: String,
    health: i32,
}

impl Character {
    // 2. Complete this method to PRINT the name and health
    // It should NOT take ownership, only READ access
    fn inspect(&mut self) {
        println!("Stats: {} has {} HP", self.name, self.health);
    }

    // 3. Complete this method to MODIFY the health
    // It must allow changing values
    fn heal(&mut self, amount: i32) {
        self.health += amount;
        println!("{} healed for {} HP!", self.name, amount);
    }
}

// 4. Complete this function to TAKE OWNERSHIP
// The character should be moved into this function and dropped here
fn retire_character(character: Character) {
    println!("{} has retired from adventuring.", character.name);
} // character is dropped here

fn main() {
    // 5. Fix this variable declaration
    // We need to modify 'hero' later, so is 'let' enough?
    let mut hero = Character {
        name: "Sunii".to_string(),
        health: 50,
    };

    // 6. Call inspect()
    hero.inspect();

    // 7. Call heal() - heal the hero by 20 points
    hero.heal(20);

    // 8. Call inspect() again to see the change
    hero.inspect();

    // 9. Call retire_character()
    retire_character(hero);

    // 10. UNCOMMENT the line below. It should CAUSE AN ERROR if you did step 9 correctly.
    // hero.inspect(); 
}