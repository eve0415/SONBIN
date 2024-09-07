pub trait Transpose<Iter: IntoIterator> {
    fn transpose(self) -> Transposed<Iter>;
}

impl<T> Transpose<T::Item> for T
where
    T: IntoIterator,
    T::Item: IntoIterator,
{
    /// 二次元配列を転置する
    /// a\[i\]\[j\] => a\[j\]\[i\]
    /// a\[j\]\[i\] => a\[i\]\[j\] (i != j)
    ///
    /// # 例
    /// ```
    /// # use board::matrix::Transpose;
    /// let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    /// assert_eq!(
    ///     matrix.into_iter().transpose().collect::<Vec<Vec<i32>>>(),
    ///     vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]
    /// )
    /// ```
    fn transpose(self) -> Transposed<T::Item> {
        Transposed(self.into_iter().map(IntoIterator::into_iter).collect())
    }
}

pub struct Transposed<Iter: IntoIterator>(Vec<Iter::IntoIter>);

impl<Iter: IntoIterator> Iterator for Transposed<Iter> {
    type Item = Vec<Iter::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.iter_mut().map(Iterator::next).collect()
    }
}

pub trait Matrix {
    fn row(&self, row: usize) -> Vec<usize>;
    fn col(&self, col: usize) -> Vec<usize>;
    /// 左上から右下への対角成分
    ///
    /// # 例
    ///
    /// ```
    /// # use board::matrix::Matrix;
    /// let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    /// assert_eq!(matrix.diagnoal_from_upper_left(), vec![1, 5, 9]);
    /// ```
    fn diagnoal_from_upper_left(&self) -> Vec<usize>;

    /// 右上から左下への対角成分
    ///
    /// # 例
    ///
    /// ```
    /// # use board::matrix::Matrix;
    /// let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    /// assert_eq!(matrix.diagnoal_from_upper_right(), vec![3, 5, 7]);
    /// ```
    fn diagnoal_from_upper_right(&self) -> Vec<usize>;
}

impl Matrix for Vec<Vec<usize>> {
    fn row(&self, row: usize) -> Vec<usize> {
        self.iter().enumerate().map(|(i, _)| self[row][i]).collect()
    }

    fn col(&self, col: usize) -> Vec<usize> {
        self.iter().enumerate().map(|(i, _)| self[i][col]).collect()
    }

    fn diagnoal_from_upper_left(&self) -> Vec<usize> {
        self.iter().enumerate().map(|(i, _)| self[i][i]).collect()
    }

    fn diagnoal_from_upper_right(&self) -> Vec<usize> {
        self.iter()
            .enumerate()
            .map(|(i, _)| self[i][self.len() - i - 1])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_transpose() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        assert_eq!(
            matrix.into_iter().transpose().collect::<Vec<Vec<i32>>>(),
            vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]
        )
    }

    #[test]
    fn it_works_matrix_row() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(matrix.row(0), vec![1, 2, 3]);
        assert_eq!(matrix.row(1), vec![4, 5, 6]);
        assert_eq!(matrix.row(2), vec![7, 8, 9]);
    }

    #[test]
    fn it_works_matrix_col() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(matrix.col(0), vec![1, 4, 7]);
        assert_eq!(matrix.col(1), vec![2, 5, 8]);
        assert_eq!(matrix.col(2), vec![3, 6, 9]);
    }

    #[test]
    fn it_works_matrix_diagnoal_from_upper_left() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(matrix.diagnoal_from_upper_left(), vec![1, 5, 9]);
    }

    #[test]
    fn it_works_matrix_diagnoal_from_upper_right() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(matrix.diagnoal_from_upper_right(), vec![3, 5, 7]);
    }
}
