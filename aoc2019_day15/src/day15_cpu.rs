use std::convert::TryInto;
use std::str::FromStr;

const ADD_OP: i64 = 1;
const MUL_OP: i64 = 2;
const READ_OP: i64 = 3;
const WRITE_OP: i64 = 4;
const JNZ_OP: i64 = 5;
const JZ_OP: i64 = 6;
const LT_OP: i64 = 7;
const EQ_OP: i64 = 8;
const ADJ_REL_BASE_OP: i64 = 9;
const END_OP: i64 = 99;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CpuState {
    Running,
    Done,
    WaitOnInput,
}

pub struct Cpu {
    input: Vec<i64>,
    mem: Vec<i64>,
    instr_ptr: usize,
    print_output: bool,
    output: Vec<i64>,
    state: CpuState,
    relative_base: i64,
}

enum ParamMode {
    Register,
    Immediate,
    Relative,
}

impl Cpu {
    const MEM_SIZE: usize = 1024 * 1024;

    pub fn new(prog: &Vec<i64>) -> Cpu {
        let mut mem = vec![0; Self::MEM_SIZE];
        mem[..prog.len()].copy_from_slice(&prog);
        Cpu {
            input: vec![],
            mem: mem,
            instr_ptr: 0,
            print_output: true,
            output: vec![],
            state: CpuState::Running,
            relative_base: 0,
        }
    }

    pub fn get_mem(&self) -> &Vec<i64> {
        &self.mem
    }

    pub fn get_state(&self) -> CpuState {
        self.state
    }

    pub fn reset(&mut self) {
        self.input.clear();
        self.instr_ptr = 0;
        self.output.clear();
        self.state = CpuState::Running;
        self.relative_base = 0;
    }

    pub fn set_prog(&mut self, mem: &Vec<i64>) {
        self.reset();
        self.mem = mem.clone();
    }

    pub fn pop_output(&mut self) -> Option<i64> {
        if self.output.is_empty() {
            None
        } else {
            Some(self.output.remove(0))
        }
    }

    pub fn has_output(&mut self) -> bool {
        !self.output.is_empty()
    }

    pub fn add_input(&mut self, input: i64) {
        self.input.push(input);
    }

    pub fn set_print_output(&mut self, print: bool) {
        self.print_output = print;
    }

    fn get_param_mode(modes: u32, param_num: u32) -> ParamMode {
        let mode = (modes >> ((param_num - 1) * 2)) & 0b11;
        match mode {
            0 => ParamMode::Register,
            1 => ParamMode::Immediate,
            2 => ParamMode::Relative,
            _ => panic!("bad param mode!"),
        }
    }

    fn get_param_val(&self, modes: u32, param_num: u32) -> i64 {
        let param = self.mem[self.instr_ptr + param_num as usize];
        match Self::get_param_mode(modes, param_num) {
            ParamMode::Register => self.mem[param as usize],
            ParamMode::Immediate => param,
            ParamMode::Relative => self.mem[(param + self.relative_base) as usize],
        }
    }

    fn get_dest_loc(&self, modes: u32, param_num: u32) -> usize {
        let param = self.mem[self.instr_ptr + param_num as usize];
        match Self::get_param_mode(modes, param_num) {
            ParamMode::Register => param as usize,
            ParamMode::Relative => (param + self.relative_base) as usize,
            _ => panic!("bad param mode in get_dest_loc"),
        }
     }

    fn do_add(&mut self, modes: u32) {
        let param1 = self.get_param_val(modes, 1);
        let param2 = self.get_param_val(modes, 2);
        let dest = self.get_dest_loc(modes, 3);
        self.mem[dest] = param1 + param2;
    }

    fn do_mul(&mut self, modes: u32) {
        let param1 = self.get_param_val(modes, 1);
        let param2 = self.get_param_val(modes, 2);
        let dest = self.get_dest_loc(modes, 3);
        self.mem[dest] = param1 * param2;
    }

    fn do_read(&mut self, modes: u32) -> bool {
        if self.input.is_empty() {
            return false;
        }

        let dest = self.get_dest_loc(modes, 1);
        self.mem[dest as usize] = self.input.remove(0);
        true
    }

    fn do_write(&mut self, modes: u32) {
        self.output.push(self.get_param_val(modes, 1));
        if self.print_output {
            println!("out> {}", self.output.last().unwrap());
        }
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
        let dest = self.get_dest_loc(modes, 3);
        self.mem[dest] = if param1 < param2 { 1 } else { 0 };
    }

    fn do_eq(&mut self, modes: u32) {
        let param1 = self.get_param_val(modes, 1);
        let param2 = self.get_param_val(modes, 2);
        let dest = self.get_dest_loc(modes, 3);
        self.mem[dest] = if param1 == param2 { 1 } else { 0 };
    }

    fn do_adj_rel_base(&mut self, modes: u32) {
        let param1 = self.get_param_val(modes, 1);
        self.relative_base += param1;
    }

    fn extract_modes(instr_val: i64) -> u32 {
        let mut instr_val = instr_val / 100;
        let mut modes = 0;
        let mut param_num = 0;
        while instr_val > 0 {
            modes |= (instr_val % 10) << (param_num * 2);
            instr_val /= 10;
            param_num += 1;
        }
        modes as u32
    }

    pub fn exec(&mut self) -> CpuState {
        if self.state == CpuState::Done
            || self.instr_ptr >= self.mem.len()
        {
            self.state = CpuState::Done;
            return self.state;
        }

        self.state = CpuState::Running;

        let instr_val = self.mem[self.instr_ptr];
        let op = instr_val % 100;
        let modes = Self::extract_modes(instr_val);

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
                if self.do_read(modes) {
                    2
                } else {
                    self.state = CpuState::WaitOnInput;
                    0
                }
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
            ADJ_REL_BASE_OP => {
                self.do_adj_rel_base(modes);
                2
            },
            END_OP => {
                self.state = CpuState::Done;
                1
            },
            _ => panic!("bad opcode: {}", op),
        };

        self.instr_ptr += instr_len;
        if self.instr_ptr >= self.mem.len() {
            self.state = CpuState::Done;
        }

        self.state
    }

    pub fn exec_prog(&mut self) {
        while self.exec() == CpuState::Running {}
    }
}

pub fn parse_prog(instr_txt: &str) -> Vec<i64> {
    instr_txt
        .split(",")
        .map(|num_str| i64::from_str(num_str).unwrap())
        .collect::<Vec<i64>>()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_new_cpu() {
        use super::*;

        let cpu = Cpu::new(&vec![1, 2, 3, 4]);
        let target = vec![1, 2, 3, 4];
        assert_eq!(cpu.input, vec![]);
        assert_eq!(cpu.mem[..target.len()], target[..]);
        assert_eq!(cpu.instr_ptr, 0);
    }

    #[test]
    fn test_extract_modes() {
        use super::*;

        assert_eq!(Cpu::extract_modes(1001056), 0b100000100);
    }

    #[test]
    fn test_exec() {
        use super::*;

        let prog = parse_prog("1,9,10,3,2,3,11,0,99,30,40,50");
        let mut cpu = Cpu::new(&prog);
        let target = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        assert_eq!(cpu.mem[..target.len()], target[..]);

        // add 30 + 40 into mem[3]
        let cont = cpu.exec();
        let target = vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
        assert_eq!(cpu.mem[..target.len()], target[..]);
        assert_eq!(cont, CpuState::Running);

        // mul 70 + 50 into mem[0]
        let cont = cpu.exec();
        let target = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
        assert_eq!(cpu.mem[..target.len()], target[..]);
        assert_eq!(cont, CpuState::Running);

        // end (no mem change)
        let cont = cpu.exec();
        let target = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
        assert_eq!(cpu.mem[..target.len()], target[..]);
        assert_eq!(cont, CpuState::Done);

        // add 9 + 19 into mem[3]
        let prog = parse_prog("1101,9,10,3,99");
        let mut cpu = Cpu::new(&prog);
        let cont = cpu.exec();
        let target = vec![1101, 9, 10, 19, 99];
        assert_eq!(cpu.mem[..target.len()], target[..]);
        assert_eq!(cont, CpuState::Running);

        // mul 9 * 19 into mem[3]
        let prog = parse_prog("1102,9,10,3,99");
        let mut cpu = Cpu::new(&prog);
        let cont = cpu.exec();
        let target = vec![1102, 9, 10, 90, 99];
        assert_eq!(cpu.mem[..target.len()], target[..]);
        assert_eq!(cont, CpuState::Running);

        // read 33 from input and write it out
        let prog = parse_prog("3,0,4,0,99");
        let mut cpu = Cpu::new(&prog);
        cpu.add_input(33);
        let target = vec![33, 0, 4, 0, 99];
        assert_eq!(cpu.input, vec![33]);
        assert_eq!(cpu.exec(), CpuState::Running);
        assert_eq!(cpu.exec(), CpuState::Running);
        assert_eq!(cpu.input, vec![]);
        assert_eq!(cpu.mem[..target.len()], target[..]);
        assert_eq!(cpu.pop_output().unwrap(), 33);

        // jmp to IP 0
        let prog = parse_prog("1105,1,0,99");
        let mut cpu = Cpu::new(&prog);
        let cont = cpu.exec();
        assert_eq!(cpu.instr_ptr, 0);
        assert_eq!(cont, CpuState::Running);

        // jmp fail to IP 3
        let prog = parse_prog("1105,0,0,99");
        let mut cpu = Cpu::new(&prog);
        let cont = cpu.exec();
        assert_eq!(cpu.instr_ptr, 3);
        assert_eq!(cont, CpuState::Running);

        // jmp fail to IP 3
        let prog = parse_prog("1106,1,0,99");
        let mut cpu = Cpu::new(&prog);
        let cont = cpu.exec();
        assert_eq!(cpu.instr_ptr, 3);
        assert_eq!(cont, CpuState::Running);

        // jmp to IP 0
        let prog = parse_prog("1106,0,0,99");
        let mut cpu = Cpu::new(&prog);
        let cont = cpu.exec();
        assert_eq!(cpu.instr_ptr, 0);
        assert_eq!(cont, CpuState::Running);

        // 5 < 7 == 1
        let prog = parse_prog("1107,5,7,0,99");
        let mut cpu = Cpu::new(&prog);
        let cont = cpu.exec();
        assert_eq!(cpu.mem[0], 1);
        assert_eq!(cont, CpuState::Running);

        // 7 < 5 == 0
        let prog = parse_prog("1107,7,5,0,99");
        let mut cpu = Cpu::new(&prog);
        let cont = cpu.exec();
        assert_eq!(cpu.mem[0], 0);
        assert_eq!(cont, CpuState::Running);

        // (5 == 5) == 1
        let prog = parse_prog("1108,5,5,0,99");
        let mut cpu = Cpu::new(&prog);
        let cont = cpu.exec();
        assert_eq!(cpu.mem[0], 1);
        assert_eq!(cont, CpuState::Running);

        // (7 == 5) == 0
        let prog = parse_prog("1108,7,5,0,99");
        let mut cpu = Cpu::new(&prog);
        let cont = cpu.exec();
        assert_eq!(cpu.mem[0], 0);
        assert_eq!(cont, CpuState::Running);

        // adj rel base +5
        let prog = parse_prog("1109,5,99");
        let mut cpu = Cpu::new(&prog);
        let cont = cpu.exec();
        assert_eq!(cpu.relative_base, 5);
        assert_eq!(cont, CpuState::Running);

        // output copy of self
        let prog_txt = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let prog = parse_prog(prog_txt);
        let mut cpu = Cpu::new(&prog);
        let target = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99
        ];
        cpu.set_print_output(false);
        cpu.exec_prog();
        assert_eq!(cpu.output, target);
        assert_eq!(cpu.get_state(), CpuState::Done);

        // output 1,219,070,632,396,864
        let prog_txt = "1102,34915192,34915192,7,4,7,99,0";
        let prog = parse_prog(prog_txt);
        let mut cpu = Cpu::new(&prog);
        let target = vec![1219070632396864];
        cpu.set_print_output(false);
        cpu.exec_prog();
        assert_eq!(cpu.output, target);
        assert_eq!(cpu.get_state(), CpuState::Done);

        // output 1125899906842624
        let prog_txt = "104,1125899906842624,99";
        let prog = parse_prog(prog_txt);
        let mut cpu = Cpu::new(&prog);
        let target = vec![1125899906842624];
        cpu.set_print_output(false);
        cpu.exec_prog();
        assert_eq!(cpu.output, target);
        assert_eq!(cpu.get_state(), CpuState::Done);
    }

    #[test]
    fn test_parse_prog() {
        use super::*;

        let result = parse_prog("1,1,1,4,99,5,6,0,99");
        assert_eq!(result, vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
     }
}
