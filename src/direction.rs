/**
 * Enum representing possible movement directions in the game
 */
#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Up, 
    Down,
    Left,
    Right,
}

impl Direction {
    /**
     * Method to get the opposite direction (used for preventing opposite turns)
     * 
     * Returns:
     * - Direction: The opposite direction
     */
    pub fn opposite(&self) -> Self {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
