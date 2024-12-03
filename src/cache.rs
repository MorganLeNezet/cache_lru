use crate::traits::Cache;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::hash::Hash;
use std::io::{self, Read, Write};

#[derive(Serialize, Deserialize, Clone)]
struct Node<K, V> {
    key: K,
    value: V,
    prev: Option<K>,
    next: Option<K>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PersistentCache<K: Eq + Hash + Clone, V: Clone> {
    map: HashMap<K, Box<Node<K, V>>>,
    head: Option<K>,
    tail: Option<K>,
    capacity: usize,
}

impl<K: Eq + Hash + Clone, V: Clone> PersistentCache<K, V> {
    // Supprime un nœud du cache en mettant à jour ses voisins.
    // Si le nœud est en tête ou en queue, met également à jour les pointeurs `head` ou `tail`.
    fn remove_node(&mut self, key: &K) {
        if let Some(node) = self.map.get(key) {
            let prev_key = node.prev.clone();
            let next_key = node.next.clone();

            if let Some(prev_key) = &prev_key {
                if let Some(prev_node) = self.map.get_mut(prev_key) {
                    prev_node.next = next_key.clone();
                }
            } else {
                self.head = next_key.clone();
            }

            if let Some(next_key) = &next_key {
                if let Some(next_node) = self.map.get_mut(next_key) {
                    next_node.prev = prev_key.clone();
                }
            } else {
                self.tail = prev_key.clone();
            }
        }
    }

    // Déplace un nœud spécifié en tête du cache, indiquant qu'il est le plus récemment utilisé.
    fn move_to_head(&mut self, key: K) {
        self.remove_node(&key);

        let mut node = self.map.remove(&key).expect("Node should exist");
        node.prev = None;
        node.next = self.head.clone();

        if let Some(head_key) = &self.head {
            if let Some(head_node) = self.map.get_mut(head_key) {
                head_node.prev = Some(key.clone());
            }
        }

        self.head = Some(key.clone());

        if self.tail.is_none() {
            self.tail = Some(key.clone());
        }

        self.map.insert(key, node);
    }

    // Supprime le nœud le moins récemment utilisé (situé à la queue).
    fn pop_tail(&mut self) {
        if let Some(tail_key) = self.tail.clone() {
            self.remove_node(&tail_key);
            self.map.remove(&tail_key);
        }
    }
}

impl<
        K: Eq + Hash + Clone + Serialize + for<'de> Deserialize<'de>,
        V: Clone + Serialize + for<'de> Deserialize<'de>,
    > Cache<K, V> for PersistentCache<K, V>
{
    // Initialise un nouveau cache LRU avec une capacité maximale spécifiée.
    fn new(capacity: usize) -> Self {
        PersistentCache {
            map: HashMap::new(),
            head: None,
            tail: None,
            capacity,
        }
    }

    // Insère un nouvel élément dans le cache.
    // Si la clé existe déjà, met à jour sa valeur et le déplace en tête.
    // Si la capacité est dépassée, évacue l'élément le moins récemment utilisé.
    fn insert(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            self.move_to_head(key.clone());
            if let Some(node) = self.map.get_mut(&key) {
                node.value = value;
            }
        } else {
            let new_node = Box::new(Node {
                key: key.clone(),
                value,
                prev: None,
                next: self.head.clone(),
            });

            if let Some(head_key) = &self.head {
                if let Some(head_node) = self.map.get_mut(head_key) {
                    head_node.prev = Some(key.clone());
                }
            }

            self.head = Some(key.clone());

            if self.tail.is_none() {
                self.tail = Some(key.clone());
            }

            self.map.insert(key.clone(), new_node);

            if self.map.len() > self.capacity {
                self.pop_tail();
            }
        }
    }

    // Récupère une valeur associée à une clé et la marque comme récemment utilisée.
    fn get(&mut self, key: &K) -> Option<&mut V> {
        if self.map.contains_key(key) {
            self.move_to_head(key.clone());
            self.map.get_mut(key).map(|node| &mut node.value)
        } else {
            None
        }
    }

    // Sauvegarde l'état actuel du cache dans un fichier sous format JSON.
    fn persist(&self, file_path: &str) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)?;
        let data = serde_json::to_string(&self)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    // Charge un état du cache depuis un fichier sauvegardé en format JSON.
    fn load(&mut self, file_path: &str) -> io::Result<()> {
        let mut file = OpenOptions::new().read(true).open(file_path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        *self = serde_json::from_str(&data)?;
        Ok(())
    }

    // Déplace un élément spécifié en tête du cache.
    fn move_to_head(&mut self, key: &K) {
        self.move_to_head(key.clone());
    }
}
