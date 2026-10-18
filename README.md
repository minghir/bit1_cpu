# 1-bit CPU Emulator in Rust

Un emulator complet funcțional pentru un procesor minimalist pe 1-bit, scris în **Rust**, conceput ca proiect de învățare pentru arhitectura calculatoarelor și programare low-level.

## Caracteristici
- Arhitectură bazată pe un Registru de Rezultat (`RR`), Program Counter (`PC`), memorie RAM (16 biți) și ROM (32 octeți).
- Suport pentru decodificare completă în binar pur (`0b...`).
- Citirea programelor din fișiere externe (`program.txt`) cu suport pentru comentarii.
- Set extins de instrucțiuni logice și de control al fluxului (11 op-coduri).

## Instrucțiuni suportate (Op-coduri pe 4 biți)
- `0000` - **HALT**: Oprește execuția.
- `0001` - **LOAD addr**: Încarcă RAM[addr] în RR.
- `0010` - **AND addr**: Realizează operația AND între RR și RAM[addr].
- `0011` - **OR addr**: Realizează operația OR între RR și RAM[addr].
- `0100` - **OUT addr**: Scrie RR în RAM[addr].
- `0101` - **NOT**: Inversează valoarea lui RR.
- `0110` - **JMP addr**: Salt necondiționat la adresă.
- `0111` - **XOR addr**: Realizează operația XOR între RR și RAM[addr].
- `1000` - **JZ addr**: Salt dacă RR este false (0).
- `1001` - **JNZ addr**: Salt dacă RR este true (1).
- `1010` - **CLR**: Resetează RR la false (0).

## Cum se rulează
Asigură-te că ai **Rust și Cargo** instalate, apoi rulează în terminal:
```bash
cargo run
