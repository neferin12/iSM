use std::cmp::Ordering;
use crate::types::Seminar;

#[derive(Debug, Clone, Eq)]
pub struct Student {
    pub id: u16,
    pub name: String,
    pub w_wishes: Vec<Seminar>,
    pub p_wishes: Vec<Seminar>,
}

impl Student {
    pub fn new(id: u16, name: String, w_wishes: Vec<Seminar>, p_wishes: Vec<Seminar>) -> Student {
        Student {
            id,
            name,
            w_wishes,
            p_wishes,
        }
    }
}

impl PartialEq for Student {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Ord for Student {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Student {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}