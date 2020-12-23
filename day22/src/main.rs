use std::collections::{HashSet, HashMap, VecDeque};
use std::error::Error;
use std::io::{self, BufRead};

#[derive(Debug)]
struct GameResult {
    winner: usize,
    cards: [VecDeque<usize>; 2],
}

fn game_easy(mut cards: [VecDeque<usize>; 2]) -> GameResult {
    loop {
        let card0 = cards[0].pop_front().unwrap();
        let card1 = cards[1].pop_front().unwrap();

        if card0 < card1 {
            cards[1].push_back(card1);
            cards[1].push_back(card0);
        } else if card0 > card1 {
            cards[0].push_back(card0);
            cards[0].push_back(card1);
        } else {
            panic!("cards are equal");
        }

        if cards[0].len() == 0 {
            return GameResult {
                winner: 1,
                cards,
            }
        }

        if cards[1].len() == 0 {
            return GameResult {
                winner: 0,
                cards,
            }
        }
    }
}

fn game_hard(original_cards: &[VecDeque<usize>; 2]) -> GameResult {
    let mut cards = original_cards.clone();
    let mut states = HashSet::new();

    loop {
        if states.contains(&cards[0]) {
            //println!("loop return from game {}", current_game_count);
            //result_cache.insert(original_cards.clone(), 0);
            return GameResult {
                winner: 0,
                cards,
            };
        }

        states.insert(cards[0].clone());

        let card0 = cards[0].pop_front().unwrap();
        let card1 = cards[1].pop_front().unwrap();

        let round_winner = if cards[0].len() >= card0 && cards[1].len() >= card1 {
            game_hard(&[cards[0].iter().cloned().take(card0).collect(), cards[1].iter().cloned().take(card1).collect()]).winner
        } else if card0 < card1 {
            1
        } else if card0 > card1 {
            0
        } else {
            panic!("cards are equal");
        };

        match round_winner {
            0 => {
                cards[0].push_back(card0);
                cards[0].push_back(card1);
            },
            1 => {
                cards[1].push_back(card1);
                cards[1].push_back(card0);
            },
            _ => panic!("wrong winner value"),
        }

        if cards[0].len() == 0 {
            //println!("winner 1 return from game {}", current_game_count);
            return GameResult {
                winner: 1,
                cards,
            };
        }

        if cards[1].len() == 0 {
            //println!("winner 0 return from game {}", current_game_count);
            return GameResult {
                winner: 0,
                cards,
            };
        }
    }
}

fn handle_result(game_result: GameResult) {
    let mut sum = 0;
    let len = game_result.cards[game_result.winner].len();
    for i in 0..len {
        sum += (len-i) * game_result.cards[game_result.winner][i];
    }
    println!("{:?}", game_result);
    println!("{}", sum);
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut cards: [VecDeque<usize>; 2] = Default::default();
    let mut current_player = usize::MAX;
    for line_result in stdin.lock().lines() {
        let line = line_result?;
        if line.starts_with("Player") {
            current_player = line.strip_prefix("Player ").unwrap().strip_suffix(":").unwrap().parse::<usize>()? - 1;
        } else if line != "" {
            cards[current_player].push_back(line.parse()?);
        }
    }

    handle_result(game_easy(cards.clone()));
    handle_result(game_hard(&cards));

    Ok(())
}
