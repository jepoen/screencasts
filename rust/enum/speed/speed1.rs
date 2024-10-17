/// Position and Speed of object during moving actions
mod simple_input;
use simple_input::Input;

const ACCEL: i32 = 10;
const EMERGE_SCALE: i32 = 3;
const MAX_SPEED: i32 = 100;

#[derive(Debug)]
enum Action {
    NoAction,
    Accelerate,
    Decelerate,
    EmergencyBreak,
    Finish,
}

fn main() {
    let mut inp = Input::new();
    let mut state = (0, 0);
    let mut t = 0;
    loop {
        let val = inp.read_int();
        let action = int_to_action(val);
        if let Action::Finish = action {
            break;
        }
        println!("# {:?}", action);
        println!("{:3} {:3} {:3}", t, state.0, state.1);
        state = step(state, action);
        t += 1;
    }
    // Ausgabe
    println!("#Finish");
    println!("{:3} {:3} {:3}", t, state.0, state.1);
}

fn int_to_action(val: i32) -> Action {
    match val {
        -2 => Action::EmergencyBreak,
        -1 => Action::Decelerate,
        0 => Action::NoAction,
        1 => Action::Accelerate,
        _ => Action::Finish,
    }
}

fn step(state: (i32, i32), action: Action) -> (i32, i32) {
    match action {
        Action::EmergencyBreak => accelerate(state, -EMERGE_SCALE * ACCEL),
        Action::Decelerate => accelerate(state, -ACCEL),
        Action::NoAction => accelerate(state, 0),
        Action::Accelerate => accelerate(state, ACCEL),
        Action::Finish => panic!("Finish should not happen!"),
    }
}

fn accelerate(state: (i32, i32), a: i32) -> (i32, i32) {
    let (x0, v0) = state;
    let mut v1 = a + v0;
    let mut x1 = a / 2 + v0 + x0;
    if v1 < 0 {
        v1 = 0;
    } else if v1 > MAX_SPEED {
        v1 = MAX_SPEED;
    }
    if x1 < 0 {
        x1 = 0;
    }
    (x1, v1)
}

#[test]
fn test_accelerate() {
    todo!()
}
