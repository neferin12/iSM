use std::collections::BTreeMap;
use crate::constants::Points;
use crate::types::{Seminar, Student, RismResult, SeminarType};
use z3::{Config, Context, Optimize, ast};
use z3::ast::{Int, Bool};

fn seminar_to_string_id(seminar: &Seminar) -> String {
    return format!("sem{}", seminar.id);
}

fn student_to_string_id(student: &Student) -> String {
    return format!("stu{}", student.id);
}

fn vote_to_string_id(student: &Student, seminar_type: &SeminarType, index: usize) -> String {
    return format!("{0}-{1}-v{2}", student_to_string_id(student), seminar_type, index);
}

fn votes_to_string_id(student: &Student, seminar_type: &SeminarType) -> Vec<String> {
    match seminar_type {
        SeminarType::Scientific => &student.w_wishes,
        SeminarType::Practical => &student.p_wishes,
    }
        .iter()
        .enumerate()
        .map(|(i, _s)| vote_to_string_id(student, seminar_type, i))
        .collect()
}

pub fn run_model_check<'a>(students: &Vec<Student>, seminars: &Vec<Seminar>, points: Points, max_points: u16) -> Option<RismResult<'a>> {
    let mut cfg = Config::new();
    cfg.set_model_generation(true);
    let ctx = Context::new(&cfg);
    let opt = Optimize::new(&ctx);

    let points_first_selection = Int::from_u64(&ctx, points.first_selection as u64);
    let points_second_selection = Int::from_u64(&ctx, points.second_selection as u64);
    let points_third_selection = Int::from_u64(&ctx, points.third_selection as u64);
    let points_no_selection = Int::from_u64(&ctx, points.no_selection as u64);

    let mut votes = BTreeMap::new();

    for s in students {
        let vote_keys_p = votes_to_string_id(s, &SeminarType::Practical);
        let vote_keys_w = votes_to_string_id(s, &SeminarType::Scientific);

        vote_keys_p.iter()
            .map(|k| (k.clone(), Bool::new_const(&ctx, k.clone())))
            .for_each(|(k, v)| { votes.insert(k, v); });

        vote_keys_w.iter()
            .map(|k| (k.clone(), Bool::new_const(&ctx, k.clone())))
            .for_each(|(k, v)| { votes.insert(k, v); });

        opt.assert(
            votes.get(&vote_keys_p[0])
                .xor(votes.get(&vote_keys_p[1]))
                .xor(votes.get(&vote_keys_p[2])).unwrap()
        );
        opt.assert(
            votes.get(&vote_keys_w[0])
                .xor(votes.get(&vote_keys_w[1]))
                .xor(votes.get(&vote_keys_w[2])).unwrap()
        )
    }

    let res = opt.check(&[]);
    println!("{:?}", opt.get_model());

    return None;
}