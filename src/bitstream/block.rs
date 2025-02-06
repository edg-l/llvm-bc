use std::collections::HashMap;

use super::abbrv::Abbr;

const MIN_ABBR_ID_WIDTH: u32 = 2;
const ABBR_INDEX_OFF: u32 = 4;

#[derive(Debug, Clone)]
pub struct AbbrEntry {
    pub abbr: Abbr,
    pub index: u32,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub id: u32,
    pub abbr_id_width: u32,
    pub abbr_map: HashMap<String, AbbrEntry>,
}

impl Block {
    pub fn new(id: u32, abbr_id_width: u32, global_abbrs: Vec<Abbr>) -> Self {
        assert!(
            MIN_ABBR_ID_WIDTH <= abbr_id_width,
            "Abbrev id width too small"
        );
        let mut s = Self {
            id,
            abbr_id_width,
            abbr_map: Default::default(),
        };

        for abbr in global_abbrs {
            s.add_abbr(abbr);
        }

        s
    }

    pub fn add_abbr(&mut self, abbr: Abbr) -> u32 {
        assert!(!self.abbr_map.contains_key(&abbr.name));

        let index = self.abbr_map.len() as u32 + ABBR_INDEX_OFF;
        self.abbr_map
            .insert(abbr.name.clone(), AbbrEntry { abbr, index });
        index
    }
}
