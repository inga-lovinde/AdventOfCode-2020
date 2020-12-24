use std::fmt;
use std::{io::{self, BufRead}};

#[derive(Clone, Copy)]
struct Entry {
    previous_value: usize,
    next_value: usize,
}

struct GameState {
    current_cup: usize,
    state: Vec<Entry>,
}

impl GameState {
    fn new(raw_data: &[usize]) -> GameState {
        let data: Vec<_> = raw_data.iter().map(|&x| x-1).collect();
        let mut state = vec![None; data.len()];
        state[data[0]] = Some(Entry {
            previous_value: data[data.len()-1],
            next_value: data[1],
        });
        for i in 1..data.len()-1 {
            state[data[i]] = Some(Entry {
                previous_value: data[i-1],
                next_value: data[i+1],
            });
        }
        state[data[data.len()-1]] = Some(Entry {
            previous_value: data[data.len()-2],
            next_value: data[0],
        });

        GameState {
            current_cup: data[0],
            state: state.into_iter().collect::<Option<_>>().unwrap(),
        }
    }

    fn get_next_value(&self, value: usize) -> usize {
        self.state[value].next_value
    }

    // unsafe, leaves self in an inconsistent state, only invoke from link_values
    fn set_next_value(&mut self, value: usize, new_next_value: usize) {
        self.state[value] = Entry {
            next_value: new_next_value,
            ..self.state[value]
        }
    }

    // unsafe, leaves self in an inconsistent state, only invoke from link_values
    fn set_previous_value(&mut self, value: usize, new_previous_value: usize) {
        self.state[value] = Entry {
            previous_value: new_previous_value,
            ..self.state[value]
        }
    }

    // unsafe, leaves self in an inconsistent state, only invoke from move_value
    fn link_values(&mut self, previous_value: usize, next_value: usize) {
        self.set_next_value(previous_value, next_value);
        self.set_previous_value(next_value, previous_value);
    }

    fn move_value(&mut self, value: usize, new_previous: usize) {
        let old_previous = self.state[value].previous_value;
        let old_next = self.state[value].next_value;
        let new_next = self.state[new_previous].next_value;
        self.link_values(old_previous, old_next);
        self.link_values(new_previous, value);
        self.link_values(value, new_next);
    }

    fn get_destination_cup(&self, current_cup: usize) -> usize {
        let blacklisted_cups = [
            current_cup,
            self.get_next_value(current_cup),
            self.get_next_value(self.get_next_value(current_cup)),
            self.get_next_value(self.get_next_value(self.get_next_value(current_cup))),
        ];

        let mut destination_label = current_cup;
        loop {
            if destination_label == 0 {
                destination_label = self.state.len();
            }
            destination_label -= 1;

            if !blacklisted_cups.contains(&destination_label) {
                return destination_label;
            }
        }
    }

    fn mutate_next_step(&mut self) {
        let current_cup = self.current_cup;
        let mut destination_cup = self.get_destination_cup(current_cup);
        for _i in 0..3 {
            let value_to_move = self.get_next_value(current_cup);
            self.move_value(value_to_move, destination_cup);
            destination_cup = value_to_move;
        }
        
        self.current_cup = self.get_next_value(current_cup);
    }

    fn get_hard_result(&self) -> u64 {
        (self.get_next_value(0) as u64 + 1) * (self.get_next_value(self.get_next_value(0)) as u64 + 1)
    }
}

impl fmt::Debug for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = Vec::with_capacity(self.state.len());
        list.push(1);
        let mut current_value = self.get_next_value(0);
        while current_value != 0 {
            list.push(current_value + 1);
            current_value = self.get_next_value(current_value);
        }

        f.debug_struct("GameState")
            .field("current_cup", &(self.current_cup + 1))
            .field("cups", &list)
            .finish()
    }
}

fn game_simple(original_data: &[usize]) {
    let mut state = GameState::new(&original_data);

    for i in 0..=100 {
        println!("{}: {:?}", i, state);
        state.mutate_next_step();
    }
}

fn game_hard(original_data: &[usize]) {
    let mut state = GameState::new(&original_data);

    for i in 0..10_000_000 {
        if i % 10_000 == 0 {
            println!("Step {}", i);
        }
        state.mutate_next_step();
    }

    println!("{}", state.get_hard_result());
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let line = lines.next().unwrap().unwrap();
    let original_state: Vec<usize> = line.chars().map(|ch| ch.to_string().parse().unwrap()).collect();
    game_simple(&original_state);

    let mut hard_state = Vec::with_capacity(1_000_000);
    for i in 0..original_state.len() {
        hard_state.push(original_state[i]);
    }

    for i in original_state.len()..1_000_000 {
        hard_state.push(i+1);
    }
    game_hard(&hard_state);
}
