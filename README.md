# 🚀 1-bit CPU Emulator in Rust

A fully functional emulator for a minimalist 1-bit CPU written in **Rust**, designed as a learning project for computer architecture, embedded systems, and low-level programming.

---

## ✨ Main Features
- **Pure Architecture:** Based on a Result Register (`RR`), Program Counter (`PC`), 16-bit RAM memory, and 32-byte ROM.
- **Pure Binary Decoding:** Hardware-level bit manipulation using masks and binary literals (`0b...`).
- **Maximum Capacity (100%):** Complete set of **16 opcodes** (fully exhausting the 4-bit opcode space, just like a real 1970s processor).
- **External File System:** Reads programs from a `program.txt` file with native comment support.
- **Control Flow:** Conditional jumps (`JZ`, `JNZ`) and unconditional jumps (`JMP`).

---

## 🛠️ Complete Instruction Set (4-bit Opcodes)

| Opcode (Binary) | Mnemonic | Description / Functionality |
| :--- | :--- | :--- |
| `0000` | **HALT** | Halts CPU execution. |
| `0001` | **LOAD addr** | Loads state from `RAM[addr]` into `RR`. |
| `0010` | **AND addr** | Performs logical AND between `RR` and `RAM[addr]`. |
| `0011` | **OR addr** | Performs logical OR between `RR` and `RAM[addr]`. |
| `0100` | **OUT addr** | Writes current value from `RR` into `RAM[addr]`. |
| `0101` | **NOT** | Inverts current state of `RR`. |
| `0110` | **JMP addr** | Unconditional jump to the specified address in ROM. |
| `0111` | **XOR addr** | Performs logical XOR between `RR` and `RAM[addr]`. |
| `1000` | **JZ addr** | Conditional jump: jumps to address if `RR` is `false` (0). |
| `1001` | **JNZ addr** | Conditional jump: jumps to address if `RR` is `true` (1). |
| `1010` | **CLR** | Directly resets `RR` to `false` (0). |
| `1011` | **NOP** | No Operation (Does nothing, just advances `PC`). |
| `1100` | **NAND addr** | Performs logical NAND between `RR` and `RAM[addr]`. |
| `1101` | **NOR addr** | Performs logical NOR between `RR` and `RAM[addr]`. |
| `1110` | **SET1** | Directly sets `RR` to `true` (1). |
| `1111` | **SET0** | Directly sets `RR` to `false` (0). |

---

## 🚀 How to Run

1. Ensure **Rust and Cargo** are installed on your system.
2. Check or edit the `program.txt` file with your desired binary instructions.
3. Run the command in your terminal:

```bash
cargo run