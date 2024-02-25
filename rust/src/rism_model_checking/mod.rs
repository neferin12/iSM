use std::collections::BTreeMap;
use std::ops::Add;
use crate::constants::Points;
use crate::types::{Seminar, Student, RismResult, SeminarType};
use z3::{Config, Context, Optimize, SatResult, Solver};
use z3::ast::{Int, Bool};

fn seminar_to_string_id(seminar: &Seminar) -> String {
    format!("sem{}", seminar.id)
}

fn student_to_string_id(student: &Student) -> String {
    format!("stu{}", student.id)
}

fn vote_to_string_id(student: &Student, seminar_type: &SeminarType, index: usize) -> String {
    format!("{0}-{1}-v{2}", student_to_string_id(student), seminar_type, index)
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

fn exactly_one_of<'a>(ctx: &'a Context, bools: Vec<Bool<'a>>) -> Bool<'a> {
    let mut results = Vec::new();

    for i in 0..bools.len() {
        let part = bools
            .clone()
            .into_iter()
            .enumerate()
            .map(
                |(i2, b)| {
                    if i == i2 {
                        b
                    } else {
                        b.not()
                    }
                })
            .reduce(|acc, e| Bool::and(ctx, &[&acc, &e]));
        results.push(part.unwrap());
    }

    return results.into_iter().reduce(|acc, e| Bool::or(ctx, &[&acc, &e])).unwrap();
}

pub fn run_model_check<'a>(students: &Vec<Student>, seminars: &Vec<Seminar>, points: Points, max_points: u16) -> Option<RismResult<'a>> {
    let mut cfg = Config::new();
    cfg.set_model_generation(true);
    cfg.set_proof_generation(true);
    let ctx = Context::new(&cfg);
    // let opt = Optimize::new(&ctx);
    let opt = Solver::new(&ctx);

    let points_first_selection = Int::from_u64(&ctx, points.first_selection as u64);
    let points_second_selection = Int::from_u64(&ctx, points.second_selection as u64);
    let points_third_selection = Int::from_u64(&ctx, points.third_selection as u64);
    let points_no_selection = Int::from_u64(&ctx, points.no_selection as u64);

    let mut votes = BTreeMap::new();

    let mut s_points_vec: Vec<Int> = Vec::new();

    for s in students {
        let mut vote_keys_p = votes_to_string_id(s, &SeminarType::Practical);
        vote_keys_p.push(vote_to_string_id(s, &SeminarType::Practical, 3));
        let mut vote_keys_w = votes_to_string_id(s, &SeminarType::Scientific);
        vote_keys_w.push(vote_to_string_id(s, &SeminarType::Scientific, 3));

        vote_keys_p.iter()
            .map(|k| (k.clone(), Bool::new_const(&ctx, k.clone())))
            .for_each(|(k, v)| { votes.insert(k, v); });

        vote_keys_w.iter()
            .map(|k| (k.clone(), Bool::new_const(&ctx, k.clone())))
            .for_each(|(k, v)| { votes.insert(k, v); });

        opt.assert(
            &exactly_one_of(
                &ctx,
                vote_keys_p.iter().map(|k| votes.get(k).unwrap().clone()).collect(),
            )
        );

        opt.assert(
            &exactly_one_of(
                &ctx,
                vote_keys_w.iter().map(|k| votes.get(k).unwrap().clone()).collect(),
            )
        );

        let s_points_w = Int::new_const(&ctx, format!("{}-w-points", student_to_string_id(s)));
        opt.assert(
            &votes.get(&vote_keys_w[0]).unwrap().implies(
                &Bool::from_bool(&ctx, s_points_w.eq(&points_first_selection))
            )
        );
        opt.assert(
            &votes.get(&vote_keys_w[1]).unwrap().implies(
                &Bool::from_bool(&ctx, s_points_w.eq(&points_second_selection))
            )
        );
        opt.assert(
            &votes.get(&vote_keys_w[2]).unwrap().implies(
                &Bool::from_bool(&ctx, s_points_w.eq(&points_third_selection))
            )
        );
        opt.assert(
            &votes.get(&vote_keys_w[3]).unwrap().implies(
                &Bool::from_bool(&ctx, s_points_w.eq(&points_no_selection))
            )
        );
        
        let s_points_p = Int::new_const(&ctx, format!("{}-p-points", student_to_string_id(s)));
        opt.assert(
            &votes.get(&vote_keys_p[0]).unwrap().implies(
                &Bool::from_bool(&ctx, s_points_p.eq(&points_first_selection))
            )
        );
        opt.assert(
            &votes.get(&vote_keys_p[1]).unwrap().implies(
                &Bool::from_bool(&ctx, s_points_p.eq(&points_second_selection))
            )
        );
        opt.assert(
            &votes.get(&vote_keys_p[2]).unwrap().implies(
                &Bool::from_bool(&ctx, s_points_p.eq(&points_third_selection))
            )
        );
        opt.assert(
            &votes.get(&vote_keys_w[3]).unwrap().implies(
                &Bool::from_bool(&ctx, s_points_p.eq(&points_no_selection))
            )
        );
        
        let s_points = s_points_p.add(s_points_w);
        s_points_vec.push(s_points);
    };

    let mut total_points = Int::from_u64(&ctx, 0);

    for p in s_points_vec {
        total_points = total_points.add(p);
    };

    if let SatResult::Sat = opt.check() {
        // opt.minimize(&total_points);
        println!("{:?}", opt.get_model());
        println!("Total Points: {:?}", opt.get_model().unwrap().get_const_interp(&total_points));
    } else {
        println!("{:?}", opt.get_proof());
        println!("Unsatisfieable")
    };
    return None;
}