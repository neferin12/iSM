use crate::types::Seminar;

#[derive(Debug, Clone)]
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