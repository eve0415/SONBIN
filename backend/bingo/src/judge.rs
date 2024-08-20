use crate::board::Board;

fn judge_bingo(board: Vec<Vec<u64>>, numbers: Vec<u64>) -> bool {
    let judge = |x: &u64| *x == 0 || numbers.iter().find(|num| *num == x).is_some();

    // 列
    for i in 0..5 {
        if board.row(i).iter().all(judge) {
            return true;
        }
    }

    // 行
    for i in 0..5 {
        if board.col(i).iter().all(judge) {
            return true;
        }
    }

    // 斜め
    if board.nw_to_se().iter().all(judge) {
        return true;
    }

    if board.ne_to_sw().iter().all(judge) {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case::col(vec![5, 14, 2, 7, 6], true)]
    #[case::col_have_free(vec![35, 44, 37, 36], true)]
    #[case::row(vec![5, 20, 35, 50, 65], true)]
    #[case::nw_to_se(vec![5, 29, 32, 52, 66], true)]
    #[case::ne_to_sw(vec![6, 22, 32, 59, 65], true)]
    #[case::not_bingo(vec![5, 29, 44, 47, 67, 66], false)]
    fn it_works_judge_bingo(#[case] numbers: Vec<u64>, #[case] expect: bool) {
        let board = vec![
            vec![5, 14, 2, 7, 6],
            vec![20, 29, 17, 22, 21],
            vec![35, 44, 0, 37, 36],
            vec![50, 59, 47, 52, 51],
            vec![65, 74, 62, 67, 66],
        ];

        assert_eq!(judge_bingo(board, numbers), expect)
    }
}
