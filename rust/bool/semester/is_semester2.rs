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
    println!("variant 1");
    if is_semester1(month, day) {
        // variant 1
        println!("in lesson period");
    } else {
        println!("vacancy");
    }
    println!("variant 2");
    if is_semester2(month, day) {
        // variant 2
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

// Variant 1
fn is_semester1(month: i32, day: i32) -> bool {
    if month == 1 {
        true
    } else if month == 2 {
        // not 1
        day >= 1 && day <= 15
    } else if month == 3 {
        // 3 .. 12
        false
    } else if month == 4 || month == 5 || month == 6 {
        // 4 .. 12
        true
    } else if month == 7 {
        // 7 ... 12
        day >= 1 && day <= 15
    } else if month == 8 || month == 9 {
        // 8 ... 12
        false
    } else if month == 10 || month == 11 || month == 12 {
        true
    } else {
        // should not happen (month < 1 || month > 12
        false
    }
}

#[test]
fn test_semester1() {
    assert_eq!(is_semester1(1, 16), true);
    // TODO more test cases
}

// Variant 2
fn is_semester2(month: i32, day: i32) -> bool {
    match month {
        // winter
        10 | 11 | 12 | 1 => true,
        2 => day >= 1 && day <= 15,
        // vacation
        3 => false,
        // summer
        4 | 5 | 6 => true,
        7 => day >= 1 && day <= 15,
        // vacation
        8 | 9 => false,
        // should not happen
        _ => panic!("wrong month"),
    }
}

#[test]
fn test_semester2() {
    assert_eq!(is_semester2(1, 16), true);
    // TODO more test cases
}
