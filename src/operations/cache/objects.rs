use crate::structs::numerics::structs::SharedNumeric;
use lazy_static::lazy_static;
use std::collections::HashMap;
use tokio::sync::RwLock;

lazy_static! {
    static ref CACHED_OBJECTS: RwLock<HashMap<String, CachedObj>> = RwLock::new(HashMap::new());
}

#[derive(Clone, Debug)]
pub struct CachedObj {
    pub id: String,
    pub data: SharedNumeric,
}

impl CachedObj {
    pub fn new(id: String, data: SharedNumeric) -> Self {
        CachedObj { id, data }
    }
}

pub async fn insert_cache_object(obj: CachedObj) {
    let mut cache = CACHED_OBJECTS.write().await;
    cache.insert(obj.id.clone(), obj);
}

pub async fn get_cached_object(id: &str) -> Option<CachedObj> {
    let cache = CACHED_OBJECTS.read().await;
    let obj = cache.get(id).clone();
    obj.cloned()
}

pub async fn remove_cached_object(id: &str) {
    let mut cache = CACHED_OBJECTS.write().await;
    cache.remove(id);
}

pub async fn clear_cached_objects() {
    let mut cache = CACHED_OBJECTS.write().await;
    cache.clear();
}

pub async fn overwrite_data_in_cached_object(id: &str, new_data: SharedNumeric) {
    let mut cache = CACHED_OBJECTS.write().await;
    if let Some(cached_obj) = cache.get_mut(id) {
        cached_obj.data = new_data;
    }
}
