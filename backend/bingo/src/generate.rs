use rand::rngs::StdRng;
use rand::Rng;

use crate::matrix::Transpose;

/// min以上max以下の数字をランダムに返す
/// 実行する度に違う値が返ります
///
/// # 例
///
/// ```
/// # use bingo::generate::generate_number;
/// let mut rng = rand::SeedableRng::seed_from_u64(0);
/// assert_eq!(generate_number(&mut rng, 1, 75), 59);
/// assert_eq!(generate_number(&mut rng, 1, 75), 17);
/// ```
pub fn generate_number(rng: &mut StdRng, min: usize, max: usize) -> usize {
    rng.gen_range(min..=max)
}

/// sizeで与えられた大きさの数字盤(二次元配列)を返す
/// ```
/// # use bingo::generate::generate_board_numbers;
/// let mut rng = rand::SeedableRng::seed_from_u64(0);
/// assert_eq!(generate_board_numbers(&mut rng, 3), Ok(vec![vec![12, 19, 34], vec![1, 0, 42], vec![9, 27, 45]]));
/// assert_eq!(generate_board_numbers(&mut rng, 2), Err("Board size must be odd".to_string()))
/// ```
pub fn generate_board_numbers(rng: &mut StdRng, size: usize) -> Result<Vec<Vec<usize>>, String> {
    if size % 2 == 0 {
        return Err("Board size must be odd".to_string());
    }
    let mut board: Vec<Vec<usize>> = vec![];
    for row in 0..size {
        let min = row * 15 + 1;
        let max = (row + 1) * 15;
        let mut column = vec![];

        while column.len() < size {
            // フリーマス
            if row == size / 2 && column.len() == size / 2 {
                column.push(0)
            }

            let gen_number = generate_number(rng, min, max);

            if !column.iter().any(|&x| x == gen_number) {
                column.push(gen_number)
            }
        }

        board.push(column)
    }

    Ok(board.into_iter().transpose().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_generate_number() {
        let mut rng = rand::SeedableRng::seed_from_u64(0);
        assert_eq!(generate_number(&mut rng, 1, 75), 59);
        assert_eq!(generate_number(&mut rng, 1, 75), 17);
    }

    #[test]
    fn it_can_generate_board_numbers() {
        let mut rng = rand::SeedableRng::seed_from_u64(0);
        assert_eq!(
            generate_board_numbers(&mut rng, 3).unwrap(),
            vec![[12, 19, 34], [1, 0, 42], [9, 27, 45]]
        );
        assert_eq!(
            generate_board_numbers(&mut rng, 5).unwrap(),
            vec![
                [3, 22, 43, 53, 61],
                [2, 29, 34, 59, 62],
                [15, 19, 0, 60, 70],
                [1, 24, 38, 49, 72],
                [7, 18, 40, 56, 68]
            ]
        );
        assert_eq!(
            generate_board_numbers(&mut rng, 7).unwrap(),
            vec![
                [12, 19, 32, 54, 61, 89, 93],
                [14, 30, 33, 55, 74, 84, 98],
                [2, 26, 40, 46, 67, 77, 92],
                [6, 27, 39, 0, 70, 80, 95],
                [3, 20, 34, 47, 73, 81, 103],
                [8, 25, 35, 50, 72, 87, 104],
                [15, 16, 36, 49, 66, 88, 99]
            ]
        );
        assert_eq!(
            generate_board_numbers(&mut rng, 9).unwrap(),
            vec![
                [12, 29, 44, 58, 63, 90, 98, 114, 123],
                [8, 19, 39, 47, 70, 85, 92, 106, 131],
                [2, 24, 37, 50, 74, 77, 97, 116, 129],
                [15, 23, 34, 46, 61, 86, 99, 107, 135],
                [4, 17, 36, 55, 0, 89, 100, 117, 134],
                [11, 28, 40, 49, 66, 78, 93, 108, 127],
                [3, 20, 45, 51, 72, 88, 91, 119, 133],
                [6, 27, 32, 59, 65, 83, 104, 112, 125],
                [5, 25, 31, 56, 69, 84, 96, 118, 122]
            ]
        );
    }

    #[test]
    fn it_err_generate_board_numbers_when_even_number_given() {
        let mut rng = rand::SeedableRng::seed_from_u64(0);
        assert_eq!(
            generate_board_numbers(&mut rng, 2),
            Err("Board size must be odd".to_string())
        )
    }
}
