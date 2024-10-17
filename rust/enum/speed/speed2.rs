/// Position and Speed of object during moving actions
mod simple_input;
use simple_input::Input;

const ACCEL: i32 = 10;
const EMERGE_SCALE: i32 = 3;
const MAX_SPEED: i32 = 100;

#[derive(Debug)]
enum Action {
    NoAction(i32),
    Accelerate(i32),
    Decelerate(i32),
    EmergencyBreak(i32),
    Finish,
}

fn main() {
    let mut inp = Input::new();
    let mut state = (0, 0);
    let mut t = 0;
    loop {
        let val = inp.read_int();
        let dur = inp.read_int();
        let action = int_to_action(val, dur);
        if let Action::Finish = action {
            break;
        }
        println!("# {:?}", action);
        println!("{:3} {:3} {:3}", t, state.0, state.1);
        // Suppose, dur is not available
        let dt;
        (state, dt) = step(state, action);
        t += dt;
    }
    // Ausgabe
    println!("# Finish");
    println!("{:3} {:3} {:3}", t, state.0, state.1);
}

fn int_to_action(val: i32, dur: i32) -> Action {
    if dur < 1 {
        Action::Finish
    } else {
        match val {
            -2 => Action::EmergencyBreak(dur),
            -1 => Action::Decelerate(dur),
            0 => Action::NoAction(dur),
            1 => Action::Accelerate(dur),
            _ => Action::Finish,
        }
    }
}

fn step(state: (i32, i32), action: Action) -> ((i32, i32), i32) {
    match action {
        Action::EmergencyBreak(t) => (accelerate(state, -EMERGE_SCALE * ACCEL, t), t),
        Action::Decelerate(t) => (accelerate(state, -ACCEL, t), t),
        Action::NoAction(t) => (accelerate(state, 0, t), t),
        Action::Accelerate(t) => (accelerate(state, ACCEL, t), t),
        Action::Finish => panic!("Finish should not happen!"),
    }
}

fn accelerate(state: (i32, i32), a: i32, t: i32) -> (i32, i32) {
    let (mut x, mut v) = state;
    for _ in 0..t {
        let mut v1 = a + v;
        let mut x1 = a / 2 + v + x;
        if v1 < 0 {
            v1 = 0;
        } else if v1 > MAX_SPEED {
            v1 = MAX_SPEED;
        }
        if x1 < 0 {
            x1 = 0;
        }
        (x, v) = (x1, v1);
    }
    (x, v)
}

#[test]
fn test_accelerate() {
    todo!()
}
