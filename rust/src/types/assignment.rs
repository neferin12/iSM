use crate::types::{Seminar, Student, SeminarType};

#[derive(Debug)]
pub struct Assignment<'a> {
    pub student: &'a Student,
    pub w_seminar: Option<&'a Seminar>,
    pub p_seminar: Option<&'a Seminar>,
    pub points: u16,
}

impl<'a> Assignment<'a> {
    pub fn new(student: &'a Student) -> Assignment<'a> {
        Assignment {
            student,
            w_seminar: None,
            p_seminar: None,
            points: 0,
        }
    }

    pub fn assign_seminar(&mut self, seminar: &'a Seminar, points: u16) {
        self.points += points;
        match seminar.seminar_type {
            SeminarType::Scientific => self.w_seminar = Some(seminar),
            SeminarType::Practical => self.p_seminar = Some(seminar),
        };
    }
}