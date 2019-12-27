use std::convert::TryInto;
use std::str::FromStr;

#[cfg(test)]
mod tests {
    #[test]
    fn test_new_cpu() {
        use super::*;

        let cpu = Cpu::new(vec![1, 2, 3, 4]);
        assert_eq!(cpu.input, vec![]);
        assert_eq!(cpu.mem, vec![1, 2, 3, 4]);
        assert_eq!(cpu.instr_ptr, 0);
    }

    #[test]
    fn test_extract_modes() {
        use super::*;

        assert_eq!(Cpu::extract_modes(1001056), 0b10010);
    }

    #[test]
    fn test_exec() {
        use super::*;

        let prog = parse_prog("1,9,10,3,2,3,11,0,99,30,40,50");
        let mut cpu = Cpu::new(prog);
        assert_eq!(cpu.mem, vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);

        // add 30 + 40 into mem[3]
        let cont = cpu.exec();
        assert_eq!(cpu.mem, vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
        assert_eq!(cont, true);

        // mul 70 + 50 into mem[0]
        let cont = cpu.exec();
        assert_eq!(cpu.mem, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
        assert_eq!(cont, true);

        // end (no mem change)
        let cont = cpu.exec();
        assert_eq!(cpu.mem, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
        assert_eq!(cont, false);

        // add 9 + 19 into mem[3]
        let prog = parse_prog("1101,9,10,3,99");
        let mut cpu = Cpu::new(prog);
        let cont = cpu.exec();
        assert_eq!(cpu.mem, vec![1101, 9, 10, 19, 99]);
        assert_eq!(cont, true);

        // mul 9 * 19 into mem[3]
        let prog = parse_prog("1102,9,10,3,99");
        let mut cpu = Cpu::new(prog);
        let cont = cpu.exec();
        assert_eq!(cpu.mem, vec![1102, 9, 10, 90, 99]);
        assert_eq!(cont, true);

        // read 5 from input and write it out
        let prog = parse_prog("3,0,4,0,99");
        let mut cpu = Cpu::new(prog);
        cpu.add_input(33);
        assert_eq!(cpu.input, vec![33]);
        assert_eq!(cpu.exec(), true);
        assert_eq!(cpu.exec(), true);
        assert_eq!(cpu.input, vec![]);
        assert_eq!(cpu.mem, vec![33, 0, 4, 0, 99]);

        // jmp to IP 0
        let prog = parse_prog("1105,1,0,99");
        let mut cpu = Cpu::new(prog);
        let cont = cpu.exec();
        assert_eq!(cpu.instr_ptr, 0);
        assert_eq!(cont, true);

        // jmp fail to IP 3
        let prog = parse_prog("1105,0,0,99");
        let mut cpu = Cpu::new(prog);
        let cont = cpu.exec();
        assert_eq!(cpu.instr_ptr, 3);
        assert_eq!(cont, true);

        // jmp fail to IP 3
        let prog = parse_prog("1106,1,0,99");
        let mut cpu = Cpu::new(prog);
        let cont = cpu.exec();
        assert_eq!(cpu.instr_ptr, 3);
        assert_eq!(cont, true);

        // jmp to IP 0
        let prog = parse_prog("1106,0,0,99");
        let mut cpu = Cpu::new(prog);
        let cont = cpu.exec();
        assert_eq!(cpu.instr_ptr, 0);
        assert_eq!(cont, true);

        // 5 < 7 == 1
        let prog = parse_prog("1107,5,7,0,99");
        let mut cpu = Cpu::new(prog);
        let cont = cpu.exec();
        assert_eq!(cpu.mem[0], 1);
        assert_eq!(cont, true);

        // 7 < 5 == 0
        let prog = parse_prog("1107,7,5,0,99");
        let mut cpu = Cpu::new(prog);
        let cont = cpu.exec();
        assert_eq!(cpu.mem[0], 0);
        assert_eq!(cont, true);

        // (5 == 5) == 1
        let prog = parse_prog("1108,5,5,0,99");
        let mut cpu = Cpu::new(prog);
        let cont = cpu.exec();
        assert_eq!(cpu.mem[0], 1);
        assert_eq!(cont, true);

        // (7 == 5) == 0
        let prog = parse_prog("1108,7,5,0,99");
        let mut cpu = Cpu::new(prog);
        let cont = cpu.exec();
        assert_eq!(cpu.mem[0], 0);
        assert_eq!(cont, true);
    }

    #[test]
    fn test_parse_prog() {
        use super::*;

        let result = parse_prog("1,1,1,4,99,5,6,0,99");
        assert_eq!(result, vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
     }
}

const ADD_OP: i64 = 1;
const MUL_OP: i64 = 2;
const READ_OP: i64 = 3;
const WRITE_OP: i64 = 4;
const JNZ_OP: i64 = 5;
const JZ_OP: i64 = 6;
const LT_OP: i64 = 7;
const EQ_OP: i64 = 8;
const END_OP: i64 = 99;

pub struct Cpu {
    input: Vec<i64>,
    pub mem: Vec<i64>,
    instr_ptr: usize,
}

impl Cpu {
    pub fn new(mem: Vec<i64>) -> Cpu {
        Cpu {
            input: vec![],
            mem: mem,
            instr_ptr: 0,
        }
    }

    pub fn add_input(&mut self, input: i64) {
        self.input.push(input);
    }

    fn get_param_val(&mut self, modes: u32, param_num: u32) -> i64 {
        let param = self.mem[self.instr_ptr + param_num as usize];
        if (modes & (1 << (param_num - 1))) == 0 {
            self.mem[param as usize]
        } else {
            param
        }
    }

    fn get_dest_param(&self, param_num: u32) -> usize {
        self.mem[self.instr_ptr + param_num as usize] as usize
    }

    fn do_add(&mut self, modes: u32) {
        let param1 = self.get_param_val(modes, 1);
        let param2 = self.get_param_val(modes, 2);
        let dest = self.get_dest_param(3);
        self.mem[dest] = param1 + param2;
    }

    fn do_mul(&mut self, modes: u32) {
        let param1 = self.get_param_val(modes, 1);
        let param2 = self.get_param_val(modes, 2);
        let dest = self.get_dest_param(3);
        self.mem[dest] = param1 * param2;
    }

    fn do_read(&mut self) {
        if self.input.is_empty() {
            return;
        }

        let dest = self.get_dest_param(1);
        self.mem[dest] = self.input.remove(0);
    }

    fn do_write(&mut self, modes: u32) {
        let to_write = self.get_param_val(modes, 1);
        println!("out> {}", to_write);
    }

    fn do_jnz(&mut self, modes: u32) -> bool {
        let param1 = self.get_param_val(modes, 1);
        let param2 = self.get_param_val(modes, 2);
        let do_jmp = param1 != 0;
        if do_jmp {
            self.instr_ptr = param2.try_into().unwrap();
        }
        do_jmp
    }

    fn do_jz(&mut self, modes: u32) -> bool {
        let param1 = self.get_param_val(modes, 1);
        let param2 = self.get_param_val(modes, 2);
        let do_jmp = param1 == 0;
        if do_jmp {
            self.instr_ptr = param2.try_into().unwrap();
        }
        do_jmp
    }

    fn do_lt(&mut self, modes: u32) {
        let param1 = self.get_param_val(modes, 1);
        let param2 = self.get_param_val(modes, 2);
        let dest = self.get_dest_param(3);
        self.mem[dest] = if param1 < param2 { 1 } else { 0 };
    }

    fn do_eq(&mut self, modes: u32) {
        let param1 = self.get_param_val(modes, 1);
        let param2 = self.get_param_val(modes, 2);
        let dest = self.get_dest_param(3);
        self.mem[dest] = if param1 == param2 { 1 } else { 0 };
    }

    fn extract_modes(instr_val: i64) -> u32 {
        let mut instr_val = instr_val / 100;
        let mut modes = 0;
        let mut param_num = 0;
        while instr_val > 0 {
            modes |= (instr_val % 10) << param_num;
            instr_val /= 10;
            param_num += 1;
        }
        modes as u32
    }

    pub fn exec(&mut self) -> bool {
        if self.instr_ptr >= self.mem.len() {
            return false;
        }

        let instr_val = self.mem[self.instr_ptr];
        let op = instr_val % 100;
        let modes = Self::extract_modes(instr_val);

        let mut cont = true;

        let instr_len = match op {
            ADD_OP => {
                self.do_add(modes);
                4
            },
            MUL_OP => {
                self.do_mul(modes);
                4
            },
            READ_OP => {
                self.do_read();
                2
            },
            WRITE_OP => {
                self.do_write(modes);
                2
            },
            JNZ_OP => {
                if self.do_jnz(modes) { 0 } else { 3 }
            },
            JZ_OP => {
                if self.do_jz(modes) { 0 } else { 3 }
            },
            LT_OP => {
                self.do_lt(modes);
                4
            },
            EQ_OP => {
                self.do_eq(modes);
                4
            },
            END_OP => {
                cont = false;
                1
            },
            _ => panic!("bad opcode: {}", op),
        };

        self.instr_ptr += instr_len;
        cont && (self.instr_ptr < self.mem.len())
    }

    pub fn exec_prog(&mut self) {
        self.instr_ptr = 0;
        while self.exec() {}
    }
}

pub fn parse_prog(instr_txt: &str) -> Vec<i64> {
    instr_txt
        .split(",")
        .map(|num_str| i64::from_str(num_str).unwrap())
        .collect::<Vec<i64>>()
}
