use std::cmp::PartialEq;
use std::fs;
use std::path::{Path, PathBuf};
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::key::{Key, RegObject};

pub struct Provider<T: serde::Serialize + serde::de::DeserializeOwned> {
    objects: Vec<RegObject<T>>
}



impl <T: Serialize + DeserializeOwned> Provider<T> {

    pub fn new() -> Self {
        Provider { objects: Vec::new() }
    }

    pub  const fn create()-> Self {
    
    Provider { objects: Vec::new() }
    }
    
    pub fn register_object(&mut self, key: crate::key::Key, data: T) -> usize {
        let object = RegObject::new(key.clone(), data);
        let index = self.objects.len();
        self.objects.push(object);
        println!("Registered new object with id: {:?}",key.to_json());
        index
    }

    pub fn register_new_item(&mut self, key: crate::key::Key, data: T)-> (usize,Key){

        (self.register_object(key.clone(),data),key.clone())

    }

    pub fn get_object_by_key(&self, key: &crate::key::Key) -> Option<&RegObject<T>> {
        let index = self.objects.iter().position(|obj| obj.key.id == key.id && obj.key.item_id == key.item_id);
        index.map(|i| &self.objects[i])
    }

    pub fn get_object_by_key_mut(&mut self, key: &crate::key::Key) -> Option<&mut RegObject<T>> {
        let index = self.objects.iter().position(|obj| {
            obj.key.id == key.id && obj.key.item_id == key.item_id
        })?;

        Some(&mut self.objects[index])
    }

    pub fn get_object_by_index(&self, index: usize) -> Option<&RegObject<T>> {
        self.objects.get(index)
    }

    pub fn get_object_by_index_mut(&mut self, index: usize) ->  Option<&mut RegObject<T>> {
        self.objects.get_mut(index)
    }

    pub fn remove_object_by_key(&mut self, key: &crate::key::Key) -> Option<RegObject<T>> {
        let index = self.objects.iter().position(|obj| obj.key.id == key.id && obj.key.item_id == key.item_id);
        index.map(|i| self.objects.remove(i))
    }

    pub fn remove_object_by_index(&mut self, index: usize) -> RegObject<T> {
        self.objects.remove(index)
    }


    pub fn get_objects(&self) -> &Vec<RegObject<T>> {
        &self.objects
    }

    pub fn count_objects(&self) -> usize {
        self.objects.len()
    }

    pub fn load_from_folder_json(&mut self, folder: &str) {
        use std::fs::{File, ReadDir};
        use std::io::Read;

        if !Path::new(folder).is_dir() {
            return;
        }

        for entry in std::fs::read_dir(folder).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_file() && entry.path().extension().unwrap_or_default() == "json" {
                let mut file = File::open(entry.path()).unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                let obj: RegObject<T> = serde_json::from_str(&contents).unwrap();
                self.register_object(obj.key.clone(), obj.data);
            }
        }
    }

    pub fn load_from_folder_yaml(&mut self, folder: &str) {
        use std::fs::{File, ReadDir};
        use std::io::Read;

        for entry in std::fs::read_dir(folder).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_file() && entry.path().extension().unwrap_or_default() == "yaml" {
                let mut file = File::open(entry.path()).unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                let obj: RegObject<T> = serde_yaml::from_str(&contents).unwrap();
                self.register_object(obj.key.clone(), obj.data);
            }
        }
    }

    pub fn write_to_folder_json(&self, folder: &str) -> std::io::Result<()> {
        // Create folder once
        fs::create_dir_all(folder)?;

        for obj in &self.objects {
            let file_name = format!("{}.json", obj.key.item_id);

            let file_path = Path::new(folder).join(file_name);

            let json_str = serde_json::to_string_pretty(obj)
                .expect("Failed to serialize object to JSON");

            fs::write(file_path, json_str)?;
        }

        Ok(())
    }

    pub fn write_to_folder_yaml(&self, folder: &str) -> std::io::Result<()> {
        // Create folder once
        fs::create_dir_all(folder)?;

        for obj in &self.objects {
            let file_name = format!("{}.yaml", obj.key.item_id);

            let file_path = Path::new(folder).join(file_name);

            let yaml_str = serde_yaml::to_string(obj)
                .expect("Failed to serialize object to YAML");

            fs::write(file_path, yaml_str)?;
        }

        Ok(())
    }

    pub fn get_or_register_object(
        &mut self,
        key: crate::key::Key,
        default: T,
    ) -> &RegObject<T> {

        // SAFETY: we only use raw pointer temporarily for lookup
        let ptr: *const Self = self;

        // Phase 1: check existence (no Rust borrows held across function)
        let index_opt = unsafe {
            (*ptr).objects
                .iter()
                .position(|obj| obj.key == key)
        };

        if let Some(i) = index_opt {
            return &self.objects[i];
        }

        // Phase 2: mutate safely (no active borrows now)
        let index = self.register_object(key.clone(), default);

        &self.objects[index]

    }

    pub fn item(key:Key, folder:String) -> Box<PathBuf> {
        Box::new(Path::new(&folder).join(format!("{}.json",key.item_id)))
    }

}


