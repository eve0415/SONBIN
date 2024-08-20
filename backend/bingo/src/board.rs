pub trait Board {
    fn row(&self, row: usize) -> &Vec<u64>;
    fn col(&self, col: usize) -> Vec<u64>;
    // 左上-右下
    fn nw_to_se(&self) -> Vec<u64>;
    // 右上-左下
    fn ne_to_sw(&self) -> Vec<u64>;
}

impl Board for Vec<Vec<u64>> {
    fn row(&self, row: usize) -> &Vec<u64> {
        &self[row]
    }

    fn col(&self, col: usize) -> Vec<u64> {
        let mut arr: Vec<u64> = vec![];
        for i in 0..self.len() {
            arr.push(self[i][col])
        }
        arr
    }

    fn nw_to_se(&self) -> Vec<u64> {
        let mut arr: Vec<u64> = vec![];
        for i in 0..self.len() {
            arr.push(self[i][i])
        }
        arr
    }

    fn ne_to_sw(&self) -> Vec<u64> {
        let mut arr: Vec<u64> = vec![];
        for i in 0..self.len() {
            arr.push(self[i][self.len() - i - 1])
        }
        arr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_board_row() {
        let board = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        assert_eq!(&vec![1, 2, 3], board.row(0))
    }

    #[test]
    fn it_works_board_col() {
        let board = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        assert_eq!(vec![1, 4, 7], board.col(0))
    }

    #[test]
    fn it_works_board_nw_to_se() {
        let board = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        assert_eq!(vec![1, 5, 9], board.nw_to_se())
    }

    #[test]
    fn it_works_board_ne_to_sw() {
        let board = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        assert_eq!(vec![3, 5, 7], board.ne_to_sw())
    }
}
