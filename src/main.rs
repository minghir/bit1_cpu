// Definirea stării procesorului pe 1 bit
struct Cpu1Bit {
    rr: bool,          // Result Register
    pc: u8,            // Program Counter
    ram: [bool; 16],   // 16 locații de memorie RAM (1 bit fiecare)
    rom: [u8; 32],     // Memoria ROM de program
    halted: bool,      // Flag pentru oprire
}

impl Cpu1Bit {
    fn new() -> Self {
        Self {
            rr: false,
            pc: 0,
            ram: [false; 16],
            rom: [0; 32],
            halted: false,
        }
    }

    // Funcția step scrisă complet cu valori binare
    fn step(&mut self) {
        if self.halted {
            return;
        }

        let instruction = self.rom[self.pc as usize];

        // Extragem opcode-ul (primii 4 biți) și adresa (ultimii 4 biți)
        let opcode = (instruction & 0b1111_0000) >> 4;
        let addr = (instruction & 0b0000_1111) as usize;

        // Decodificarea folosind strict literali binari
        match opcode {
            0b0000 => {
                // HALT
                println!("[CPU] HALT întâlnit.");
                self.halted = true;
            }
            0b0001 => {
                // LOAD addr
                self.rr = self.ram[addr];
                println!("[CPU] LOAD din RAM[{}] -> RR devine {}", addr, self.rr);
                self.pc += 1;
            }
            0b0010 => {
                // AND addr
                self.rr = self.rr && self.ram[addr];
                println!("[CPU] AND cu RAM[{}] -> RR devine {}", addr, self.rr);
                self.pc += 1;
            }
            0b0011 => {
                // OR addr
                self.rr = self.rr || self.ram[addr];
                println!("[CPU] OR cu RAM[{}] -> RR devine {}", addr, self.rr);
                self.pc += 1;
            }
            0b0100 => {
                // OUT addr
                self.ram[addr] = self.rr;
                println!("[CPU] OUT: scriem RR ({}) în RAM[{}]", self.rr, addr);
                self.pc += 1;
            }
            0b0101 => {
                // NOT (inverseaza RR)
                self.rr = !self.rr;
                println!("[CPU] NOT -> RR a fost inversat, acum este: {}", self.rr);
                self.pc += 1;

            }
            0b0110 => {
                // JMP addr (Salt necondiționat la adresa specificată)
                println!("[PC: {}] JMP -> Salt necondiționat la adresa {}", self.pc, addr);
                self.pc = addr as u8;
            }
            0b0111 => {
                // XOR addr
                self.rr = self.rr ^ self.ram[addr];
                println!("[PC: {}] XOR cu RAM[{}] -> RR devine {}", self.pc, addr, self.rr);
                self.pc += 1;
            }
            0b1000 => {
                // JZ addr (Jump if Zero/False: sare dacă RR este false)
                if !self.rr {
                    println!("[PC: {}] JZ luat -> Salt la adresa {}", self.pc, addr);
                    self.pc = addr as u8;
                } else {
                    println!("[PC: {}] JZ ignorat (RR este true)", self.pc);
                    self.pc += 1;
                }
            }
            0b1001 => {
                // JNZ addr (Jump if Not Zero/True: sare dacă RR este true)
                if self.rr {
                    println!("[PC: {}] JNZ luat -> Salt la adresa {}", self.pc, addr);
                    self.pc = addr as u8;
                } else {
                    println!("[PC: {}] JNZ ignorat (RR este false)", self.pc);
                    self.pc += 1;
                }
            }
            0b1010 => {
                // CLR (Setează RR pe false)
                self.rr = false;
                println!("[PC: {}] CLR -> RR a fost resetat la false", self.pc);
                self.pc += 1;
            }
            0b1011 => {
                // NOP (No Operation) - Doar avansează PC-ul, nu face nimic altceva
                println!("[PC: {}] NOP -> Fără operație", self.pc);
                self.pc += 1;
            }
            0b1100 => {
                // NAND addr (Not-AND: negația lui AND)
                self.rr = !(self.rr && self.ram[addr]);
                println!("[PC: {}] NAND cu RAM[{}] -> RR devine {}", self.pc, addr, self.rr);
                self.pc += 1;
            }
            0b1101 => {
                // NOR addr (Not-OR: negația lui OR)
                self.rr = !(self.rr || self.ram[addr]);
                println!("[PC: {}] NOR cu RAM[{}] -> RR devine {}", self.pc, addr, self.rr);
                self.pc += 1;
            }
            0b1110 => {
                // SET1 (Setează RR direct pe true)
                self.rr = true;
                println!("[PC: {}] SET1 -> RR a fost setat la true (1)", self.pc);
                self.pc += 1;
            }
            0b1111 => {
                // SET0 (Setează RR direct pe false)
                self.rr = false;
                println!("[PC: {}] SET0 -> RR a fost setat la false (0)", self.pc);
                self.pc += 1;
            }
            _ => {
                println!("[CPU] Erore: Opcode binar necunoscut {:#010b}", opcode);
                self.halted = true;
            }
        }
    }

    fn run(&mut self) {
        while !self.halted {
            self.step();
        }
    }
}


use std::fs;

fn main() {
    let mut cpu = Cpu1Bit::new();

    println!("========================================");
    println!("  EMULATOR 1-BIT (VALORI HARDCODATE)    ");
    println!("========================================");

    // --- SETĂM DATELE INIȚIALE DIRECT ÎN RAM ---
    cpu.ram[0] = true;  // Valoarea pentru prima instrucțiune (LOAD / JNZ)
    cpu.ram[1] = true;  // Valoarea pentru OR
    cpu.ram[2] = false; // Valoarea pentru XOR
    cpu.ram[3] = true;  // Valoarea pentru AND
    
    println!("[Sistem] RAM inițializat: RAM[0]={}, RAM[1]={}, RAM[2]={}, RAM[3]={}", 
        cpu.ram[0], cpu.ram[1], cpu.ram[2], cpu.ram[3]
    );

    // --- CITIREA PROGRAMULUI DIN FIȘIERUL EXTERN "program.txt" ---
    println!("[Sistem] Se citește programul din 'program.txt'...");
    
    let file_contents = match fs::read_to_string("program.txt") {
        Ok(content) => content,
        Err(e) => {
            println!("Eroare la citirea fișierului program.txt: {}", e);
            return;
        }
    };

    let mut program = Vec::new();
    for line in file_contents.lines() {
        let clean_line = line.trim();
        // Ignorăm liniile goale sau comentariile
        if clean_line.is_empty() || clean_line.starts_with('#') {
            continue;
        }

        // Luăm doar prima parte a liniei (ignorând comentariile de la sfârșitul liniei, ex: "00010000  # LOAD")
        let instruction_str = clean_line.split('#').next().unwrap().trim();
        if instruction_str.is_empty() {
            continue;
        }

        match u8::from_str_radix(instruction_str, 2) {
            Ok(byte) => program.push(byte),
            Err(e) => println!("Eroare de parsare pe linia '{}': {}", instruction_str, e),
        }
    }

    // Încărcăm programul în ROM
    for (i, &byte) in program.iter().enumerate() {
        if i < cpu.rom.len() {
            cpu.rom[i] = byte;
        }
    }

    println!("\n--- Pornire Execuție ---");
    cpu.run();
    println!("--- Oprire Execuție ---");

    println!("\nRezultat final: RAM[4] = {}", if cpu.ram[4] { 1 } else { 0 });
}
