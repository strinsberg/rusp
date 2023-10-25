use crate::data::{DisplayRep, Error, ExternalRep, Val};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Map {
    mutable: bool,
    entries: HashMap<Val, Val>,
}

impl Map {
    // Constructors //

    pub fn new() -> Map {
        Map {
            mutable: true,
            entries: HashMap::new(),
        }
    }

    pub fn map(entries: &[(Val, Val)]) -> Result<Map, Error> {
        let mut map = Map {
            mutable: true,
            entries: HashMap::with_capacity(entries.len()),
        };
        for (k, v) in entries.iter() {
            match map.assoc(k.clone(), v.clone()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        Ok(map)
    }

    pub fn dict(entries: &[(Val, Val)]) -> Result<Map, Error> {
        let mut map = Map::map(entries)?;
        map.mutable = false;
        Ok(map)
    }

    pub fn map_from_vec(entries: &[Val]) -> Result<Map, Error> {
        let mut map = Map {
            mutable: true,
            entries: HashMap::with_capacity(entries.len()),
        };
        map.add_pairs_from_vec(entries)?;
        Ok(map)
    }

    pub fn add_pairs_from_vec(&mut self, vals: &[Val]) -> Result<(), Error> {
        let mut iter = vals.iter();
        loop {
            let key = iter.next();
            let val = iter.next();
            match (key, val) {
                (Some(k), Some(v)) => match self.assoc(k.clone(), v.clone()) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                },
                (Some(_), None) => return Err(Error::MapArgsNotEven(Val::from(vals.to_vec()))),
                _ => break,
            }
        }
        Ok(())
    }

    pub fn dict_from_vec(entries: &[Val]) -> Result<Map, Error> {
        let mut map = Map::map_from_vec(entries)?;
        map.mutable = false;
        Ok(map)
    }

    pub fn copy_to_map(map: Map) -> Map {
        let entries: Vec<(Val, Val)> = map.entries().map(|(k, v)| (k.clone(), v.clone())).collect();
        Map::map(&entries).unwrap()
    }

    pub fn copy_to_dict(map: Map) -> Map {
        let mut map = Map::copy_to_map(map);
        map.mutable = false;
        map
    }

    // Access //

    pub fn freeze(&mut self) {
        self.mutable = false
    }

    pub fn assoc(&mut self, key: Val, val: Val) -> Result<(), Error> {
        if !self.mutable {
            Err(Error::Immutable)
        } else if key.is_hashable() {
            self.entries.insert(key, val);
            Ok(())
        } else {
            Err(Error::NotHashable(key))
        }
    }

    pub fn dissoc(&mut self, key: Val) -> Result<(), Error> {
        if !self.mutable {
            Err(Error::Immutable)
        } else if key.is_hashable() {
            self.entries.remove(&key);
            Ok(())
        } else {
            Err(Error::NotHashable(key))
        }
    }

    pub fn get(&self, key: Val) -> Option<Val> {
        if key.is_hashable() {
            match self.entries.get(&key) {
                Some(val) => Some(val.clone()),
                None => None,
            }
        } else {
            None
        }
    }

    pub fn clear(&mut self) -> Result<(), Error> {
        if !self.mutable {
            Err(Error::Immutable)
        } else {
            self.entries.clear();
            Ok(())
        }
    }

    pub fn keys(&self) -> Vec<&Val> {
        self.entries.keys().collect()
    }

    pub fn values(&self) -> Vec<&Val> {
        self.entries.values().collect()
    }

    pub fn entries(&self) -> std::collections::hash_map::Iter<'_, Val, Val> {
        self.entries.iter()
    }

    // Information

    pub fn is_dict(&self) -> bool {
        !self.mutable
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

// Traits /////////////////////////////////////////////////////////////////////

// Equality //

impl PartialEq for Map {
    fn eq(&self, other: &Map) -> bool {
        self.entries == other.entries
    }

    fn ne(&self, other: &Map) -> bool {
        !self.eq(other)
    }
}

impl Eq for Map {}

// Representaitons //

impl DisplayRep for Map {
    fn to_display(&self) -> String {
        let vals = self
            .entries()
            .map(|(k, v)| format!("{} {}", k.to_display(), v.to_display()))
            .collect::<Vec<String>>()
            .join(" ");
        if self.mutable {
            format!("{{{vals}}}")
        } else {
            format!("#{{{vals}}}")
        }
    }
}

impl ExternalRep for Map {
    fn to_external(&self) -> String {
        let vals = self
            .entries()
            .map(|(k, v)| format!("{} {}", k.to_external(), v.to_external()))
            .collect::<Vec<String>>()
            .join(" ");
        if self.mutable {
            format!("{{{vals}}}")
        } else {
            format!("#{{{vals}}}")
        }
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_display())
    }
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Map{{ {} }}", self.to_external())
    }
}
