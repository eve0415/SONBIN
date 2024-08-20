use rand;
use rand::prelude::*;

fn gen_number(min: u64, max: u64, num: &Vec<u64>, seed: Option<u64>) -> u64 {
    let mut rng: rand::rngs::StdRng = match seed {
        Some(x) => rand::SeedableRng::seed_from_u64(x),
        None => rand::SeedableRng::from_entropy(),
    };

    loop {
        let gen_number = rng.gen_range(min..=max);
        if num.iter().find(|&&x| x == gen_number) == None {
            return gen_number;
        }
    }
}

fn gen_bingo_col(col: u64, seed: Option<u64>) -> Vec<u64> {
    let mut arr: Vec<u64> = vec![];

    let min = col * 15 + 1;
    let max = (col + 1) * 15;

    while arr.len() < 5 {
        // 真ん中のマスはフリーマス
        if col == 2 && arr.len() == 2 {
            arr.push(0);
        }

        let gen_number = gen_number(min, max, &arr, seed);
        // まだ出ていない数字だった場合
        if arr.iter().find(|&&x| x == gen_number) == None {
            arr.push(gen_number);
        };
    }

    arr
}

fn gen_bingo(seed: Option<u64>) -> Vec<Vec<u64>> {
    let mut bingo: Vec<Vec<u64>> = vec![];
    for i in 0..5 {
        bingo.push(gen_bingo_col(i, seed))
    }

    bingo
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_gen_number() {
        let expect = 66;
        let actual = gen_number(1, 75, &vec![], Some(1724166657));
        assert_eq!(expect, actual)
    }

    #[test]
    fn it_works_gen_bingo_col() {
        let expect = vec![5, 14, 2, 7, 6];
        let actual = gen_bingo_col(0, Some(1724166657));
        assert_eq!(expect, actual);
    }

    #[test]
    fn it_works_gen_bingo() {
        let expect = vec![
            vec![5, 14, 2, 7, 6],
            vec![20, 29, 17, 22, 21],
            vec![35, 44, 0, 32, 37],
            vec![50, 59, 47, 52, 51],
            vec![65, 74, 62, 67, 66],
        ];
        let actual = gen_bingo(Some(1724166657));
        assert_eq!(expect, actual)
    }
}
