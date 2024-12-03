use project::cache::PersistentCache;
use project::traits::Cache;

#[test]
// Teste l'insertion et la récupération d'une clé-valeur.
fn test_insert_and_get() {
    let mut cache = PersistentCache::new(2);
    cache.insert("key1".to_string(), "value1".to_string());
    assert_eq!(
        cache.get(&"key1".to_string()),
        Some(&mut "value1".to_string())
    );
}

#[test]
// Teste la persistance du cache dans un fichier et son rechargement.
fn test_persist_and_load() {
    let mut cache = PersistentCache::new(2);
    cache.insert("key1".to_string(), "value1".to_string());
    cache.persist("test_cache.json").unwrap();

    let mut loaded_cache = PersistentCache::new(2);
    loaded_cache.load("test_cache.json").unwrap();
    assert_eq!(
        loaded_cache.get(&"key1".to_string()),
        Some(&mut "value1".to_string())
    );
}

#[test]
// Teste l'expulsion de l'élément le moins récemment utilisé (LRU) lorsque la capacité est dépassée.
fn test_eviction_lru() {
    let mut cache = PersistentCache::new(2);
    cache.insert("key1".to_string(), "value1".to_string());
    cache.insert("key2".to_string(), "value2".to_string());
    cache.insert("key3".to_string(), "value3".to_string()); // Cela devrait expulser "key1".
    assert_eq!(cache.get(&"key1".to_string()), None);
    assert_eq!(
        cache.get(&"key2".to_string()),
        Some(&mut "value2".to_string())
    );
    assert_eq!(
        cache.get(&"key3".to_string()),
        Some(&mut "value3".to_string())
    );
}

#[test]
// Teste la mise à jour de la valeur associée à une clé existante.
fn test_update_value() {
    let mut cache = PersistentCache::new(2);
    cache.insert("key1".to_string(), "value1".to_string());
    cache.insert("key1".to_string(), "value1_updated".to_string());
    assert_eq!(
        cache.get(&"key1".to_string()),
        Some(&mut "value1_updated".to_string())
    );
}

#[test]
// Teste le respect de la capacité maximale et l'expulsion en conséquence.
fn test_capacity() {
    let mut cache = PersistentCache::new(2);
    cache.insert("key1".to_string(), "value1".to_string());
    cache.insert("key2".to_string(), "value2".to_string());
    cache.insert("key3".to_string(), "value3".to_string()); // Cela devrait expulser "key1".
    cache.insert("key4".to_string(), "value4".to_string()); // Cela devrait expulser "key2".
    assert_eq!(cache.get(&"key1".to_string()), None);
    assert_eq!(cache.get(&"key2".to_string()), None);
    // Vérifie que les autres clés sont encore présentes.
    assert_eq!(
        cache.get(&"key3".to_string()),
        Some(&mut "value3".to_string())
    );
    assert_eq!(
        cache.get(&"key4".to_string()),
        Some(&mut "value4".to_string())
    );
}
