use std::cmp::Ordering;
use std::fmt::Display;

#[derive(PartialEq, Debug, Clone, Eq)]
pub enum SeminarType {
    Scientific,
    Practical
}

impl Display for SeminarType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            SeminarType::Scientific => "Scientific".to_string(),
            SeminarType::Practical => "Practical".to_string()
        };
        write!(f, "{}", str)
    }
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

impl Ord for Seminar {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Seminar {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}