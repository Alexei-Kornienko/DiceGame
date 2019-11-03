use DiceGame::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Dice Game!");
    let (player1, player2) = create_players()?;
    let mut game = Game::new(player1, player2);
    game.run();
    Ok(())
}
