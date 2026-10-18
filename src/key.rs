use std::env::SplitPaths;
use std::fmt::format;
use std::ops::{Add, Shl};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;



#[derive(Serialize, Deserialize, Clone, Hash)]
pub struct Key {
    pub id: String,
    pub item_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RegObject<T>
{
    pub key: Key,
    pub data: T,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyStr(pub String);

pub fn key_json(id:String,item_id:String)->String{
    let k = Key{id,item_id };
    serde_json::to_string(&k).unwrap()
}
pub fn key_yaml(id:String,item_id:String)->String{
    let k = Key{id,item_id };
    serde_yaml::to_string(&k).unwrap()
}

pub fn to_json<T: Serialize>(obj:&T) -> String{
    serde_json::to_string(&obj).unwrap()
}


impl Key {

    pub fn new(id: String, item_id: String) -> Self {
        Self { id, item_id }
    }

    pub const fn new_K(id: String, item_id: String) -> Self {
        Self { id, item_id }
    }

    pub  fn new_key(id:  impl  Into<String>, item_id: impl Into<String>) -> Key {
        Self { id: id.into(), item_id: item_id.into() }
    }



    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn from_yaml(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn to_yaml(&self) -> String {
        serde_yaml::to_string(self).unwrap()
    }

    pub fn as_path(&self)->PathBuf{
        let p = Path::new(&self.id);
        p.join(format!("{}.key",self.item_id))
    }

    pub fn as_json_path(&self) -> PathBuf {
        self.as_path().with_extension("json")
    }

    pub fn as_yaml_path(&self)->PathBuf{
        self.as_path().with_extension("yaml")
    }

pub fn join_path(&self,add:String)->String{
    let p = Path::new(&self.id);
    let add_path = Path::new(add.as_str());
    p.join(add).as_path().to_str().unwrap().to_string()
}

    pub fn to_key_string(&self)->String{
        
        format!("{}:{}", self.id, self.item_id)
        
        
    }
    
    pub fn from_key_string(key_string: &str)->Result<Key,String>{
        
        let parts: Vec<&str> = key_string.split(':').collect();
        
        if parts.len() != 2 {
            return Err(format!("Invalid key format: {}", key_string));
        }
        Ok(Key{id:parts[0].to_string(),item_id:parts[1].to_string()})
        
    }
}

impl Add for Key {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            id: format!("{}/{}", self.id, other.id),
            item_id: format!("{}", self.item_id),
        }
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.item_id == other.item_id
    }
}
impl <T: Serialize + DeserializeOwned> RegObject<T> {


    pub fn new(key: Key, data: T) -> Self {
        Self { key, data }
    }

    pub fn to_json(&self) -> String{
        serde_json::to_string(self).unwrap()
    }

    pub fn from_json(json: &str)-> Result<Self,serde_json::Error>{
        serde_json::from_str(json)
    }

    pub fn to_yaml(&self)->String{
        serde_yaml::to_string(self).unwrap()
    }

    pub fn from_yaml(yaml: &str) -> Result<Self,serde_yaml::Error>{
        serde_yaml::from_str(yaml)
    }

    pub fn get_key(&self)->&Key{
        &self.key
    }

    pub fn get_data(&self)->&T{
        &self.data
    }

    pub fn id(&self) -> &String {
        &self.key.id
    }

}



impl Eq for Key{

}

