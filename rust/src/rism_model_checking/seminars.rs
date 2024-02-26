use std::collections::BTreeMap;
use z3::ast::Bool;
use crate::types::{Seminar, Student};

pub(crate) fn seminar_to_string_id(seminar: Option<&Seminar>) -> String {
    match seminar {
        Some(s) => s.id.to_string(),
        None => "x".to_string()
    }
}

pub(crate) fn vote_to_structs<'a>(vote: &String, students: &'a Vec<Student>, seminars: &'a Vec<Seminar>) -> (&'a Student, Option<&'a Seminar>) {
    let mut spl = vote.split("-");
    let stu_id: u16 = spl.next().unwrap().parse().unwrap();
    let sem_id: Option<u16> = spl.next().unwrap().parse().ok();

    (
        students.iter().find(|s| s.id == stu_id).unwrap(),
        match sem_id {
            Some(id) => Some(seminars.iter().find(|s| s.id == id).unwrap()),
            None => None
        }
    )
}

pub(crate) fn sort_votes_to_seminars<'a>(votes: &'a BTreeMap<String, Bool>, students: &'a Vec<Student>, seminars: &'a Vec<Seminar>) -> BTreeMap<&'a Seminar, Vec<(&'a String, &'a Bool<'a>)>> {
    votes
        .iter()
        .map(|(k, b)| {
            (k, b, vote_to_structs(k, students, seminars).1)
        }).fold(BTreeMap::new(), |mut acc: BTreeMap<&Seminar, Vec<(&String, &Bool)>>, (k, b, sem)| {
        if sem.is_none(){
            return acc;
        }
        let mut assoc_votes = acc.get_mut(&sem.unwrap());
        if assoc_votes.is_none() {
            acc.insert(sem.unwrap(), Vec::new());
            assoc_votes = acc.get_mut(&sem.unwrap());
        }
        assoc_votes.unwrap().push((k, b));

        acc
    })
}