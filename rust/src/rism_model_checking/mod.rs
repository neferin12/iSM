use std::collections::BTreeMap;
use std::ops::Add;
use crate::constants::Points;
use crate::types::{Seminar, Student, RismResult, SeminarType};
use z3::{Config, Context, Optimize, SatResult, Solver};
use z3::ast::{Int, Bool, Ast};

fn seminar_to_string_id(seminar: &Seminar) -> String {
    format!("sem{}", seminar.id)
}

fn student_to_string_id(student: &Student) -> String {
    format!("stu{}", student.id)
}

fn vote_to_string_id(student: &Student, seminar_type: &SeminarType, index: usize) -> String {
    format!("{0}-{1}-v{2}", student_to_string_id(student), seminar_type, index)
}

fn student_to_points_id(student: &Student, seminar_type: SeminarType) -> String {
    format!("{0}-{1}-points",student.id, seminar_type)
}

fn votes_to_string_id(student: &Student, seminar_type: &SeminarType) -> Vec<String> {
    let mut stud_votes: Vec<String> = match seminar_type {
        SeminarType::Scientific => &student.w_wishes,
        SeminarType::Practical => &student.p_wishes,
    }
        .iter()
        .enumerate()
        .map(|(i, _s)| vote_to_string_id(student, seminar_type, i))
        .collect();
    stud_votes.push(vote_to_string_id(student, seminar_type, 3));

    stud_votes
}

fn exactly_one_of<'a>(ctx: &'a Context, bools: Vec<Bool<'a>>) -> Bool<'a> {
    // let mut results = Vec::new();
    //
    // for i in 0..bools.len() {
    //     let part = bools
    //         .clone()
    //         .into_iter()
    //         .enumerate()
    //         .map(
    //             |(i2, b)| {
    //                 if i == i2 {
    //                     b
    //                 } else {
    //                     b.not()
    //                 }
    //             })
    //         .reduce(|acc, e| Bool::and(ctx, &[&acc, &e]));
    //     results.push(part.unwrap());
    // }
    //
    // return results.into_iter().reduce(|acc, e| Bool::or(ctx, &[&acc, &e])).unwrap();
    let t: Vec<(&Bool, i32)> = bools.iter().map(|b| (b, 1)).collect();
    return Bool::pb_eq(ctx, &*t, 1);
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
    let mut points = BTreeMap::new();

    let mut s_points_vec: Vec<Int> = Vec::new();

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

        let key_w = student_to_points_id(s, SeminarType::Scientific);
        let s_points_w = Int::new_const(&ctx, key_w.clone());
        points.insert(key_w.clone(), s_points_w);

        let s_points_w = points.get(&key_w).unwrap();
        opt.assert(
            &votes.get(&vote_keys_w[0]).unwrap().implies(
                &&s_points_w._eq(&points_first_selection)
            )
        );
        opt.assert(
            &votes.get(&vote_keys_w[1]).unwrap().implies(
                &&s_points_w._eq(&points_second_selection)
            )
        );
        opt.assert(
            &votes.get(&vote_keys_w[2]).unwrap().implies(
                &&s_points_w._eq(&points_third_selection)
            )
        );
        opt.assert(
            &votes.get(&vote_keys_w[3]).unwrap().implies(
                &&s_points_w._eq(&points_no_selection)
            )
        );


        let key_p = student_to_points_id(s, SeminarType::Practical);
        let s_points_p = Int::new_const(&ctx, key_p.clone());
        points.insert(key_p.clone(), s_points_p);

        let s_points_p = points.get(&key_p).unwrap();
        opt.assert(
            &votes.get(&vote_keys_p[0]).unwrap().implies(
                &&s_points_p._eq(&points_first_selection)
            )
        );
        opt.assert(
            &votes.get(&vote_keys_p[1]).unwrap().implies(
                &&s_points_p._eq(&points_second_selection)
            )
        );
        opt.assert(
            &votes.get(&vote_keys_p[2]).unwrap().implies(
                &&s_points_p._eq(&points_third_selection)
            )
        );
        opt.assert(
            &votes.get(&vote_keys_p[3]).unwrap().implies(
                &&s_points_p._eq(&points_no_selection)
            )
        );

        let s_points = points.get(&key_p.clone()).unwrap().add(points.get(&key_w).unwrap());
        s_points_vec.push(s_points);
    };

    let mut total_points = Int::from_u64(&ctx, 0);

    for p in s_points_vec {
        total_points = total_points.add(p);
    };

    println!("{:?}", opt.get_assertions());

    if let SatResult::Sat = opt.check() {
        // opt.minimize(&total_points);
        let model = opt.get_model().unwrap();
        // println!("{:?}", model);

        let s = &students[0];
        println!("Details for Student {}", s.name);
        println!("Practical: {:?}",
                 votes_to_string_id(s, &SeminarType::Practical)
                     .iter()
                     .map(|s| votes.get(s).unwrap())
                     .map(|v| model.get_const_interp(v).unwrap())
                     .collect::<Vec<Bool>>());
        println!("Practical points: {:?}", model.get_const_interp(points.get(&student_to_points_id(s, SeminarType::Practical)).unwrap()));

        println!("Total Points: {:?}", opt.get_model().unwrap().get_const_interp(&total_points));
    } else {
        println!("{:?}", opt.get_proof().unwrap());
        println!("Unsatisfieable")
    };
    return None;
}
