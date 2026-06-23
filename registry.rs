use std::path::PathBuf;

pub struct DataDrivenProvider<T>{
    folder:Path,
    provider: Provider<T>
}

pub fn new<T>(folder: Path)->DataDrivenProvider<T>{
    DataDrivenProvider{ folder, provider: Provider::new() }
}

pub trait ProviderLifecycleJSON{
    fn begin(&self);

    fn end(&self);

    fn flush(&self);

    fn remove(&self,key:Key);

    fn remove_all(&self);
    fn reload(&self);
    fn save(&self);

}

impl ProviderLifecycleJSON for DataDrivenProvider<T> {
    fn begin(&self) {
        if self.folder.exists() && self.folder.is_dir() {
            self.provider.load_from_folder_json(self.folder.to_str().unwrap());
        }


    }

    fn end(&self) {
       self.provider.write_to_folder_json(self.folder.to_str().unwrap());
    }

    fn flush(&self) {
        self.provider.write_to_folder_json(self.folder.to_str().unwrap());
    }

    fn remove(&self, key: Key) {
        if self.provider.get_object_by_key(&key).is_some() {
            self.provider.remove_object_by_key(&key);

            if self.folder.exists() && self.folder.is_dir(){
                let item: PathBuf =  provider::Provider.item(key,self.folder.to_str().unwrap());

                if item.exists() && item.is_file(){
                    std::fs::remove_file(item).unwrap();
                }

            }


        }
    }

    fn remove_all(&self) {

        if self.folder.exists() && self.folder.is_dir(){

            for item in self.provider.get_objects().iter() {
                if let Some(item) = provider::Provider.item(item.key,self.folder.to_str().unwrap()){
                    std::fs::remove_file(item).unwrap();
                }
            }

        }

    }

    fn reload(&self) {
       remove_all(self);
        begin(self);
    }

    fn save(&self) {
        end(self);
    }
}