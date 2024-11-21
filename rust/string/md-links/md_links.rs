use std::io;

fn main() {
    let mut text = String::new();
    while io::stdin().read_line(&mut text).unwrap() > 0 {}
    println!("{}", text);
    let link_list = dfa(&text);
    println!("Links:");
    show_link_list(&link_list);
}

fn dfa(text: &str) -> Vec<(String, String)> {
    enum State {
        S0,
        Title,
        Quot,
        S1,
        Url,
    }
    let mut state = State::S0;
    let mut link_list = Vec::new();
    let mut title = String::new();
    let mut url = String::new();
    for c in text.chars() {
        match state {
            State::S0 => {
                if c == '[' {
                    title.clear();
                    state = State::Title;
                }
            }
            State::Title => match c {
                ']' => state = State::S1,
                '[' => title.clear(),
                '\\' => state = State::Quot,
                _ => title.push(c),
            },
            State::Quot => {
                title.push(c);
                state = State::Title;
            }
            State::S1 => match c {
                '(' => {
                    url.clear();
                    state = State::Url;
                }
                '[' => {
                    title.clear();
                    state = State::Title;
                }
                _ => state = State::S0,
            },
            State::Url => {
                match c {
                    '[' => {
                        title.clear();
                        state = State::Title;
                    }
                    ')' => {
                        println!("found {} {}", title, url);
                        link_list.push((title.clone(), url.clone()));
                        title.clear();
                        url.clear();
                        state = State::S0;
                    }
                    _ => url.push(c),
                    // TODO: \ in Url state
                }
            }
        }
    }
    link_list
}

fn show_link_list(link_list: &[(String, String)]) {
    for (title, url) in link_list {
        println!("{:20} {}", title, url);
        // TODO: shorten long title
    }
}
