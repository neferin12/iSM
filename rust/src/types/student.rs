use crate::types::Seminar;

#[derive(Debug)]
pub struct Student {
    pub name: String,
    pub w_wishes: Vec<Seminar>,
    pub p_wishes: Vec<Seminar>,
}

impl Student {
    pub fn new(name: String, w_wishes: Vec<Seminar>, p_wishes: Vec<Seminar>) -> Student {
        Student {
            name,
            w_wishes,
            p_wishes,
        }
    }
}