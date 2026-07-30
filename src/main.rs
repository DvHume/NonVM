use std::io::*;
#[derive(Debug, Clone)]
enum Instruction {
    Push(i32), // Положить число на вершину стека
    Read, // Прочитать число от пользователся и положить в стек
    Add, // Взять 2 верхних числа со стека, сложить и положить результат обратно
    Sub, // Взять 2 верхних числа, вычесть и положить результат обратно
    Mul, // Перемножить 2 верхних числа
    Print, // Достать верхнее число и вывести на экран
    Halt, // остановить работу
}

struct VM {
    stack: Vec<i32>, // стек чисел
    ip: usize, // указатель на текущую инструкцию(начинаем с 0)
}

impl VM {
    fn new() -> Self {
        VM { stack: Vec::new(), ip: 0, }
    }


fn run(&mut self, program: Vec<Instruction>) {
    loop {
        // Проверяем, не вышли ли мы за пределы программы
        if self.ip >= program.len() {
            break;
        }

        // берём текущую инструкцию
        let instruction = &program[self.ip];
        self.ip += 1; // переходим к следующей инструкции

        match instruction {
            Instruction::Push(val) => self.stack.push(*val),
            Instruction::Read => {
                print!("Input number: ");
                stdout().flush().unwrap();

                let mut input = String::new();
                stdin().read_line(&mut input).expect("msg");

                let number: i32 = input.trim().parse().expect("msg");

                self.stack.push(number);
            }
            Instruction::Add => {
                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                self.stack.push(a + b);
            }
            Instruction::Sub => {
                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                self.stack.push(a - b);
            }
            Instruction::Mul => {
                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                self.stack.push(a * b);
            }
            Instruction::Print => {
                let val = self.stack.pop().unwrap();
                println!("Result: {}", val);
            }
            Instruction::Halt => {
                println!("Programm stoped...");
                break;
            }
        }
    }
 }
}

fn main() {
    let program = vec![
        Instruction::Read,
        Instruction::Push(10),
        Instruction::Mul,
        Instruction::Print,
        Instruction::Halt,
    ];
    let mut vm = VM::new();
    vm.run(program);
}
