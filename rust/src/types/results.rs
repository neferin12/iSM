use std::cmp::Ordering;
use serde::Serialize;
use crate::types::{Assignment, Seminar, Student};

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct RismResult<'a> {
    points: Option<u16>,
    pub assignments: Vec<Assignment<'a>>,
    pub seminars: &'a Vec<Seminar>,
    pub capacities: Vec<Option<u16>>,
}

impl<'a> RismResult<'a> {
    pub fn get_capacity(&self, seminar: &Seminar) -> u16 {
        self.capacities.get(seminar.id as usize).unwrap().unwrap_or(seminar.capacity)
    }

    fn decrease_capacity_helper(&mut self, seminar: &Seminar) -> Result<(), &str> {
        let remaining_capacity = self.get_capacity(seminar);
        if remaining_capacity <= 0 {
            return Err("Capacity would have been decreased below 0")
        }
        self.capacities[seminar.id as usize] = Some(remaining_capacity - 1);

        Ok(())
    }

    pub fn decrease_capacity_if_left(&mut self, seminar: &Seminar) -> bool {
        self.decrease_capacity_helper(seminar).is_ok()
    }

    pub fn decrease_capacity(&mut self, seminar: Option<&Seminar>) {
        if let Some(seminar_unwr) = seminar {
            self.decrease_capacity_helper(seminar_unwr).unwrap();
        }
    }
    pub fn assign_student_to_seminar(&mut self, student: &Student, seminar: Option<&'a Seminar>, points: u16) {
        let assignment: &mut Assignment = self.assignments.get_mut(student.id as usize).unwrap();
        assignment.assign_seminar(seminar, points);
    }
    
    pub fn calculate_points(&mut self) {
        self.points = Some(self.assignments.iter().map(|a| a.points).sum());
    }
    
    pub fn total_points(&self) -> u16 {
        match self.points {  
            Some(p) => p,
            None => panic!("Tried to access points before calculating them")
        }
    }
    pub fn new(assignments: Vec<Assignment<'a>>, seminars: &'a Vec<Seminar>, capacities: Vec<Option<u16>>) -> Self {
        Self { points: None, assignments, seminars, capacities }
    }
}

impl<'a> PartialOrd<Self> for RismResult<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for RismResult<'a>{
    fn cmp(&self, other: &Self) -> Ordering {
        self.total_points().cmp(&other.total_points())
    }
}