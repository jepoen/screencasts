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

fn supervised_week_minutes(course: &Course) -> u32 {
    (course.lecture_cred_h + course.tutorial_cred_h) * CRED_H_MIN
}

fn unsupervised_week_minutes(course: &Course) -> u32 {
    (supervised_week_minutes(course) as f64 * course.factor_self_study) as u32
}

fn supervised_semester_minutes(course: &Course) -> u32 {
    supervised_week_minutes(course) * SEM_WEEKS
}

fn unsupervised_semester_minutes(course: &Course) -> u32 {
    unsupervised_week_minutes(course) * SEM_WEEKS
}

struct HourMin {
    hour: u32,
    min: u32,
}

fn min_to_hourmin(val: u32) -> HourMin {
    HourMin {
        hour: val / MIN_P_HOUR,
        min: val % MIN_P_HOUR,
    }
}

fn min_to_hour_str(val: u32) -> String {
    let hm = min_to_hourmin(val);
    format!("{:3}:{:02} h", hm.hour, hm.min)
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
    println!();
    show_times(&courses);
    println!();
    show_total_time(&courses);
    let exams = count_exams(&courses);
    println!("# exams: {}", exams);
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
        );
    }
}

fn show_times(courses: &[Course]) {
    for course in courses {
        let supervised_week = supervised_week_minutes(course);
        let unsupervised_week = unsupervised_week_minutes(course);
        let supervised_semester = supervised_semester_minutes(course);
        let unsupervised_semester = unsupervised_semester_minutes(course);
        let sum_week = supervised_week + unsupervised_week;
        let sum_semester = supervised_semester + unsupervised_semester;
        println!("Course: {}", course.name);
        println!(
            "- per week supervised:     {} unsupervised {} sum {}",
            min_to_hour_str(supervised_week),
            min_to_hour_str(unsupervised_week),
            min_to_hour_str(sum_week),
        );
        println!(
            "- per semester supervised: {} unsupervised {} sum {}",
            min_to_hour_str(supervised_semester),
            min_to_hour_str(unsupervised_semester),
            min_to_hour_str(sum_semester),
        );
        println!();
    }
}

fn show_total_time(courses: &[Course]) {
    let mut sum_week = 0;
    let mut sum_semester = 0;
    for course in courses {
        let supervised_week = supervised_week_minutes(course);
        let unsupervised_week = unsupervised_week_minutes(course);
        sum_week += supervised_week + unsupervised_week;
        let supervised_semester = supervised_semester_minutes(course);
        let unsupervised_semester = unsupervised_semester_minutes(course);
        sum_semester += supervised_semester + unsupervised_semester;
    }
    println!(
        "Total: week: {} semester {}",
        min_to_hour_str(sum_week),
        min_to_hour_str(sum_semester),
    )
}

fn count_exams(courses: &[Course]) -> u32 {
    let mut count = 0;
    for course in courses {
        if course.exam {
            count += 1;
        }
    }
    count
}
