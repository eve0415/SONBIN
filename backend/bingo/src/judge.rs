use crate::{board::Board, matrix::Matrix};

impl Board {
    /// ビンゴかどうか判定する
    ///
    /// # 例
    /// ```
    /// # use bingo::board::Board;
    /// let mut board = Board::new(1, 3);
    /// board.numbers = vec![
    ///   vec![15, 18, 45],
    ///   vec![11, 0, 36],
    ///   vec![7, 19, 41],
    /// ];
    /// board.opened = vec![15, 18, 45];
    /// assert_eq!(board.judge_bingo(), Some(vec![vec![15, 18, 45]]));
    /// ```
    pub fn judge_bingo(&mut self) -> Option<Vec<Vec<usize>>> {
        let mut bingo = vec![];

        for i in 0..self.size {
            if self.opened_count_in_vec(self.numbers.row(i)) == self.size {
                bingo.push(self.numbers.row(i));
            }
        }

        for i in 0..self.size {
            if self.opened_count_in_vec(self.numbers.col(i)) == self.size {
                bingo.push(self.numbers.col(i));
            }
        }

        if self.opened_count_in_vec(self.numbers.diagnoal_from_upper_left()) == self.size {
            bingo.push(self.numbers.diagnoal_from_upper_left());
        }

        if self.opened_count_in_vec(self.numbers.diagnoal_from_upper_right()) == self.size {
            bingo.push(self.numbers.diagnoal_from_upper_right());
        }

        match bingo.len() {
            1.. => Some(bingo),
            _ => None,
        }
    }

    /// リーチかどうか判定する
    ///
    /// # 例
    /// ```
    /// # use bingo::board::Board;
    /// let mut board = Board::new(1, 3);
    /// board.numbers = vec![
    ///   vec![15, 18, 45],
    ///   vec![11, 0, 36],
    ///   vec![7, 19, 41],
    /// ];
    /// board.opened = vec![15, 45];
    /// assert_eq!(board.judge_reach(), Some(vec![vec![15, 18, 45], vec![15, 0, 41], vec![45, 0, 7]]));
    /// ```
    pub fn judge_reach(&mut self) -> Option<Vec<Vec<usize>>> {
        let mut reach = vec![];

        for i in 0..self.size {
            if self.opened_count_in_vec(self.numbers.row(i)) == self.size - 1 {
                reach.push(self.numbers.row(i))
            }
        }

        for i in 0..self.size {
            if self.opened_count_in_vec(self.numbers.col(i)) == self.size - 1 {
                reach.push(self.numbers.col(i));
            }
        }

        if self.opened_count_in_vec(self.numbers.diagnoal_from_upper_left()) == self.size - 1 {
            reach.push(self.numbers.diagnoal_from_upper_left());
        }

        if self.opened_count_in_vec(self.numbers.diagnoal_from_upper_right()) == self.size - 1 {
            reach.push(self.numbers.diagnoal_from_upper_right());
        }

        match reach.len() {
            1.. => Some(reach),
            _ => None,
        }
    }

    /// vecで与えられた数字が[Board].openedに何個存在するか返す
    ///
    /// # 例
    ///
    /// ```
    /// # use bingo::board::Board;
    /// let mut board = Board::new(1, 5);
    /// board.opened = vec![2, 4];
    /// assert_eq!(board.opened_count_in_vec(vec![2, 4, 5]), 2)
    /// ```
    pub fn opened_count_in_vec(&self, vec: Vec<usize>) -> usize {
        vec.iter()
            .filter(|&x| *x == 0 || self.opened.contains(x))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_opened_count_in_vec() {
        let mut board = Board::new(1, 5);
        board.open(2);
        board.open(4);
        assert_eq!(board.opened_count_in_vec(vec![2, 4, 5]), 2);
    }

    #[test]
    fn works_judge_reach() {
        // [
        //   [15, 18, 45],
        //   [11, 0, 36],
        //   [7, 19, 41],
        // ]
        let mut board = Board::new(1, 3);
        board.open(15);
        board.open(45);

        // X = 開けたマス
        // [
        //   [X, 18,  X],
        //   [11, X, 36],
        //   [7, 19, 41],
        // ]
        assert_eq!(
            board.judge_reach(),
            Some(vec![vec![15, 18, 45], vec![15, 0, 41], vec![45, 0, 7]])
        );

        board.open(11);

        // X = 開けたマス
        // [
        //   [X, 18,  X],
        //   [X, X, 36],
        //   [7, 19, 41],
        // ]
        println!("{:?}", board.opened);
        assert_eq!(
            board.judge_reach(),
            Some(vec![
                vec![15, 18, 45],
                vec![11, 0, 36],
                vec![15, 11, 7],
                vec![15, 0, 41],
                vec![45, 0, 7]
            ])
        )
    }

    #[test]
    fn works_judge_bingo_row() {
        // [
        //   [15, 18, 45],
        //   [11, 0, 36],
        //   [7, 19, 41],
        // ]
        let mut board = Board::new(1, 3);
        board.open(15);
        board.open(45);
        board.open(18);
        assert_eq!(board.judge_bingo(), Some(vec![vec![15, 18, 45]]));
    }

    #[test]
    fn works_judge_bingo_col() {
        let mut board = Board::new(1, 3);
        board.open(15);
        board.open(11);
        board.open(7);
        assert_eq!(board.judge_bingo(), Some(vec![vec![15, 11, 7]]));
    }

    #[test]
    fn works_judge_bingo_diagnoal_from_upper_left() {
        let mut board = Board::new(1, 3);
        board.open(15);
        board.open(41);
        assert_eq!(board.judge_bingo(), Some(vec![vec![15, 0, 41]]));
    }

    #[test]
    fn works_judge_bingo_diagnoal_from_upper_right() {
        let mut board = Board::new(1, 3);
        board.open(45);
        board.open(7);
        assert_eq!(board.judge_bingo(), Some(vec![vec![45, 0, 7]]));
    }
}
