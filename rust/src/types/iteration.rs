use crate::types::{Assignment, Seminar};

#[derive(Debug)]
pub struct Iteration {
    points: Option<u16>,
    pub assignments: Vec<Assignment>,
    pub w_seminars: Vec<Seminar>,
    pub p_seminars: Vec<Seminar>,
}

impl Iteration {
    pub fn total_points(&mut self) -> u16 {
        if self.points.is_none() {
            self.points = Some(self.assignments.iter().map(|a| a.points).sum());
        }
        self.points.unwrap()
    }
}