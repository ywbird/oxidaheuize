#[rustfmt::skip]
pub mod jamo {
    pub const _START: [char; 19] = [
        'ㄱ','ㄲ','ㄴ','ㄷ','ㄸ','ㄹ',
        'ㅁ','ㅂ','ㅃ','ㅅ','ㅆ','ㅇ',
        'ㅈ','ㅉ','ㅊ','ㅋ','ㅌ','ㅍ',
        'ㅎ'
    ];
    pub const _MIDDLE: [char; 21] = [
        'ㅏ','ㅐ','ㅑ','ㅒ','ㅓ','ㅔ',
        'ㅕ','ㅖ','ㅗ','ㅘ','ㅙ','ㅚ',
        'ㅛ','ㅜ','ㅝ','ㅞ','ㅟ','ㅠ',
        'ㅡ','ㅢ','ㅣ'
    ];
    pub const _END: [(char,char); 28] = [
        (' ' , ' '),('ㄱ', ' '),('ㄱ','ㄱ'),('ㄱ','ㅅ'),
        ('ㄴ', ' '),('ㄴ','ㅈ'),('ㄴ','ㅎ'),('ㄷ', ' '),
        ('ㄹ', ' '),('ㄹ','ㄱ'),('ㄹ','ㅁ'),('ㄹ','ㅂ'),
        ('ㄹ','ㅅ'),('ㄹ','ㅌ'),('ㄹ','ㅍ'),('ㄹ','ㅎ'),
        ('ㅁ', ' '),('ㅂ', ' '),('ㅂ','ㅅ'),('ㅅ', ' '),
        ('ㅅ','ㅅ'),('ㅇ', ' '),('ㅈ', ' '),('ㅊ', ' '),
        ('ㅋ', ' '),('ㅌ', ' '),('ㅍ' ,' '),('ㅎ', ' ')
    ];
}
use jamo::*;

pub fn count_lines_in_char(chr: (char, char)) -> i32 {
    match chr {
        ('ㄱ', ' ') | ('ㄴ', ' ') | ('ㅅ', ' ') => 2,
        ('ㄷ', ' ') | ('ㅈ', ' ') | ('ㅋ', ' ') => 3,
        ('ㅁ', ' ')
        | ('ㅂ', ' ')
        | ('ㅊ', ' ')
        | ('ㅌ', ' ')
        | ('ㅍ', ' ')
        | ('ㄱ', 'ㄱ')
        | ('ㄱ', 'ㅅ')
        | ('ㅅ', 'ㅅ') => 4,
        ('ㄹ', ' ') | ('ㄴ', 'ㅈ') | ('ㄴ', 'ㅎ') => 5,
        ('ㅂ', 'ㅅ') => 6,
        ('ㄹ', 'ㄱ') | ('ㄹ', 'ㅅ') => 7,
        ('ㄹ', 'ㅎ') => 8,
        ('ㄹ', 'ㅁ') | ('ㄹ', 'ㅂ') | ('ㄹ', 'ㅌ') | ('ㄹ', 'ㅍ') => 9,
        _ => 0,
    }
}

pub fn get_end_count(chr: (char, char)) -> usize {
    _END.iter().position(|&e| e == chr).unwrap()
}

#[derive(Debug, Copy, Clone)]
pub struct KChar(
    /// 초성(닿소리, 자음)
    pub char,
    /// 중성(홀소리, 모음)
    pub char,
    /// 종성
    pub (char, char),
    /// 원본
    pub char,
);

pub fn disassemble_jamo(chr: char) -> KChar {
    if ((chr as i32) < ('가' as i32)) || ((chr as i32) > ('힣' as i32)) {
        return KChar(' ', ' ', (' ', ' '), chr);
    }

    let num = chr as i32 - '가' as i32;

    let len_mid = _MIDDLE.len() as i32;
    let len_end = _END.len() as i32;

    let end_num = num % len_end;
    let mid_num = (num / len_end) % len_mid;
    let stt_num = (num / len_end) / len_mid;

    KChar(
        _START[stt_num as usize],
        _MIDDLE[mid_num as usize],
        _END[end_num as usize],
        chr,
    )
}

pub fn assemble_jamo(start: char, middle: char, end: (char, char)) -> char {
    // Find indices in the arrays
    let stt_idx = _START.iter().position(|&c| c == start);
    let mid_idx = _MIDDLE.iter().position(|&c| c == middle);
    let end_idx = _END.iter().position(|&end_char| end_char == end);

    // If any component is not found, return a space or error character
    if stt_idx.is_none() || mid_idx.is_none() || end_idx.is_none() {
        return ' ';
    }

    let stt_num = stt_idx.unwrap() as i32;
    let mid_num = mid_idx.unwrap() as i32;
    let end_num = end_idx.unwrap() as i32;

    let len_mid = _MIDDLE.len() as i32;
    let len_end = _END.len() as i32;

    // Reverse the mathematical formula
    let num = stt_num * len_mid * len_end + mid_num * len_end + end_num;

    // Convert back to Korean character
    let result_char = char::from_u32(('가' as u32) + (num as u32));

    result_char.unwrap_or(' ')
}
