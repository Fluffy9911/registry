pub mod key;
pub mod provider;
pub mod events;

#[cfg(test)]
mod tests {
    use crate::provider::Provider;
    use super::*;

    #[test]
    fn it_works() {

        let mut provider: Provider<TestData> = provider::Provider::new();
provider.load_from_folder_json("TestJson");
   let object =      provider.get_or_register_object(key::Key::new("testid".to_string(),"test_item".to_string()),TestData::random());

println!("{}",object.to_yaml());


provider.write_to_folder_json("TestJson");
    }
}
use rand::{Rng, RngExt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestData {
    pub id: u64,
    pub username: String,
    pub level: u32,
    pub health: f32,
    pub alive: bool,
    pub inventory: Vec<String>,
}

impl TestData {
    /// Create a new instance manually
    pub fn new(
        id: u64,
        username: String,
        level: u32,
        health: f32,
        alive: bool,
        inventory: Vec<String>,
    ) -> Self {
        Self {
            id,
            username,
            level,
            health,
            alive,
            inventory,
        }
    }

    /// Generate random test data
    pub fn random() -> Self {
        let mut rng = rand::rng();

        let usernames = [
            "Alpha",
            "Bravo",
            "Charlie",
            "Delta",
            "Echo",
            "Foxtrot",
            "Ghost",
            "Nova",
        ];

        let items = [
            "Sword",
            "Shield",
            "Potion",
            "Bow",
            "Helmet",
            "Apple",
            "Gem",
            "Key",
        ];

        let inventory_size = rng.random_range(1..=5);

        let inventory = (0..inventory_size)
            .map(|_| {
                let index = rng.random_range(0..items.len());
                items[index].to_string()
            })
            .collect();

        Self {
            id: rng.next_u64(),
            username: format!(
                "{}{}",
                usernames[rng.random_range(0..usernames.len())],
                rng.random_range(1000..9999)
            ),
            level: rng.random_range(1..=100),
            health: rng.random_range(0.0..=100.0),
            alive: rng.random_bool(0.5),
            inventory,
        }
    }


}

