use crossterm::style::Stylize;
use std::collections::VecDeque;
use std::io;

use crate::hangul::{jamo::*, *};

#[derive(Debug, Default)]
pub struct Aheui {
    pub cursor: (i32, i32),
    pub dir: (i32, i32),
    pub prev: (i32, i32),
    pub stacks: [VecDeque<i32>; 27],
    pub result: String,
    pub sel: usize,
    pub src_map: Vec<Vec<KChar>>,
    pub src_mat: (usize, usize),
    pub step: usize,
    pub ended: bool,
    use_debugger: bool,
    verbose: bool,
}

#[derive(Debug)]
pub enum ExitCode {
    Success(i32),
    DivideByZero,
}

impl Aheui {
    pub fn new(src: String) -> Self {
        let max_line = src.lines().map(|l| l.chars().count()).max().unwrap_or(0);

        let src_map: Vec<Vec<KChar>> = src
            .lines()
            .map(|l| {
                let mut line = l.to_string();
                line.push_str(
                    'ㅇ'
                        .to_string()
                        .repeat(max_line - line.chars().count())
                        .as_str(),
                );
                line.chars().map(disassemble_jamo).collect()
            })
            .collect();

        // for c in src_mat.0.iter_mut() {
        //     *c += 1;
        // }
        // for c in src_mat.1.iter_mut() {
        //     *c += 1;
        // }

        Self {
            src_map,
            src_mat: (max_line, src.lines().count()),
            dir: (0, 1),
            verbose: false,
            use_debugger: false,
            ended: false,
            ..Default::default()
        }
    }

    pub fn next(&mut self) {
        if self.ended {
            panic!("Cannot process after ended!");
        }
        self.step += 1;
        let curr = *self.current();

        let mut valid = false;

        if curr.0 != ' ' {
            // 닿소리(자음) 실행
            match curr.0 {
                // ㅇ 묶음
                'ㅇ' | 'ㄱ' | 'ㄲ' => {
                    // PASS
                    valid = true;
                }
                'ㅎ' => {
                    // END
                    let result = self.get_value(self.sel);

                    self.exit(ExitCode::Success(result));

                    return;
                }
                // ㄷ 묶음 - 셈
                'ㄷ' => {
                    // ADDITION
                    if self.check_require(2) {
                        let num1 = self.get_value(self.sel);
                        let num2 = self.get_value(self.sel);

                        self.stacks[self.sel].push_front(num1 + num2);

                        valid = true;
                    }
                }
                'ㄸ' => {
                    // MULTIPLICATION
                    if self.check_require(2) {
                        let num1 = self.get_value(self.sel);
                        let num2 = self.get_value(self.sel);

                        self.stacks[self.sel].push_front(num1 * num2);
                        valid = true;
                    }
                }
                'ㅌ' => {
                    // SUBTRACTION
                    if self.check_require(2) {
                        let num1 = self.get_value(self.sel);
                        let num2 = self.get_value(self.sel);

                        self.stacks[self.sel].push_front(num2 - num1);
                        valid = true;
                    }
                }
                'ㄴ' => {
                    // DIVISION
                    if self.check_require(2) {
                        let num1 = self.get_value(self.sel);
                        let num2 = self.get_value(self.sel);

                        if num2 == 0 {
                            self.exit(ExitCode::DivideByZero);
                            return;
                        }

                        self.stacks[self.sel].push_front(num2 / num1);
                        valid = true;
                    }
                }
                'ㄹ' => {
                    // REMAIN
                    if self.check_require(2) {
                        let num1 = self.get_value(self.sel);
                        let num2 = self.get_value(self.sel);

                        if num2 == 0 {
                            self.exit(ExitCode::DivideByZero);
                            return;
                        }

                        self.stacks[self.sel].push_front(num2 % num1);
                        valid = true;
                    }
                }
                // ㅁ 묶음 - 저장공간
                'ㅁ' => {
                    if self.check_require(1) {
                        let num = self.get_value(self.sel);

                        match curr.2 {
                            ('ㅇ', ' ') => {
                                if self.verbose {
                                    self.print_debug(format!("Print number: {} \n", num));
                                } else {
                                    self.output(num.to_string());
                                }
                                self.result.push_str(num.to_string().as_str());
                            }
                            ('ㅎ', ' ') => {
                                let chr = char::from_u32(num as u32).unwrap_or(' ');
                                if self.verbose {
                                    self.print_debug(format!(
                                        "Print character: {:?}({})",
                                        chr, num
                                    ));
                                } else {
                                    self.output(chr.to_string());
                                }
                                self.result.push(chr);
                            }
                            _ => {}
                        }
                        valid = true;
                    }
                }
                'ㅂ' => {
                    match curr.2 {
                        ('ㅇ', ' ') => {
                            let mut line = String::new();

                            self.request_input(RequestType::Number, &mut line);

                            self.insert_value(
                                self.sel,
                                line.trim() //
                                    .parse() // 1 번째 숫자만
                                    .unwrap_or(0),
                            );
                        }
                        ('ㅎ', ' ') => {
                            let mut line = String::new();

                            self.request_input(RequestType::Number, &mut line);

                            for chr in line.chars() {
                                self.insert_value(self.sel, chr as i32);
                            }
                        }
                        c => {
                            self.insert_value(self.sel, count_lines_in_char(c));
                        }
                    }

                    valid = true;
                }
                'ㅃ' => {
                    // DUPLICATION
                    if self.check_require(1) {
                        let num = self.get_value(self.sel);

                        self.stacks[self.sel].push_front(num);
                        self.stacks[self.sel].push_front(num);
                        valid = true;
                    }
                }
                'ㅍ' => {
                    // SWAP
                    if self.check_require(2) {
                        let num1 = self.get_value(self.sel);
                        let num2 = self.get_value(self.sel);

                        self.stacks[self.sel].push_front(num1);
                        self.stacks[self.sel].push_front(num2);
                        valid = true;
                    }
                }
                // ㅅ 묶음 - 제어
                'ㅅ' => {
                    // SELECT STACK/QUEUE
                    self.sel = get_end_count(curr.2);
                    valid = true;
                }
                'ㅆ' => {
                    // MOVE
                    if self.check_require(1) {
                        let num = self.get_value(self.sel);
                        self.insert_value(get_end_count(curr.2), num);
                        valid = true;
                    }
                }
                'ㅈ' => {
                    // COMPARE
                    if self.check_require(2) {
                        let num1 = self.get_value(self.sel);
                        let num2 = self.get_value(self.sel);

                        if num2 >= num1 {
                            self.insert_value(self.sel, 1);
                        } else {
                            self.insert_value(self.sel, 0);
                        }
                        valid = true;
                    }
                }
                'ㅊ' => {
                    // CONDITION
                    if self.check_require(1) {
                        let num = self.get_value(self.sel);

                        valid = num != 0;
                    }
                }
                _ => {}
            }

            // 방향 재정의 예약 (ㅊ 등,)

            // 홀소리(모음) 실행 (방향)
            self.dir = match curr.1 {
                'ㅏ' => (1, 0),
                'ㅓ' => (-1, 0),
                'ㅗ' => (0, -1),
                'ㅜ' => (0, 1),
                'ㅑ' => (2, 0),
                'ㅕ' => (-2, 0),
                'ㅛ' => (0, -2),
                'ㅠ' => (0, 2),
                'ㅣ' => {
                    if self.dir.1 == 0 {
                        (-self.dir.0, 0)
                    } else {
                        self.dir
                    }
                }
                'ㅡ' => {
                    if self.dir.0 == 0 {
                        (0, -self.dir.1)
                    } else {
                        self.dir
                    }
                }
                'ㅢ' => (-self.dir.0, -self.dir.1),
                _ => self.dir,
            };

            // 방향 재정의 실행
            if !valid {
                self.dir = (-self.dir.0, -self.dir.1);
            }
        }

        if self.verbose {
            self.print_state();
        }

        // 이동
        self.prev = self.cursor;
        self.cursor.0 = (self.cursor.0 + self.dir.0).rem_euclid(self.src_mat.0 as i32);
        self.cursor.1 = (self.cursor.1 + self.dir.1).rem_euclid(self.src_mat.1 as i32);
    }

    fn check_require(&self, count: usize) -> bool {
        self.stacks[self.sel].len() >= count
    }

    fn get_value(&mut self, sel: usize) -> i32 {
        if sel == 21 {
            self.stacks[21].pop_front().unwrap_or(0)
        } else {
            self.stacks[sel].pop_front().unwrap_or(0)
        }
    }

    fn insert_value(&mut self, sel: usize, val: i32) {
        if sel == 21 {
            self.stacks[21].push_back(val);
        } else {
            self.stacks[sel].push_front(val);
        }
    }

    pub fn verbose(&mut self, opt: bool) {
        self.verbose = opt;
    }

    pub fn debug(&mut self, opt: bool) {
        self.use_debugger = opt;
    }

    pub fn exit(&mut self, code: ExitCode) {
        println!("\n");

        if self.verbose {
            print!("Final result: {}", self.result);
        }

        self.ended = true;
        match &code {
            ExitCode::Success(_) if self.verbose => {
                println!("Finished, {:?}", code);
            }
            ExitCode::DivideByZero => {
                println!(
                    "{:?}: divide by 0 at ({}, {})",
                    code, self.cursor.0, self.cursor.1
                );
            }
            _ => {}
        }
        // std::process::exit(code);
    }

    pub fn current(&self) -> &KChar {
        &self.src_map[self.cursor.1 as usize][self.cursor.0 as usize]
    }

    pub fn print_state(&self) {
        println!(
            "\nstep: {}, cursor: ({}, {}), dir: ({}, {})",
            self.step, self.cursor.0, self.cursor.1, self.dir.0, self.dir.1
        );
        for (y, row) in self.src_map.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if (x as i32, y as i32) == self.cursor {
                    print!("{}", format!("{}", cell.3).black().on_dark_blue());
                } else if (x as i32, y as i32) == self.prev {
                    print!("{}", format!("{}", cell.3).white().on_red());
                } else {
                    print!("{}", cell.3);
                }
            }
            println!();
        }

        for (idx, items) in self.stacks.iter().enumerate() {
            if items.len() > 0 {
                println!("{}: {:?}", assemble_jamo('ㅇ', 'ㅏ', _END[idx]), items);
            }
        }
    }

    fn print_debug(&self, text: String) {
        if !self.use_debugger {
            print!("{}", text);
        }
    }

    fn output(&self, text: String) {
        if !self.use_debugger {
            print!("{}", text);
        }
    }

    fn request_input(&mut self, request_type: RequestType, line: &mut String) {
        if self.use_debugger {
        } else {
            io::stdin().read_line(line).unwrap();
        }
    }
}

enum RequestType {
    Number,
    Char,
}
