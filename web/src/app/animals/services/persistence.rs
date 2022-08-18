use std::sync::{Arc, Mutex};

use internal::animals::models::animal::Animal;

lazy_static! {
    pub static ref PERSISTENCE_INSTANCE: Persistence = Persistence::new();
}

pub struct Persistence {
    animals: Arc<Mutex<Vec<Animal>>>,
}

impl Persistence {
    pub fn new() -> Persistence {
        Persistence {
            animals: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn create(&self, mut animal: Animal) -> i64 {
        let mut animals = self.animals.lock().unwrap();
        let id = if animals.is_empty() {
            let id = 1;
            id
        } else {
            let id = animals[animals.len() - 1].id + 1;
            id
        };
        animal.id = id;
        animals.push(animal);
        id
    }

    pub fn delete(&self, id: i64) -> () {
        let mut animals = self.animals.lock().unwrap();
        animals.retain(|x| x.id != id);
    }

    pub fn get(&self, id: i64) -> Option<Animal> {
        let animals = self.animals.lock().unwrap();
        for animal in animals.iter() {
            if animal.id == id {
                return Some(animal.clone());
            }
        }
        None
    }

    // pub fn batch_get(&self, ids: Vec<i64>) -> Result<Vec<Animal>, String> {
    //     todo!();
    // }

    pub fn get_all(&self) -> Vec<Animal> {
        let animals = self.animals.lock().unwrap();
        animals.clone()
    }

    pub fn update(&self, animal: Animal) -> () {
        let mut animals = self.animals.lock().unwrap();
        for i in 0..animals.len() {
            if animals[i].id == animal.id {
                animals[i] = animal;
                return;
            }
        }
    }
}
