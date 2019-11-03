use core::fmt;
use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;
use std::error::Error;

#[derive(Debug)]
pub struct Pair(u8, u8);

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "({} {})", self.0, self.1)
    }
}


#[derive(Debug)]
pub struct Field {
    pub width: usize,
    pub height: usize,
    cells: Vec<u8>,

}

impl Field {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![0; width * height]
        }
    }

    pub fn place_rect(&mut self, player: u8, pos: Pair, rect: Pair, left_corner: bool) -> Result<(), Box<dyn Error>> {
        if left_corner {
            for i in pos.1 .. pos.1+rect.1 {
                for j in pos.0..pos.0 + rect.0 {
                    self.cells[usize::from(i)*self.width + usize::from(j)] = player;
                }
            }
        } else {
            for i in (pos.1 - rect.1 + 1) .. pos.1 + 1 {
                for j in (pos.0 - rect.0 + 1) .. pos.0 + 1  {
                    self.cells[usize::from(i)*self.width + usize::from(j)] = player;
                }
            }
        }
        Ok(())
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        writeln!(f, "╔{:═<1$}╗", "", self.width+1)?;
        write!(f, "║  ")?;
        for i in 0..self.width {
            write!(f, "{} ", i)?;
        }
        write!(f, "║\n")?;

        for i in 0..self.height {
            write!(f, "║{} ", i)?;
            for j in 0..self.width {
                match self.cells.get(i*self.width + j) {
                    Some(0) => write!(f, "□")?,
                    Some(1) => write!(f, "▣")?,
                    Some(2) => write!(f, "▦")?,
                    Some(_) => write!(f, "?")?,
                    None => panic!("Out of bounds")
                };
            }
            write!(f, "║\n")?;
        }
        writeln!(f, "╚{:═<1$}╝", "", self.width+1)?;
        Ok(())
    }
}

pub struct Player {
    pub name: String
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.name)
    }
}

pub struct Game {
    pub field: Field,
    pub player1: Player,
    pub player2: Player,
    pub winner: Option<u8>,
    current_turn: u8,
    random: ThreadRng,
    dice: Uniform<u8>
}

impl Game {
    pub fn new(player1: Player, player2: Player) -> Self {
        Self {
            field: Field::new(10, 10),
            player1,
            player2,
            winner: None,
            current_turn: 1,
            random: rand::thread_rng(),
            dice: Uniform::new_inclusive(1,6)
        }
    }

    pub fn roll_dice(&mut self) -> u8 {
        self.dice.sample(&mut self.random)
    }

    fn dice_pair(&mut self) -> Pair {
        println!("Rolling dice...");
        Pair(self.roll_dice(), self.roll_dice())
    }

    pub fn run(&mut self) {
        println!("Initial turn");
        let dice = self.dice_pair();
        println!("{} got {}", self.player1, dice);
        self.field.place_rect(1, Pair(0,0), dice, true).expect("Failed to place initial rect");
        let dice = self.dice_pair();
        println!("{} got {}", self.player2, dice);
        self.field.place_rect(2, Pair(9,9), dice, false).expect("Failed to place initial rect");
        println!("{}", self.field);
        panic!("Stop");
        while let None = self.winner {
            self.current_turn = match self.current_turn {
                1 => {
                    println!("{} move", self.player1);
                    2
                }
                2 => {
                    println!("{} move", self.player2);
                    1
                },
                _ => panic!("Turns broken :(")
            }
        }
        match self.winner {
            Some(1) => println!("Player {} Wins!", self.player1),
            Some(2) => println!("Player {} Wins!", self.player2),
            _ => panic!("Winner unknown :(")
        }
    }

}

pub fn create_players() -> Result<(Player, Player), Box<dyn Error>> {
    use dialoguer::Input;

    let name1 = Input::<String>::new().with_prompt("Enter name for Player1").interact()?;

    let name2 = Input::<String>::new().with_prompt("Enter name for Player2").interact()?;

    Ok((Player {name: name1}, Player {name: name2}))
}

#[cfg(test)]
mod tests {
    use crate::Field;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn display_field() {
        let expected = "####\n####\n####\n####\n";
        let field = Field::new(4, 4);
        assert_eq!(expected, format!("{}", field));
    }
}