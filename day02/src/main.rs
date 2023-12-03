use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Game {
    id: u64,
    green: u8,
    red: u8,
    blue: u8
}

fn main() {
    let result: Vec<Game> = get_valid_games(12, 13, 14, false);
    let part_a: u64 = result.iter().map(|g| g.id).sum();
    println!("{:?}", part_a);

    let result: Vec<Game> = get_valid_games(12, 13, 14, true);
    let part_b: u64 = result.iter().map(|g| (g.green as u64) * (g.red as u64) * (g.blue as u64)).sum();
    println!("{:?}", part_b);
}


fn get_valid_games(red: u8, green: u8, blue: u8, check_b: bool) -> Vec<Game> {
    let mut gms: Vec<Game> = Vec::new();
    if let Ok(input) = read_lines("./src/input.txt") {
        for line in input {
            if let Ok(str) = line {
                let game = parse_line(str);
                if !check_b && game.green <= green && game.red <= red && game.blue <= blue {
                    gms.push(game);
                } else if check_b {
                    gms.push(game);
                }
            }
        }
    }
    gms
}

fn parse_line(str: String) -> Game {
    let mut game = Game {id: u64::MIN,green:u8::MIN,red: u8::MIN, blue:u8::MIN };
    let split = str.split([':', ';']);

    for part in split {
        if let Some(_) = part.find("Game") {
            game.id = part[5..].parse::<u64>().unwrap();
        } else {
            let round: Vec<String> = part.split(',').map(|p| p.trim().to_string()).collect();
            for reveal in round {
                let r: Vec<String> = reveal.split(' ').map(|c| c.to_string()).collect();
                let val = r[0].parse::<u8>().unwrap();
                //let col = String::from(r[1]);
                match r[1].as_str() {
                    "green" => {
                        if val > game.green {
                            game.green = val;
                        }
                    },
                    "red" => {
                        if val > game.red {
                            game.red = val;
                        }
                    },
                    "blue" => {
                        if val > game.blue {
                            game.blue = val;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    game
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>where P:AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
