use std::slice::Iter;
use std::iter::StepBy;

type Ean13Result = [u8; 13];
type ModulePattern = [u8; 4];
type ParityPattern = [u8; 6];
const MODULE_PATTERNS: [ModulePattern; 20] = [
    // 奇数
    [3, 2, 1, 1],
    [2, 2, 2, 1],
    [2, 1, 2, 2],
    [1, 4, 1, 1],
    [1, 1, 3, 2],
    [1, 2, 3, 1],
    [1, 1, 1, 4],
    [1, 3, 1, 2],
    [1, 2, 1, 3],
    [3, 1, 1, 2],
    // 偶数
    [1, 1, 2, 3],
    [1, 2, 2, 2],
    [2, 2, 1, 2],
    [1, 1, 4, 1],
    [2, 3, 1, 1],
    [1, 3, 2, 1],
    [4, 1, 1, 1],
    [2, 1, 3, 1],
    [3, 1, 2, 1],
    [2, 1, 1, 3],
];
const PARITY_PATTERNS: [ParityPattern; 10] = [
    [1, 1, 1, 1, 1, 1],
    [1, 1, 0, 1, 0, 0],
    [1, 1, 0, 0, 1, 0],
    [1, 1, 0, 0, 0, 1],
    [1, 0, 1, 1, 0, 0],
    [1, 0, 0, 1, 1, 0],
    [1, 0, 0, 0, 1, 1],
    [1, 0, 1, 0, 1, 0],
    [1, 0, 1, 0, 0, 1],
    [1, 0, 0, 1, 0, 1],
];

#[inline]
pub fn decode(data: &mut StepBy<Iter<u8>>) -> Option<Ean13Result> {
    let mut result: Ean13Result = [0; 13];
    let mut parity: ParityPattern = [0; 6];

    skip_bar(4, data);
    for i in 1..7 {
        let p = detect_pattern(data);
        let d = detect_character(&p);
        if d >= 10 {
            parity[i - 1] = 0;
            result[i] = d - 10;
        } else {
            parity[i - 1] = 1;
            result[i] = d;
        }
    }
    skip_bar(5, data);
    for i in 7..13 {
        let p = detect_pattern(data);
        let d = detect_character(&p);
        result[i] = if d >= 10 { d - 10 } else { d };
    }

    for (i, pp) in PARITY_PATTERNS.iter().enumerate() {
        if *pp == parity {
            result[0] = i as u8;
            break;
        }
    }

    if !valid_check_digit(&result) {
        return None;
    }

    Some(result)
}

// 左・右ガードバーおよびセンターバーはスキップする
#[inline]
fn skip_bar(skip_count: usize, data: &mut StepBy<Iter<u8>>) {
    let mut skip_count = skip_count;
    let mut last_color = match data.next() {
        Some(v) if *v == 255 => *v,
        Some(v) if *v == 0 => *v,
        _ => return,
    };

    loop {
        if skip_count == 0 {
            break;
        }
        let next_color = match data.next() {
            None => break,
            Some(v) => *v,
        };
        if last_color != next_color {
            last_color = next_color;
            skip_count -= 1;
        }
    }
}

// 画像列からモジュールのパターンを見つける
#[inline]
fn detect_pattern(data: &mut StepBy<Iter<u8>>) -> ModulePattern {
    let mut count = 4;
    let mut pattern_code: ModulePattern = [0; 4];

    let mut last_color = match data.next() {
        Some(v) if *v == 255 => *v,
        Some(v) if *v == 0 => *v,
        _ => return pattern_code,
    };
    pattern_code[0] += 1;

    loop {
        if count == 0 {
            break;
        }
        let next_color = match data.next() {
            None => break,
            Some(v) => *v,
        };
        pattern_code[4 - count] += 1;
        if last_color != next_color {
            last_color = next_color;
            count -= 1;
        }
    }
    pattern_code
}

/// 入力されたコードから最もフィットするコードパターンを求める
#[inline]
fn detect_character(input_code: &[u8; 4]) -> u8 {
    let mut best: (u8, f64) = (0, 100.0);

    let modulo = 7; // CodePattern[0].iter().sum()

    for (i, pattern_code) in MODULE_PATTERNS.iter().enumerate() {
        let sum = input_code.iter().sum::<u8>();
        let ratio = sum as f64 / modulo as f64;

        let mut diff = 0.0;
        for (pt, ic) in pattern_code.iter().zip(input_code) {
            let scaled = *pt as f64 * ratio as f64;
            diff += (*ic as f64 - scaled).abs() / scaled;
        }

        if diff < best.1 {
            best.1 = diff;
            best.0 = i as u8;
        }
    }
    best.0
}

fn valid_check_digit(target: &Ean13Result) -> bool {
    let mut sum_even = 0;
    let mut sum_odd = 0;

    for i in 0..12 {
        if i % 2 == 0 {
            sum_even += target[i];
        } else {
            sum_odd += target[i];
        }
    }

    target[12] == (10 - (sum_even + sum_odd * 3) % 10)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::GenericImageView;

    #[test]
    fn test_detect_character() {
        let input = [2, 8, 2, 2];
        let actual = detect_character(&input);
        assert_eq!(3, actual);
    }

    #[test]
    fn test_ean_13() {
        //    1           2           3           4           5           6           7
        //  : 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1,
        // 2: 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1,
        // 3: 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1,
        // 4: 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1,
        // 5: 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1,
        // 6: 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1,
        // 7: 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1,
        //  : 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0,
        // 8: 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // 9: 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0,
        // 0: 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0,
        // 1: 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0,
        // 2: 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0,
        // 8: 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        //  : 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1

        let path = "./tests/fixtures/decoder/ean-jan13.gif";
        let img = image::open(path).unwrap();
        let img_width = img.width();
        let img_height = img.height();

        let (begin, end) = {
            let center = img_height / 2;
            let begin = img_width * 4 * center;
            let end = img_width * 4 * (center + 1);
            (begin as usize, end as usize)
        };
        let img = img.to_bytes();
        let img = img[begin..end].to_vec();
        let mut img = img.iter().step_by(4);

        let actual = decode(&mut img).unwrap();

        assert_eq!([1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 8], actual);
    }
}
