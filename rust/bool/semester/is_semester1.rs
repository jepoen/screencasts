/// checks if date inside semester (lesson period)
/// Call: ./is_semester1 month day
mod simple_args;

fn main() {
    let args = simple_args::int_args();
    if args.len() != 2 {
        simple_args::usage("month day");
        return;
    }
    let month = args[0];
    let day = args[1];
    println!("Day: {:02}-{:02}", month, day);
    if !is_valid_date(month, day) {
        println!("Invalid Date");
        return;
    }
    // date is valid
    if is_semester(month, day) {
        println!("in lesson period");
    } else {
        println!("vacancy");
    }
}

fn is_valid_date(month: i32, day: i32) -> bool {
    if month >= 1 && month <= 12 {
        // valid month
        if day >= 1 && day <= 31 {
            return true;
        }
    }
    false
}

#[test]
fn test_valid_date() {
    assert_eq!(is_valid_date(3, 15), true);
    assert_eq!(is_valid_date(12, 31), true);
    assert_eq!(is_valid_date(14, 12), false);
    // TODO more cases
}

fn is_semester(month: i32, day: i32) -> bool {
    is_winter_sem(month, day) || is_summer_sem(month, day)
}

#[test]
fn test_semester() {
    // TODO
}

// without day (July)
#[allow(dead_code)]
fn is_summer_sem_naive(month: i32, _day: i32) -> bool {
    month >= 4 && month <= 7
    // return month >= 4 && month <= 7;
}

fn is_summer_sem(month: i32, day: i32) -> bool {
    (month >= 4 && month <= 6) || (month == 7 && day >= 1 && day <= 15)
}

#[test]
fn test_summer_sem() {
    // TODO
}

// without day (Feb.)
#[allow(dead_code)]
fn is_winter_sem_naive(month: i32, _day: i32) -> bool {
    // old year
    month >= 10 && month <= 12
	||
	// new year
	month >= 1 && month <= 2
}

fn is_winter_sem(month: i32, day: i32) -> bool {
    (month >= 10 && month <= 12) || month == 1 || (month == 2 && day >= 1 && day <= 15)
}

#[test]
fn test_winter_sem() {
    assert_eq!(is_winter_sem(10, 1), true);
    assert_eq!(is_winter_sem(12, 31), true);
    assert_eq!(is_winter_sem(1, 1), true);
    assert_eq!(is_winter_sem(2, 15), true);
    assert_eq!(is_winter_sem(9, 15), false);
    assert_eq!(is_winter_sem(3, 1), false);
    assert_eq!(is_winter_sem(2, 16), false);
}
