use project::cache::PersistentCache;
use project::traits::Cache;

fn main() {
    // Créer une instance de PersistentCache avec une capacité de 4
    let mut cache = PersistentCache::new(4);

    // Insérer des paires clé-valeur dans le cache
    cache.insert("key1".to_string(), "value1".to_string());
    cache.insert("key2".to_string(), "value2".to_string());
    cache.insert("key3".to_string(), "value3".to_string());
    cache.insert("key4".to_string(), "value4".to_string());

    // Récupérer et afficher les valeurs associées aux clés
    println!("key1: {:?}", cache.get(&"key1".to_string())); // Some("value1")
    println!("key2: {:?}", cache.get(&"key2".to_string())); // Some("value2")
    println!("key3: {:?}", cache.get(&"key3".to_string())); // Some("value3")
    println!("key4: {:?}", cache.get(&"key4".to_string())); // Some("value4")

    // Insérer une nouvelle paire clé-valeur, ce qui devrait évincer "key1" car la capacité est de 4
    cache.insert("key5".to_string(), "value5".to_string());

    // Récupérer et afficher les valeurs associées aux clés après l'éviction
    println!("key1: {:?}", cache.get(&"key1".to_string())); // None
    println!("key2: {:?}", cache.get(&"key2".to_string())); // Some("value2")
    println!("key3: {:?}", cache.get(&"key3".to_string())); // Some("value3")
    println!("key4: {:?}", cache.get(&"key4".to_string())); // Some("value4")
    println!("key5: {:?}", cache.get(&"key5".to_string())); // Some("value5")

    // Persister l'état actuel du cache dans un fichier
    cache.persist("cache.json").unwrap();

    // Créer une nouvelle instance de PersistentCache et charger l'état à partir du fichier
    let mut loaded_cache = PersistentCache::<String, String>::new(4);
    loaded_cache.load("cache.json").unwrap();

    // Récupérer et afficher les valeurs associées aux clés après le chargement
    println!("Loaded key2: {:?}", loaded_cache.get(&"key2".to_string())); // Some("value2")
    println!("Loaded key3: {:?}", loaded_cache.get(&"key3".to_string())); // Some("value3")
    println!("Loaded key4: {:?}", loaded_cache.get(&"key4".to_string())); // Some("value4")
    println!("Loaded key5: {:?}", loaded_cache.get(&"key5".to_string())); // Some("value5")
}