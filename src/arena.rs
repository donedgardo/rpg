use crate::player::Player;

pub struct Arena<'a> {
    players: Vec<&'a Player>,
}

impl<'a> Arena<'a> {
    pub fn new() -> Self {
        Self {
            players: vec![]
        }
    }
    pub fn add_player(&mut self, player: &'a Player) {
        self.players.push(player)
    }
    pub fn add_players(&mut self, players: Vec<&'a Player>) {
        for player in players {
            self.players.push(player)
        }
    }
    pub fn is_empty(&self) -> bool {
        self.players.is_empty()
    }
    pub fn player_count(&self) -> usize {
        self.players.len()
    }
}

// Weapons
// Concave Mirrrors Concentrate energy focus fire
// Alchemy study of light.
// Telescopes.
// Time Travel
#[cfg(test)]
mod arena_tests {
    use crate::arena::Arena;
    use crate::player::Player;

    #[test]
    fn can_be_created() {
        let arena = Arena::new();
        assert!(arena.is_empty());
    }

    #[test]
    fn player_can_join() {
        let mut arena = Arena::new();
        let player = Player::new();
        arena.add_player(&player);
        assert!(!arena.is_empty());
    }

    #[test]
    fn multiple_players_can_join() {
        let mut arena = Arena::new();
        let player1 = Player::new();
        let player2 = Player::new();
        arena.add_players(vec![&player1, &player2]);
        assert_eq!(arena.player_count(), 2);
    }
}
