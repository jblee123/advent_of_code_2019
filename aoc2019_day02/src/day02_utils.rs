use std::str::FromStr;

#[cfg(test)]
mod tests {
    #[test]
    fn test_new_cpu() {
        use super::*;

        let cpu = Cpu::new(vec![1, 2, 3, 4]);
        assert_eq!(cpu.mem, vec![1, 2, 3, 4]);
        assert_eq!(cpu.instr_ptr, 0);
    }

    #[test]
    fn test_exec() {
        use super::*;

        let prog = parse_prog("1,9,10,3,2,3,11,0,99,30,40,50");
        let mut cpu = Cpu::new(prog);
        assert_eq!(cpu.mem, vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);

        let cont = cpu.exec();
        assert_eq!(cpu.mem, vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
        assert_eq!(cont, true);

        let cont = cpu.exec();
        assert_eq!(cpu.mem, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
        assert_eq!(cont, true);

        let cont = cpu.exec();
        assert_eq!(cpu.mem, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
        assert_eq!(cont, false);
    }

    #[test]
    fn test_run_prog() {
        use super::*;

        let do_test = |prog_str, target_mem| {
            let prog = parse_prog(prog_str);
            let mut cpu = Cpu::new(prog);
            cpu.exec_prog();
            assert_eq!(cpu.mem, target_mem);
        };

        do_test(
            "1,9,10,3,2,3,11,0,99,30,40,50",
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
        );
        do_test(
            "1,0,0,0,99",
            vec![2, 0, 0, 0, 99],
        );
        do_test(
            "2,3,0,3,99",
            vec![2, 3, 0, 6, 99],
        );
        do_test(
            "2,4,4,5,99,0",
            vec![2, 4, 4, 5, 99, 9801],
        );
        do_test(
            "1,1,1,4,99,5,6,0,99",
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
        );
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
const END_OP: i64 = 99;

pub struct Cpu {
    pub mem: Vec<i64>,
    instr_ptr: usize,
}

impl Cpu {
    pub fn new(mem: Vec<i64>) -> Cpu {
        Cpu {
            mem: mem,
            instr_ptr: 0,
        }
    }

    pub fn exec(&mut self) -> bool {
        if self.instr_ptr >= self.mem.len() {
            return false;
        }

        let op = self.mem[self.instr_ptr];

        if op == END_OP {
            return false;
        }

        let src1 = self.mem[self.instr_ptr + 1] as usize;
        let src2 = self.mem[self.instr_ptr + 2] as usize;
        let dest = self.mem[self.instr_ptr + 3] as usize;

        match op {
            ADD_OP => {
                self.mem[dest] = self.mem[src1] + self.mem[src2];
            },
            MUL_OP => {
                self.mem[dest] = self.mem[src1] * self.mem[src2];
            },
            _ => panic!("bad opcode: {}", op),
        };

        self.instr_ptr += 4;
        self.instr_ptr < self.mem.len()
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
