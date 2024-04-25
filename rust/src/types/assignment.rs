use std::cmp::Ordering;
use crate::types::{Seminar, Student, SeminarType};

#[derive(Debug, Eq, Clone)]
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

    pub fn assign_seminar(&mut self, seminar: Option<&'a Seminar>, points: u16) {
        self.points += points;
        if let Some(seminar) = seminar {
            match seminar.seminar_type {
                SeminarType::Scientific => self.w_seminar = Some(seminar),
                SeminarType::Practical => self.p_seminar = Some(seminar),
            };
        }
    }
}


impl<'a> Ord for Assignment<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.points.cmp(&other.points)
    }
}

impl<'a> PartialOrd for Assignment<'a>{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for Assignment<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.student.id.eq(&other.student.id)
    }
}

