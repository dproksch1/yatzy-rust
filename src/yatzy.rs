use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::io;
use std::process::exit;
use regex::Regex;
mod data;

fn main() {

    let mut pdata = data::Data::new();
    let mut dice: Vec<u8> = vec![0,0,0,0,0];
    let mut rng = thread_rng();
    let mut input = String::new();
    let re = Regex::new(r"reroll\s*((?:\d\s*)+)").unwrap();

    'control_loop: for _ in 0..data::FIELD_CNT {
        println!("\nNew round");
        init_roll(&mut dice, &mut rng);
        println!("{:?}", dice);
        let mut completed: bool = false;
        let mut reroll_count = 0;

        while !completed {
            println!("What do you want to do?");
            if let Err(e) = io::stdin().read_line(&mut input) {println!("{}", e)};
            let s = input.as_str();

            if re.is_match(s) {
                let numbers: Vec<u8> = re.captures(s)
                    .and_then(|cap| cap.get(1).map(|digits| {
                        digits.as_str()
                            .split_whitespace()
                            .filter_map(|ss| ss.parse().ok())
                            .collect()
                    })).unwrap_or_default();
                
                check_reroll(&mut dice, &numbers, &mut reroll_count, &mut rng);
            } else {
                match s {
                    "assign ones\n" => {
                            match pdata.add_ones(&dice) {
                                Ok(()) => completed = true,
                                Err(e) => println!("Error: {}", e)
                            }
                        },
                    "assign twos\n" => {
                            match pdata.add_twos(&dice) {
                                Ok(()) => completed = true,
                                Err(e) => println!("Error: {}", e)
                            }
                        },
                    "assign threes\n" => {
                            match pdata.add_threes(&dice) {
                                Ok(()) => completed = true,
                                Err(e) => println!("Error: {}", e)
                            }
                        },
                    "assign fours\n" => {
                            match pdata.add_fours(&dice) {
                                Ok(()) => completed = true,
                                Err(e) => println!("Error: {}", e)
                            }
                        },
                    "assign fives\n" => {
                            match pdata.add_fives(&dice) {
                                Ok(()) => completed = true,
                                Err(e) => println!("Error: {}", e)
                            }
                        },
                    "assign sixes\n" => {
                            match pdata.add_sixes(&dice) {
                                Ok(()) => completed = true,
                                Err(e) => println!("Error: {}", e)
                            }
                        },
                    "assign threeofakind\n" => {
                            match pdata.add_threeofakind(&dice) {
                                Ok(()) => completed = true,
                                Err(e) => println!("Error: {}", e)
                            }
                        },
                    "assign fourofakind\n" => {
                            match pdata.add_fourofakind(&dice) {
                                Ok(()) => completed = true,
                                Err(e) => println!("Error: {}", e)
                            }
                        },
                    "assign smallstraight\n" => {
                            match pdata.add_smallstraight(&dice) {
                                Ok(()) => completed = true,
                                Err(e) => println!("Error: {}", e)
                            }
                        },
                    "assign largestraight\n" => {
                            match pdata.add_largestraight(&dice) {
                                Ok(()) => completed = true,
                                Err(e) => println!("Error: {}", e)
                            }
                        },
                    "assign fullhouse\n" => {
                            match pdata.add_fullhouse(&dice) {
                                Ok(()) => completed = true,
                                Err(e) => println!("Error: {}", e)
                            }
                        },
                    "assign yatzy\n" => {
                            match pdata.add_yatzy(&dice) {
                                Ok(()) => completed = true,
                                Err(e) => println!("Error: {}", e)
                            }
                        },
                    "exit\n" => exit(1),
                    "fin\n" => break 'control_loop,
                    _ => print!("Unknown Command: {}", input)
                }
            }
            input = String::new();
        }
        
    }
    // println!("Score: {}", pdata.iter().sum());

}


fn init_roll(dice: &mut Vec<u8>, rng: &mut ThreadRng) {
    for i in 0..5 {
        dice[i] = rng.gen_range(1..=6);
    }
}

fn check_reroll(dice: &mut Vec<u8>, roll: &Vec<u8>, reroll_count: &mut u8, rng: &mut ThreadRng) {
    if *reroll_count < 2 {
        reroll(dice, roll, rng);
        *reroll_count += 1;
        println!("{:?}", dice);
    } else {
        println!("No reroll available");
    }
}

fn reroll(dice: &mut Vec<u8>, roll: &Vec<u8>, rng: &mut ThreadRng) {
    for i in roll.iter() {
        if *i > 0 && *i < 6 {
            dice[(*i-1) as usize] = rng.gen_range(1..=6);
        }
    }
}

