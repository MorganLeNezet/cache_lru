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
    prev: Option<Box<Node<K, V>>>,
    next: Option<Box<Node<K, V>>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PersistentCache<K: Eq + Hash + Clone, V: Clone> {
    map: HashMap<K, Box<Node<K, V>>>,
    head: Option<Box<Node<K, V>>>,
    tail: Option<Box<Node<K, V>>>,
    capacity: usize,
}

impl<K: Eq + Hash + Clone, V: Clone> PersistentCache<K, V> {
    fn remove_node(&mut self, node: &mut Box<Node<K, V>>) {
        // Retirer le nœud de la liste
        if let Some(mut prev) = node.prev.take() {
            prev.next = node.next.take();
        } else {
            self.head = node.next.take();
        }

        if let Some(mut next) = node.next.take() {
            next.prev = node.prev.take();
        } else {
            self.tail = node.prev.take();
        }
    }

    fn move_to_head(&mut self, node: &mut Box<Node<K, V>>) {
        // Si le nœud est déjà à la tête, rien à faire
        if self.head.as_ref() == Some(node) {
            return;
        }

        // Retirer le nœud de sa position actuelle
        self.remove_node(node);

        // Déplacer le nœud en tête de la liste
        node.prev = None;
        node.next = self.head.take();

        if let Some(ref mut next) = node.next {
            next.prev = Some(node.clone());
        }

        self.head = Some(node.clone());

        // Si la queue est vide, mettre à jour la queue
        if self.tail.is_none() {
            self.tail = Some(node.clone());
        }
    }

    fn pop_tail(&mut self) -> Option<Box<Node<K, V>>> {
        // Retirer le nœud de la queue (le moins récemment utilisé)
        if let Some(mut tail) = self.tail.take() {
            self.remove_node(&mut tail);
            Some(tail)
        } else {
            None
        }
    }
}

impl<K: Eq + Hash + Clone + Serialize + for<'de> Deserialize<'de>, V: Clone + Serialize + for<'de> Deserialize<'de>> Cache<K, V> for PersistentCache<K, V> {
    fn new(capacity: usize) -> Self {
        PersistentCache {
            map: HashMap::new(),
            head: None,
            tail: None,
            capacity,
        }
    }

    fn insert(&mut self, key: K, value: V) {
        // Si l'élément existe déjà, mettez-le à jour et déplacez-le en tête
        if let Some(node) = self.map.get_mut(&key) {
            node.value = value;
            self.move_to_head(node);
        } else {
            // Insérer un nouveau nœud
            let mut new_node = Box::new(Node {
                key: key.clone(),
                value,
                prev: None,
                next: self.head.take(),
            });

            // Relier le nœud à l'ancien nœud en tête, s'il existe
            if let Some(ref mut next) = new_node.next {
                next.prev = Some(new_node.clone());
            }

            self.head = Some(new_node.clone());

            if self.tail.is_none() {
                self.tail = Some(new_node.clone());
            }

            self.map.insert(key, new_node);

            // Eviction si nécessaire
            if self.map.len() > self.capacity {
                if let Some(tail) = self.pop_tail() {
                    self.map.remove(&tail.key);
                }
            }
        }
    }

    fn get(&mut self, key: &K) -> Option<&mut V> {
        if let Some(node) = self.map.get_mut(key) {
            Some(&mut node.value)
        } else {
            None
        }
    }

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

    fn load(&mut self, file_path: &str) -> io::Result<()> {
        let mut file = OpenOptions::new().read(true).open(file_path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        *self = serde_json::from_str(&data)?;
        Ok(())
    }

    fn move_to_head(&mut self, key: &K) {
        if let Some(node) = self.map.get_mut(key) {
            self.move_to_head(node);
        }
    }
}
