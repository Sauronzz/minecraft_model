use std::{collections::HashMap, hash::Hash};

pub trait ModelMerge {
    fn merge(&mut self, other: &Self);
}

impl ModelMerge for String {
    fn merge(&mut self, other: &Self) {
        
    }
}

impl ModelMerge for bool {
    fn merge(&mut self, other: &Self) {
        
    }
}

impl ModelMerge for f32 {
    fn merge(&mut self, other: &Self) {
        
    }
}

impl<T> ModelMerge for Vec<T>
    where T: ModelMerge + Clone
{
    fn merge(&mut self, other: &Self) {
        
    }
}

impl<K, V> ModelMerge for HashMap<K, V>
where 
    K: Eq + Hash + Clone,
    V: Clone
{
    fn merge(&mut self, other: &Self) {
        for (key, value) in other.iter() {
            if self.get(key).is_none() {
                self.insert(key.clone(), value.clone());
            }
        }
    }
}