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
            5 => println!("jit({}{}, {}{})",
                          chars[m1 as usize], self.program[position+1],
                          chars[m2 as usize], self.program[position+2],
            ),
            6 => println!("jif({}{}, {}{})",
                          chars[m1 as usize], self.program[position+1],
                          chars[m2 as usize], self.program[position+2],
            ),
            7 => println!("{}{} < {}{} => {}{}",
                          chars[m1 as usize], self.program[position+1],
                          chars[m2 as usize], self.program[position+2],
                          chars[m3 as usize], self.program[position+3],
            ),
            8 => println!("{}{} == {}{} => {}{}",
                          chars[m1 as usize], self.program[position+1],
                          chars[m2 as usize], self.program[position+2],
                          chars[m3 as usize], self.program[position+3],
            ),
            99 => println!("exit"),
            _ => panic!("unknown opcode {}", opcode),
        }
    }

    pub fn run(&mut self) {
        while !self.step() {}
    }

    pub fn quick_run(&mut self, input: &[i32]) -> i32 {
        self.reset();
        for v in input {
            self.push_input(*v);
        }
        self.run();

        *self.output.last().unwrap()
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
            }
            2 => {
                self.program_pos += 4;
                self.write(
                    position + 3, m3,
                    self.read(position + 1, m1) * self.read(position + 2, m2)
                );

                false
            }
            3 => {
                self.program_pos += 2;
                self.write(
                    position + 1, m3,
                    self.input[self.input_pos],
                );
                self.input_pos += 1;

                false
            }
            4 => {
                self.program_pos += 2;
                self.output.push(self.read(position + 1, m1));

                false
            }
            5 => {
                if self.read(position + 1, m1) != 0 {
                    self.program_pos = self.read(position + 2, m2) as usize;
                } else {
                    self.program_pos += 3;
                }

                false
            }
            6 => {
                if self.read(position + 1, m1) == 0 {
                    self.program_pos = self.read(position + 2, m2) as usize;
                } else {
                    self.program_pos += 3;
                }

                false
            }
            7 => {
                self.program_pos += 4;
                self.write(position + 3, m3,
                    (self.read(position + 1, m1) < self.read(position + 2, m2)) as i32,
                );

                false
            }
            8 => {
                self.program_pos += 4;
                self.write(position + 3, m3,
                    (self.read(position + 1, m1) == self.read(position + 2, m2)) as i32,
                );

                false
            }
            99 => {
                true
            }
            _ => panic!("Unknown opcode {}", opcode)
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
    fn test_part2() {
        let mut vm1 = VM::parse("3,9,8,9,10,9,4,9,99,-1,8");
        assert_eq!(vm1.quick_run(&[8]), 1);
        assert_eq!(vm1.quick_run(&[7]), 0);
        assert_eq!(vm1.quick_run(&[9]), 0);

        let mut vm2 = VM::parse("3,9,7,9,10,9,4,9,99,-1,8");
        assert_eq!(vm2.quick_run(&[8]), 0);
        assert_eq!(vm2.quick_run(&[7]), 1);
        assert_eq!(vm2.quick_run(&[9]), 0);

        let mut vm3 = VM::parse("3,3,1108,-1,8,3,4,3,99");
        assert_eq!(vm3.quick_run(&[8]), 1);
        assert_eq!(vm3.quick_run(&[7]), 0);
        assert_eq!(vm3.quick_run(&[9]), 0);

        let mut vm4 = VM::parse("3,3,1107,-1,8,3,4,3,99");
        assert_eq!(vm4.quick_run(&[8]), 0);
        assert_eq!(vm4.quick_run(&[7]), 1);
        assert_eq!(vm4.quick_run(&[9]), 0);

        let mut vm5 = VM::parse("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
        assert_eq!(vm5.quick_run(&[0]), 0);
        assert_eq!(vm5.quick_run(&[-1]), 1);
        assert_eq!(vm5.quick_run(&[1]), 1);

        let mut vm6 = VM::parse("3,3,1105,-1,9,1101,0,0,12,4,12,99,1");
        assert_eq!(vm6.quick_run(&[0]), 0);
        assert_eq!(vm6.quick_run(&[-1]), 1);
        assert_eq!(vm6.quick_run(&[1]), 1);

        let mut vm7 = VM::parse("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
        assert_eq!(vm7.quick_run(&[-4]), 999);
        assert_eq!(vm7.quick_run(&[8]), 1000);
        assert_eq!(vm7.quick_run(&[14]), 1001);
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