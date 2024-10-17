/// Stone-Scissors-Paper
mod simple_input;
use simple_input::Input;

//#[derive(Debug, Clone, Copy)]
#[derive(Debug)]
enum State {
    Start,
    S1,
    S2,
    S3,
    Stop,
    Error,
}

enum Action {
    A1,
    A2,
    A3,
    Stop,
    Error,
}

fn main() {
    let mut inp = Input::new();
    let mut state = State::Start;
    println!("{:?}", state);
    while !is_terminal(&state) {
        let action = int_to_action(inp.read_int());
        state = next_state(state, action);
        println!("{:?}", state);
    }
    match state {
        State::Stop => println!("Valid strategy"),
        State::Error => println!("Invalid strategy or action"),
        _ => panic!("not terminal state {:?}", state),
    }
}

fn int_to_action(val: i32) -> Action {
    match val {
        0 => Action::Stop,
        1 => Action::A1,
        2 => Action::A2,
        3 => Action::A3,
        _ => Action::Error,
    }
}

fn next_state(state: State, action: Action) -> State {
    match state {
        State::Start => match action {
            Action::A1 => State::S1,
            Action::A2 => State::S2,
            Action::A3 => State::S3,
            _ => State::Error,
        },
        State::S1 => match action {
            Action::Stop => State::Stop,
            Action::A2 => State::S2,
            Action::A3 => State::S3,
            _ => State::Error,
        },
        State::S2 => match action {
            Action::Stop => State::Stop,
            Action::A1 => State::S1,
            Action::A3 => State::S3,
            _ => State::Error,
        },
        State::S3 => match action {
            Action::Stop => State::Stop,
            Action::A1 => State::S1,
            Action::A2 => State::S2,
            _ => State::Error,
        },
        _ => State::Error,
    }
}

fn is_terminal(state: &State) -> bool {
    match state {
        State::Stop | State::Error => true,
        _ => false,
    }
}
