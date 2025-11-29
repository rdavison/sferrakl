use crate::key::{Id, Src, ANSI30};
use std::collections::HashMap;

pub struct Map<T>(pub HashMap<(Id, Id), T>);

impl<T> Map<T> {
    pub fn init(src: Src, f: &dyn Fn(Id, Id) -> T) -> Self {
        let key_ids = match src {
            Src::Ansi30 => &ANSI30,
        };

        let mut map = HashMap::new();
        for &id1 in key_ids.iter() {
            for &id2 in key_ids.iter() {
                map.insert((id1, id2), f(id1, id2));
            }
        }

        Map(map)
    }
}
