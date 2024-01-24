use crate::types::{Seminar, Student, SeminarType};

#[derive(Debug)]
pub struct Assignment {
    pub student: Student,
    pub w_seminar: Option<Seminar>,
    pub p_seminar: Option<Seminar>,
    pub points: u16,
}

impl Assignment {
    pub fn new(student: Student) -> Assignment {
        Assignment {
            student,
            w_seminar: None,
            p_seminar: None,
            points: 0,
        }
    }

    pub fn assign_seminar(&mut self, seminar: Seminar, points: u16) {
        self.points += points;
        match seminar.seminar_type {
            SeminarType::Scientific => self.w_seminar = Some(seminar),
            SeminarType::Practical => self.p_seminar = Some(seminar),
        };
    }
}