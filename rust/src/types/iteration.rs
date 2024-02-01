use crate::types::{Assignment, Seminar};

#[derive(Debug)]
pub struct Iteration<'a> {
    points: Option<u16>,
    pub assignments: Vec<Assignment<'a>>,
    pub seminars: &'a Vec<Seminar>,
    pub capacities: Vec<u16>
}

impl<'a> Iteration<'a> {


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