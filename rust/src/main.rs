use clap::Parser;
use rism::io::{import_students, import_seminars};

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
}

fn main() {
    let args = Args::parse();
    let seminars = import_seminars(&args.seminars_path);
    let students = import_students(&args.students_path, seminars);
    print!("students");
}
