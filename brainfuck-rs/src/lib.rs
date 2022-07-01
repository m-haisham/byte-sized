use std::io::stdin;

const TAPE_LENGTH: usize = 30000;

pub struct BrainFuck {
    tape: [u8; TAPE_LENGTH],
    ptr: usize,
    is_looping: bool,
    loop_stack: Vec<usize>,
    inner_loops: usize,
}

impl BrainFuck {
    pub fn new() -> Self {
        Self {
            tape: [0; 30000],
            ptr: 0,
            is_looping: false,
            loop_stack: vec![],
            inner_loops: 0,
        }
    }

    pub fn compile(&mut self, program: &str) {
        let chars = program.chars().collect::<Vec<_>>();

        let mut i = 0;
        while i < program.len() {
            let c = chars[i];

            if self.is_looping {
                if c == ']' {
                    self.inner_loops += 1
                }

                if c == ']' {
                    if self.inner_loops == 0 {
                        self.is_looping = false;
                    } else {
                        self.inner_loops -= 1;
                    }
                }

                continue;
            }

            match c {
                '+' => self.tape[self.ptr] += 1,
                '-' => self.tape[self.ptr] -= 1,
                '>' => self.ptr += 1,
                '<' => self.ptr -= 1,
                '.' => print!("{}", self.tape[self.ptr] as char),
                ',' => {
                    let mut line = String::new();
                    stdin().read_line(&mut line).unwrap();
                    match line.chars().nth(0) {
                        Some(char) => self.tape[self.ptr] = char as u8,
                        None => (),
                    }
                }
                '[' => {
                    if self.tape[self.ptr] == 0 {
                        self.is_looping = true;
                    } else {
                        self.loop_stack.push(i);
                    };
                }
                ']' => {
                    if self.tape[self.ptr] != 0 {
                        i = *self.loop_stack.last().unwrap();
                    } else {
                        self.loop_stack.pop();
                    }
                }
                _ => (),
            }

            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
