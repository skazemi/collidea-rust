use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
    marker::PhantomData,
};
use xxhash_rust::xxh3::Xxh3;

#[derive(Clone)]
pub struct CollideMap<K: Hash, V> {
    inner: HashMap<u64, V>,
    _marker: PhantomData<*const K>, // Using `*const T` indicates that we do not own a K
}

fn get_hash_xxh<K: Hash>(k: &K) -> u64 {
    let mut hasher = Xxh3::new();
    k.hash(&mut hasher);
    hasher.finish()
}

impl<K: Hash, V> CollideMap<K, V> {
    pub fn new() -> CollideMap<K, V> {
        CollideMap {
            inner: HashMap::new(),
            _marker: PhantomData,
        }
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn contains_key(&self, k: &K) -> bool {
        self.inner.contains_key(&get_hash_xxh(k))
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        self.inner.get(&get_hash_xxh(k))
    }

    pub fn get_mut(&mut self, k: &K) -> Option<&mut V> {
        self.inner.get_mut(&get_hash_xxh(k))
    }

    pub fn insert(&mut self, k: &K, v: V) -> Option<V> {
        self.inner.insert(get_hash_xxh(k), v)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn remove(&mut self, k: &K) -> Option<V> {
        self.inner.remove(&get_hash_xxh(k))
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.inner.values()
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> {
        self.inner.values_mut()
    }
}

#[derive(Clone)]
pub struct CollideSet<K: Hash> {
    inner: HashSet<u64>,
    _marker: PhantomData<*const K>, // Using `*const T` indicates that we do not own a K
}

impl<K: Hash> CollideSet<K> {
    pub fn new() -> CollideSet<K> {
        CollideSet {
            inner: HashSet::new(),
            _marker: PhantomData,
        }
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn contains(&self, k: &K) -> bool {
        self.inner.contains(&get_hash_xxh(k))
    }

    pub fn insert(&mut self, k: K) -> bool {
        self.inner.insert(get_hash_xxh(&k))
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn remove(&mut self, k: &K) -> bool {
        self.inner.remove(&get_hash_xxh(k))
    }

    pub fn union(&self, other: &CollideSet<K>) -> CollideSet<K> {
        let union_m = self.inner.union(&other.inner);
        let mut union: CollideSet<K> = CollideSet::new();
        union.inner = union_m.map(|k| k.clone()).collect();
        union
    }

    pub fn intersection(&self, other: &CollideSet<K>) -> CollideSet<K> {
        let intersection_m = self.inner.intersection(&other.inner);
        let mut intersection: CollideSet<K> = CollideSet::new();
        intersection.inner = intersection_m.map(|k| k.clone()).collect();
        intersection
    }
}
