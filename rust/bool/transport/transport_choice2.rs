mod simple_args;

const DUR_WALK: i32 = 30;
const DUR_TRAM: i32 = 20;
const DUR_BIKE: i32 = 10;

fn main() {
    let args = simple_args::int_args();
    if args.len() != 2 {
        simple_args::usage("wake_up_time start_lesson_time");
        return;
    }
    let wake_up = args[0];
    let lesson_start = args[1];
    transport_choice(wake_up, lesson_start);
}

// TODO: Improvement: return choice as function value -> better testing
fn transport_choice(wake_up: i32, lesson_start: i32) {
    let delta = time_diff(lesson_start, wake_up);
    if delta > DUR_WALK {
        println!("Walk");
    }
    if delta > DUR_TRAM {
        println!("Tram");
    }
    if delta > DUR_BIKE {
        println!("Bike");
    }
    if delta > 0 {
        println!("Online");
    } else {
        println!("Too late :-)");
    }
}

// timediff between t0 and t1 (t0 - t1)
fn time_diff(t0: i32, t1: i32) -> i32 {
    time_to_mins(t0) - time_to_mins(t1)
}

#[test]
fn test_time_diff() {
    assert_eq!(time_diff(900, 730), 90);
    assert_eq!(time_diff(815, 745), 30);
}

fn time_to_mins(t: i32) -> i32 {
    // TODO check if valid (mins <= 59, 0 <= hours < 24)
    (t / 100) * 60 + t % 100
}

#[test]
fn test_time_to_mins() {
    // TODO
}
