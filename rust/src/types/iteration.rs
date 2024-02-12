use crate::constants::Points;
use crate::types::{Assignment, Seminar, Student};

#[derive(Debug)]
pub struct Iteration<'a> {
    points: Option<u16>,
    pub assignments: Vec<Assignment<'a>>,
    pub seminars: &'a Vec<Seminar>,
    pub capacities: Vec<u16>,
}

impl<'a> Iteration<'a> {
    pub fn decrease_capacity_if_left(&mut self, seminar: &Seminar) -> bool {
        let remaining_capacity: u16 = *self.capacities.get(seminar.id as usize).unwrap_or(&seminar.capacity);

        if remaining_capacity <= 0 {
            return false;
        }

        self.capacities[seminar.id as usize] = remaining_capacity - 1;

        return true;
    }
    pub fn assign_student_to_seminar(&mut self, student: &Student, seminar: &'a Seminar, points: u16) {
        let assignment: &mut Assignment = self.assignments.get_mut(student.id as usize).unwrap();
        assignment.assign_seminar(seminar, points);
    }
    pub fn total_points(&mut self) -> u16 {
        if self.points.is_none() {
            self.points = Some(self.assignments.iter().map(|a| a.points).sum());
        }
        self.points.unwrap()
    }
    pub fn new(assignments: Vec<Assignment<'a>>, seminars: &'a Vec<Seminar>, capacities: Vec<u16>) -> Self {
        Self { points: None, assignments, seminars, capacities }
    }
}