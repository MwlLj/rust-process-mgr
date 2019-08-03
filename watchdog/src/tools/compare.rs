use crate::config::{Process};

use std::collections::{HashMap, VecDeque};

pub fn processCompare<'a, F>(news: &'a VecDeque<Process>, olds: &'a VecDeque<Process>
    , f: F) -> (Vec<&'a Process>, Vec<&'a Process>, Vec<&'a Process>)
    where F: Fn(&Process, &Process) -> bool {
    // to new map
    let mut newMap = HashMap::new();
    for item in news {
        newMap.insert(item.name.to_string(), item);
    }
    println!("newMap: {:?}", &newMap);
    // to old map
    let mut oldMap = HashMap::new();
    for item in olds {
        oldMap.insert(item.name.to_string(), item);
    }
    println!("oldMap: {:?}", &oldMap);
    // compare
    let mut adds = Vec::new();
    let mut updates = Vec::new();
    let mut deletes = Vec::new();
    for (key, value) in newMap.iter() {
        if let Some(v) = oldMap.get(key) {
            // new exist, old exist -> update
            println!("new exist, old exist");
            if f(v, value) {
                updates.push(*v)
            }
            oldMap.remove(key);
        } else {
            println!("new exist, old not exist");
            // new exist, old not exist -> add
            adds.push(*value);
        }
    }
    for (_, value) in oldMap.iter() {
        deletes.push(*value);
    }
    (adds, updates, deletes)
}

pub fn processCompare2<'a, F>(news: &'a VecDeque<Process>, olds: &'a VecDeque<Process>
    , adds: &'a mut Vec<&'a Process>, updates: &'a mut Vec<&'a Process>, deletes: &'a mut Vec<&'a Process>
    , f: F)
    where F: Fn(&Process, &Process) -> bool {
    // to new map
    let mut newMap = HashMap::new();
    for item in news {
        newMap.insert(item.name.to_string(), item);
    }
    // to old map
    let mut oldMap = HashMap::new();
    for item in olds {
        oldMap.insert(item.name.to_string(), item);
    }
    // compare
    for (key, value) in newMap.iter() {
        if let Some(v) = oldMap.get(key) {
            // new exist, old exist -> update
            if f(v, value) {
                updates.push(*v)
            }
            oldMap.remove(key);
        } else {
            // new exist, old not exist -> add
            adds.push(*value);
        }
    }
    for (_, value) in oldMap.iter() {
        deletes.push(*value);
    }
}
