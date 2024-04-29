// Defines a seminar, whether it's a wish or an assigned seminar
interface Seminar {
    name: string;
    capacity: number;
    id: number;
    seminar_type: 'Scientific' | 'Practical';
}

// Defines the structure for a student, including their wishes for scientific and practical seminars
interface Student {
    id: number;
    name: string;
    w_wishes: Seminar[]; // Wishes for scientific seminars
    p_wishes?: Seminar[]; // Wishes for practical seminars (optional as your JSON sample has a student missing this field)
}

// Represents an assignment of a student to seminars, including points for the assignment
interface Assignment {
    student: Student;
    w_seminar?: Seminar; // Assigned scientific seminar
    p_seminar?: Seminar; // Assigned practical seminar
    points: number;
}

// The root structure of your JSON data
interface Result {
    points: number;
    assignments: Assignment[];
}