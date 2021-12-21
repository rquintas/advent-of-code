
use std::collections::HashMap;

#[derive(Debug)]
struct Roll {
    value: u8,
    times: u8
}

fn main() {
    // let mut pos: [u8; 2] = [ 4, 8]; // test case
    let mut pos: [u8; 2] = [ 4, 10]; // puzzle input

    let mut score: [u8; 2] = [ 0, 0];

    let mut rolls: Vec<Roll> = vec![];
    rolls.push(Roll{value: 3, times: 1});
    rolls.push(Roll{value: 4, times: 3});
    rolls.push(Roll{value: 5, times: 6});
    rolls.push(Roll{value: 6, times: 7});
    rolls.push(Roll{value: 7, times: 6});
    rolls.push(Roll{value: 8, times: 3});
    rolls.push(Roll{value: 9, times: 1});

    let mut cache: HashMap<[u8; 5],[u128; 2]> = HashMap::new();

    let wins = calculate(&mut cache, &rolls, 0, pos, score);

    println!("{:?}", wins);
}

fn calculate(cache: &mut HashMap<[u8; 5],[u128; 2]>, rolls: &Vec<Roll>, current_player: usize, pos: [u8; 2], score: [u8; 2]) -> [u128; 2] {
    
    let mut wins: [u128; 2] = [0,0];

    // Check if previous player won.
    if score[1-current_player] >= 21 {
        wins[1-current_player] = 1;
        return wins;
    }

    // Check if this branch is already computed.
    let key: [u8; 5] = [current_player as u8, pos[0], pos[1], score[0], score[1]];
    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }

    let mut wins: [u128; 2] = [0,0];


    for roll in rolls {
        // println!("{:?}", roll);
        let mut _pos = pos.clone();
        let mut _score = score.clone();

        _pos[current_player] = ( _pos[current_player] + roll.value ) % 10 ;
        _score[current_player] += if _pos[current_player] == 0 { 10 } else { _pos[current_player] };
        
        let winners: [u128; 2] = calculate(cache, rolls, 1 - current_player, _pos, _score);
        wins[0] += winners[0] * roll.times as u128;
        wins[1] += winners[1] * roll.times as u128;
    }

    // println!("{:?}", wins);

    cache.insert(key, wins);
    return wins;
}

fn part1() {
    
    // let mut pos: [u128; 2] = [ 4, 8]; // test case
    let mut pos: [u128; 2] = [ 4, 10]; // puzzle input
    let mut dierolls: [u128; 2] = [ 0, 0];
    let mut score: [u128; 2] = [ 0, 0];

    let board_size = 10;
    let mut die = 1;
    let mut current_player = 0;

    while ! ( score[0] >= 1000 || score[1] >= 1000 ) {

        for roll in 0..3 {
            pos[current_player] += die;
            pos[current_player] = pos[current_player] % board_size;
            die += 1;
        }
        score[current_player] += if pos[current_player] == 0 { 10 } else { pos[current_player] };
        current_player = 1 - current_player;
    }

    println!("Die: {}", die);
    println!("Die rolls : {}", die -  1);

    if score[0] >= 1000 {
        println!("Player 1 wins the game.");
    }

    if score[1] >= 1000 {
        println!("Player 2 wins the game.");
    }

    println!("P1 Score: {}", score[0]);
    println!("P2 Score: {}", score[1]);
    println!("P1 Die Roll Score: {}", score[0] * (die - 1));
    println!("P2 Die Roll Score: {}", score[1] * (die - 1));

}
