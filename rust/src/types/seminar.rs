#[derive(PartialEq, Debug, Clone, Eq)]
pub enum SeminarType {
    Scientific,
    Practical
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Seminar {
    pub name: String,
    pub capacity: u16,
    pub id: u16,
    pub seminar_type: SeminarType
}

impl Seminar {
    pub fn new(id: u16, name: String, capacity: u16, seminar_type: SeminarType) -> Seminar {
        Seminar {
            name,
            capacity,
            id,
            seminar_type
        }
    }
}