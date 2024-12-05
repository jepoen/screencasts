use std::io;

const CRED_H_MIN: u32 = 45;
const SEM_WEEKS: u32 = 15;
const MIN_P_HOUR: u32 = 60;

struct Course {
    name: String,
    lecture_cred_h: u32,
    tutorial_cred_h: u32,
    factor_self_study: f64,
    exam: bool,
}

fn main() {
    let mut courses = Vec::new();
    for line in io::stdin().lines() {
        let line = line.unwrap();
        match course_from_line(&line) {
            Ok(opt_course) => {
                if let Some(course) = opt_course {
                    courses.push(course);
                }
            }
            Err(err) => println!("Error: {} line: <{}>", err, line),
        }
    }
    show_courses(&courses);
}

fn course_from_line(line: &str) -> Result<Option<Course>, String> {
    let mut it = line.trim().chars();
    // blank lines and comments
    match it.next() {
        Some(c) => {
            if c == '#' {
                return Ok(None);
            }
        }
        None => return Ok(None),
    }
    let parts: Vec<_> = line.split(",").collect();
    if parts.len() != 5 {
        return Err(format!(
            "wrong # of columns, expected 5 got {}",
            parts.len()
        ));
    }
    let lecture_cred_h: u32 = parts[1]
        .parse()
        .map_err(|_| format!("credits is not a number: {}", parts[1]))?;
    let tutorial_cred_h: u32 = parts[2]
        .parse()
        .map_err(|_| format!("credits is not a number: {}", parts[2]))?;
    let factor_self_study: f64 = parts[3]
        .parse()
        .map_err(|_| format!("factor is not a number: {}", parts[3]))?;
    if factor_self_study < 0.0 {
        return Err(format!("factor {} must not be negative", factor_self_study));
    }
    it = parts[4].chars();
    let mut exam = false;
    match it.next() {
        Some(c) => {
            let c_upper = c.to_ascii_uppercase();
            exam = c_upper == 'J' || c_upper == 'Y';
        }
        None => (),
    }
    Ok(Some(Course {
        name: parts[0].to_string(),
        lecture_cred_h,
        tutorial_cred_h,
        factor_self_study,
        exam,
    }))
}

fn show_courses(courses: &[Course]) {
    fn exam_symbol(val: bool) -> char {
        if val {
            '√'
        } else {
            '-'
        }
    }

    for course in courses {
        println!(
            "{:30} {:2} {:2} {:5.1} {}",
            course.name,
            course.lecture_cred_h,
            course.tutorial_cred_h,
            course.factor_self_study,
            exam_symbol(course.exam),
        )
    }
}
