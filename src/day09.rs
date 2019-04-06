use std::collections::VecDeque;

pub fn solve() {
    let player_count: usize = 468;
    let part_1_value = 71843u32;
    let last_marble_value = part_1_value * 100;

    let mut player_scores: Vec<u32> = vec![0; player_count];
    let mut marbles: VecDeque<u32> = VecDeque::new();
    marbles.reserve(last_marble_value as usize);
    marbles.push_back(0);
    marbles.push_back(2);
    marbles.push_back(1);
    // let mut index : usize = 1;
    let mut current_marble = 3 as u32;
    let mut current_player: usize = 1;
    let mut score_marble = 23;
    let mut part_1_score = 0u32;
    while current_marble <= last_marble_value {
        current_player = (current_player + 1) % player_count;

        // if current_marble % 23 == 0 {
        if current_marble == score_marble {
            for _ in 0..7 {
                let tmp = marbles.pop_back().expect("Score marble rotate fail!");
                marbles.push_front(tmp);
            }

            player_scores[current_player] +=
                current_marble + marbles.pop_front().expect("No value in the ring");
            score_marble = score_marble + 23;
        } else {
            for _ in 0..2 {
                let m = marbles.pop_front().expect("Insert marble rotate fail!");
                marbles.push_back(m);
            }
            marbles.push_front(current_marble);
        }

        if current_marble == part_1_value {
            part_1_score = *player_scores.iter().max().unwrap();
        }

        current_marble += 1;
    }

    println!("Day 09 part 1: Max score is: {}", part_1_score);
    println!(
        "Day 09 part 2: Max score is: {}",
        player_scores.iter().max().unwrap()
    );
}
