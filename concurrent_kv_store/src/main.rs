use std::collections::HashMap;
use std::sync::{Arc,Mutex};
use std::thread;
struct KVStore{
    map: Arc<Mutex<HashMap<String,String>>>,
}

impl KVStore{
    fn new() -> Self {
        KVStore {
            map : Arc::new(Mutex::new(HashMap::new())),
        }
    }
    fn insert(&self,  key: String, value:String){
        let mut map = self.map.lock().unwrap();
        map.insert(key,value);
    }
    fn get(&self, key: String) -> Option<String>{
        let map = self.map.lock().unwrap();
        map.get(&key).cloned()
    }
    fn delete(&self, key: String){
        let mut map = self.map.lock().unwrap();
        map.remove(&key);
    }
}

fn main() {
    let store = Arc::new(KVStore::new());

    let store1 = Arc::clone(&store);
    let handle1 = thread::spawn(move || {
        store1.insert("test".to_string(),"123".to_string());
        println!("Insert");
    });
    let store2 = Arc::clone(&store);
    let handle2 = thread::spawn(move || {
        if let Some(val) = store2.get("test".to_string()) {
            println!("{}",val);
        }
    });
    let store3 = Arc::clone(&store);
    let handle3 = thread::spawn(move || {
        store3.delete("test".to_string());
        println!("Deleted test");
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();

}