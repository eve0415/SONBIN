use crate::generate;

#[derive(Debug, PartialEq)]
pub enum BoardState {
    BINGO(Vec<Vec<usize>>),
    REACH(Vec<Vec<usize>>),
    NONE,
}

pub struct Board {
    /// UserのSnowflake ID + SessionID
    id: u64,
    /// 数字盤のサイズ
    pub size: usize,
    /// 数字盤に書かれている数字
    pub numbers: Vec<Vec<usize>>,
    /// 開けた数字
    pub opened: Vec<usize>,
}

impl Board {
    pub fn new(id: u64, size: usize) -> Self {
        let mut rng = rand::SeedableRng::seed_from_u64(id);
        Self {
            id,
            size,
            numbers: generate::board_numbers(&mut rng, size),
            opened: vec![],
        }
    }

    pub fn open(&mut self, number: usize) -> BoardState {
        // TODO: バリデーション
        self.opened.push(number);

        if let Some(bingo) = self.judge_bingo() {
            return BoardState::BINGO(bingo);
        }

        if let Some(reach) = self.judge_reach() {
            return BoardState::REACH(reach);
        }

        BoardState::NONE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_open() {
        // [
        //   [15, 30, 31, 48, 75],
        //   [11, 21, 45, 59, 69],
        //   [ 7, 26,  0, 51, 70],
        //   [ 3, 23, 37, 46, 62],
        //   [ 4, 19, 33, 55, 63]
        // ]
        let mut board = Board::new(1, 5);
        assert_eq!(board.open(15), BoardState::NONE);
        assert_eq!(board.open(21), BoardState::NONE);
        assert_eq!(
            board.open(63),
            BoardState::REACH(vec![vec![15, 21, 0, 46, 63]])
        );
        assert_eq!(
            board.open(46),
            BoardState::BINGO(vec![vec![15, 21, 0, 46, 63]])
        );
    }
}
