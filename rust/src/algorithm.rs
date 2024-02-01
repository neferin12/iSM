use crate::types::{Assignment, Iteration, Seminar, Student};
use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::constants::Points;

fn try_assignment(student: &Student, seminar: &Seminar, iteration: &mut Iteration, points: u16) -> bool {
    let remaining_capacity: &u16 = iteration.capacities.get(seminar.id as usize).unwrap_or(&seminar.capacity);

    if (remaining_capacity <= &0) {
        return false;
    }

    iteration.capacities[seminar.id as usize] = remaining_capacity - 1;

    let assignment: &mut Assignment = iteration.assignments.get_mut(student.id as usize).unwrap();
    assignment.assign_seminar(seminar, points);

    return true;
}

pub fn run_algorithm(mut students: Vec<Student>, seminars: Vec<Seminar>, iterations: u16, points: Points) {
    let best_iteration: Option<Iteration> = None;

    for i in 0..iterations {
        students.shuffle(&mut thread_rng());

        let mut iteration: Iteration = Iteration::new(Vec::new(), &seminars, Vec::new());

        for s in students {
            iteration.assignments[s.id as usize] = Assignment::new(&s);
        }
    }
}