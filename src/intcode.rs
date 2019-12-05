#[derive(Clone)]
pub struct VM {
    initial_program: Vec<i32>,
    program: Vec<i32>,
    program_pos: usize,
    input: Vec<i32>,
    input_pos: usize,
    output: Vec<i32>,
}

impl VM {
    pub fn reset(&mut self) {
        self.program.copy_from_slice(&self.initial_program);
        self.program_pos = 0;
        self.input.clear();
        self.input_pos = 0;
        self.output.clear();
    }

    pub fn push_input(&mut self, v: i32) {
        self.input.push(v);
    }

    pub fn output(&self) -> &[i32] {
        &self.output
    }

    fn read(&self, addr: usize, mode: i32) -> i32 {
        match mode {
            0 => self.program[self.program[addr] as usize],
            1 => self.program[addr] as i32,
            _ => unreachable!(),
        }
    }

    fn write(&mut self, addr: usize, mode: i32, v: i32) {
        let addr = match mode {
            0 => self.program[addr] as usize,
            _ => panic!("mode {} not supported for write", mode),
        };

        self.program[addr] = v;
    }

    pub fn print_next(&self) {
        let position = self.program_pos;
        let (opcode, m1, m2, m3) = parse_opcode(self.program[position]);

        let chars = ['&', '='];

        match opcode {
            1 => println!("{}{} + {}{} => {}{}",
                          chars[m1 as usize], self.program[position+1],
                          chars[m2 as usize], self.program[position+2],
                          chars[m3 as usize], self.program[position+3],
            ),
            2 => println!("{}{} * {}{} => {}{}",
                          chars[m1 as usize], self.program[position+1],
                          chars[m2 as usize], self.program[position+2],
                          chars[m3 as usize], self.program[position+3],
            ),
            3 => println!("read({}{})",
                          chars[m1 as usize], self.program[position+1],
            ),
            4 => println!("write({}{})",
                          chars[m1 as usize], self.program[position+1],
            ),
            99 => println!("exit"),
            _ => panic!("unknown opcode {}", opcode),
        }
    }

    pub fn run(&mut self) {
        while !self.step() {}
    }

    pub fn step(&mut self) -> bool {
        let position = self.program_pos;
        let (opcode, m1, m2, m3) = parse_opcode(self.program[position]);

        match opcode {
            1 => {
                self.program_pos += 4;
                self.write(
                    position + 3, m3,
                    self.read(position + 1, m1) + self.read(position + 2, m2)
                );

                false
            },
            2 => {
                self.program_pos += 4;
                self.write(
                    position + 3, m3,
                    self.read(position + 1, m1) * self.read(position + 2, m2)
                );

                false
            },
            3 => {
                self.program_pos += 2;
                self.write(
                    position + 1, m3,
                    self.input[self.input_pos],
                );
                self.input_pos += 1;

                false
            },
            4 => {
                self.program_pos += 2;
                self.output.push(self.read(position + 1, m1));

                false
            },
            5 => {

                false
            }
            6 => {

                false
            }
            7 => {


                false
            }
            8 => {

                false
            }
            99 => {
                true
            },
            _ => panic!("Unknown opcode {}", opcode),
        }
    }

    pub fn new(initial_program: &[i32]) -> VM {
        VM{
            initial_program: initial_program.to_vec(),
            program: initial_program.to_vec(),
            program_pos: 0,
            input: Vec::with_capacity(16),
            input_pos: 0,
            output: Vec::with_capacity(16),
        }
    }

    pub fn parse(program_data: &str) -> VM {
        let data: Vec<i32> = program_data.split(',').map(|t| t.parse::<i32>().unwrap()).collect();
        Self::new(&data)
    }
}

pub fn parse_opcode(code: i32) -> (i32, i32, i32, i32) {
    (code % 100, ((code / 100) % 10), ((code / 1000) % 10), ((code / 10000) % 10))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm() {
        let mut vm = VM::parse("103,13,1001,13,5,13,11002,13,14,14,4,14,99,5,5");
        vm.reset();

        vm.push_input(5);

        assert_eq!(vm.step(), false);
        assert_eq!(vm.step(), false);
        assert_eq!(vm.step(), false);
        assert_eq!(vm.step(), false);
        assert_eq!(vm.step(), true);

        assert_eq!(vm.output().len(), 1);
        assert_eq!(vm.output()[0], 140);
    }

    #[test]
    fn test_parse_opcode() {
        assert_eq!(parse_opcode(99), (99, 0, 0, 0));
        assert_eq!(parse_opcode(199), (99, 1, 0, 0));
        assert_eq!(parse_opcode(10102), (2, 1, 0, 1));
        assert_eq!(parse_opcode(1102), (2, 1, 1, 0));
        assert_eq!(parse_opcode(1003), (3, 0, 1, 0));
        assert_eq!(parse_opcode(10004), (4, 0, 0, 1));
    }
}