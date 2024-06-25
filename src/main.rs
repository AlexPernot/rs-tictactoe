use std::io::Write;
use std::str::FromStr;

type GameState = [[char; 3]; 3];

enum Players {
    Human,
    Computer,
}

fn print_game(game: GameState) -> () {
    println!();
    for row in game {
        let str = format!(" {} | {} | {}", row[0], row[1], row[2]);
        println!("{}", str);
    }
    println!();
}

fn prompt() -> Result<u8, <u8 as FromStr>::Err> {
    let mut input = String::new();
    print!("Entrez votre case (1-9) : ");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Erreur: impossible de lire votre entrée.");

    return input.trim().parse::<u8>();
}

fn play_round(game: &mut GameState, input: u8) -> Result<(), Box<dyn std::error::Error>> {
    let row = ((input - 1) / 3) as usize;
    let col = ((input - 1) % 3) as usize;

    if game[row][col] != ' ' {
        return Err(Box::from("Case déjà jouée"));
    }

    game[row][col] = 'O';

    return Ok(());
}

fn is_game_over(game: GameState) -> Option<Players> {
    // Row check
    for row in 0..=2 {
        if game[row][0] == game[row][1] && game[row][1] == game[row][2] {
            match game[row][0] {
                'O' => return Some(Players::Human),
                'X' => return Some(Players::Computer),
                _ => (),
            }
        }
    }

    // Col check
    for col in 0..=2 {
        if game[0][col] == game[1][col] && game[1][col] == game[2][col] {
            match game[0][col] {
                'O' => return Some(Players::Human),
                'X' => return Some(Players::Computer),
                _ => (),
            }
        }
    }

    // Diagonal check
    if game[0][0] == game[1][1] && game[1][1] == game[2][2] {
        match game[0][0] {
            'O' => return Some(Players::Human),
            'X' => return Some(Players::Computer),
            _ => (),
        }
    }

    if game[2][0] == game[1][1] && game[1][1] == game[0][2] {
        match game[2][0] {
            'O' => return Some(Players::Human),
            'X' => return Some(Players::Computer),
            _ => (),
        }
    }

    return None;
}

fn play_cpu_round(game: &mut GameState) -> () {
    // Dumb AI, can only play in first available spot
    for row in game {
        for col in row {
            if *col == ' ' {
                *col = 'X';
                return;
            }
        }
    }
}

fn main() {
    println!("Tic tac toe");
    println!("Vous jouez les ronds. Le but est de faire une ligne de 3.");
    println!("Les cases sont numérotées comme ceci :");
    println!(" 1 | 2 | 3 ");
    println!(" 4 | 5 | 6 ");
    println!(" 7 | 8 | 9 ");
    println!("À chaque tour, entrez le numéro de la case que vous voulez jouer.");

    let mut game: GameState = [[' '; 3]; 3];

    loop {
        print_game(game);

        match is_game_over(game) {
            Some(Players::Computer) => {
                println!("L'ordinateur a gagné...");
                break;
            }
            Some(Players::Human) => {
                println!("Vous avez gagné !");
                break;
            }
            None => (),
        }

        let Ok(input) = prompt() else {
            println!("Entrez un chiffre entre 1 et 9.");
            continue;
        };

        if input < 1 || input > 9 {
            println!("Entrez un chiffre entre 1 et 9.");
            continue;
        }

        let Ok(_) = play_round(&mut game, input) else {
            println!("Cette case a déjà été jouée.");
            continue;
        };

        match is_game_over(game) {
            Some(Players::Computer) => {
                println!("L'ordinateur a gagné...");
                print_game(game);
                break;
            }
            Some(Players::Human) => {
                println!("Vous avez gagné !");
                print_game(game);
                break;
            }
            None => {
                play_cpu_round(&mut game);
            }
        }
    }
}
