use lru::LruCache;
use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};

#[derive(Serialize, Deserialize, Debug)]
struct PersistentCache<K, V> {
    cache: LruCache<K, V>,
}


impl<K, V> PersistentCache<K, V> {}

impl<K, V> PersistentCache<K, V> {}

impl<K: Eq + std::hash::Hash + Serialize + for<'de> Deserialize<'de>, V: Serialize + for<'de> Deserialize<'de>> PersistentCache<K, V> {
    // Crée une nouvelle instance de PersistentCache avec une capacité donnée et charge les données depuis un fichier
    fn new(capacity: usize, file_path: &str) -> Self {
        let cache = LruCache::new(capacity);
        let mut persistent_cache = PersistentCache { cache };
        persistent_cache.load(file_path).unwrap_or(());
        persistent_cache
    }

    // Insère une nouvelle paire clé-valeur dans le cache et persiste les données dans un fichier
    fn insert(&mut self, key: K, value: V, file_path: &str) {
        self.cache.put(key, value);
        self.persist(file_path).unwrap_or(());
    }

    // Récupère une valeur du cache en fonction de la clé
    fn get(&mut self, key: &K) -> Option<&V> {
        self.cache.get(key)
    }

    // Persiste les données du cache dans un fichier
    fn persist(&self, file_path: &str) -> io::Result<()> {
        let file = OpenOptions::new().write(true).create(true).truncate(true).open(file_path)?;
        let data = serde_json::to_string(&self)?;
        write!(file, "{}", data)?;
        Ok(())
    }

    // Charge les données du cache depuis un fichier
    fn load(&mut self, file_path: &str) -> io::Result<()> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let loaded_cache: PersistentCache<K, V> = serde_json::from_str(&contents)?;
        self.cache = loaded_cache.cache;
        Ok(())
    }
}

fn main() {
    // Crée une nouvelle instance de PersistentCache avec une capacité de 2 et charge les données depuis "cache.json"
    let mut cache = PersistentCache::new(2, "cache.json");
    // Insère des paires clé-valeur dans le cache et persiste les données
    cache.insert(1, "one", "cache.json");
    cache.insert(2, "two", "cache.json");
    // Affiche la valeur associée à la clé 1
    println!("{:?}", cache.get(&1)); 
    // Insère une nouvelle paire clé-valeur, ce qui peut provoquer l'éviction de la plus ancienne entrée
    cache.insert(3, "three", "cache.json");
    // Affiche la valeur associée à la clé 2, qui peut avoir été évincée
    println!("{:?}", cache.get(&2)); 
    // Affiche la valeur associée à la clé 3
    println!("{:?}", cache.get(&3)); 
}