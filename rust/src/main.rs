use clap::Parser;
use serde::Serialize;
use rism::rism_classic::run_algorithm;
use rism::constants::get_default_points;
use rism::io::{import_students, import_seminars};
use rism::rism_model_checking::run_model_check;
use console::style;
use tabled::{Table, Tabled};
use tabled::settings::Style;

#[derive(
clap::ValueEnum, Clone, Default, Debug, Serialize, PartialEq
)]
#[serde(rename_all = "kebab-case")]
enum ExecutionVariants {
    #[default]
    Classic,

    ModelChecking,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Students file path
    #[arg(long)]
    students_path: String,

    /// Seminars file path
    #[arg(long)]
    seminars_path: String,

    /// Number of Iterations
    #[arg(short, long)]
    iterations: u32,

    /// Choose variant of calculation
    #[clap(short, long, default_value_t, value_enum)]
    variant: ExecutionVariants,
}

#[derive(Tabled)]
struct PrintableStudent {
    name: String,
    points: u16,
    w_seminar: String,
    p_seminar: String,
}

fn main() {
    let args = Args::parse();
    let seminars = import_seminars(&args.seminars_path);
    let students = import_students(&args.students_path, &seminars);


    let best_iteration = match args.variant {
        ExecutionVariants::Classic => Some(run_algorithm(&students, &seminars, args.iterations, get_default_points())),
        ExecutionVariants::ModelChecking => run_model_check(&students, &seminars, get_default_points())
    };
    if let Some(bi_unwr) = best_iteration {

        let mut students_table_data = Vec::new();

        for a in &bi_unwr.assignments {
            let ps = PrintableStudent {
                name: a.student.name.clone(),
                points: a.points,
                w_seminar: match a.w_seminar {
                    Some(s) => s.name.clone(),
                    None => "Null".to_string()
                },
                p_seminar: match a.p_seminar {
                    Some(s) => s.name.clone(),
                    None => "Null".to_string()
                },
            };
            students_table_data.push(ps);
        }

        let mut table = Table::new(students_table_data);
        table.with(Style::rounded());
        print!("{}\n", table.to_string());
        print!("Total points: {}\n", style(bi_unwr.total_points()).bold().green());
    } else {
        print!("{} Try the classic variant or increase the max points.\n", style("No result was found!").bold().yellow())
    }
}
