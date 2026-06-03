# CHIP-8 Emulator

A CHIP-8 emulator built from scratch in Rust. Implements the full CHIP-8 instruction set with a display renderer, sound, and keyboard input.

<img width="752" height="460" alt="demo" src="https://github.com/user-attachments/assets/69498ace-3039-4715-bcf4-810386e2a545" />


## What is CHIP-8?

CHIP-8 is a virtual machine from the 1970s designed to make game development easier on early computers. It has 4KB of memory, 16 registers, a 64x32 pixel display, and 35 opcodes. Building an emulator for it is a classic systems programming exercise since it teaches you how CPUs work at the instruction level — fetch, decode, execute.

## Features

- Full CHIP-8 instruction set (35 opcodes)
- 64x32 display scaled to 640x320
- Sound via sine wave beeper
- Keyboard input (16 keys)
- FPS counter in window title
- Font rendering (0-F characters)

## Tech Stack

- **Rust**
- **minifb** — window and display rendering
- **rodio** — audio

## Controls

CHIP-8 has a 16 key hex keypad mapped to your keyboard:

```
CHIP-8      Keyboard
1 2 3 C  →  1 2 3 4
4 5 6 D  →  Q W E R
7 8 9 E  →  A S D F
A 0 B F  →  Z X C V
```

`Esc` to quit.

## Getting Started

### Prerequisites

- Rust

### Run

```bash
git clone https://github.com/tasvln/chip8
cd chip8
cargo run
```

To load a different ROM, update the path in `main.rs`:

```rust
cpu.load_rom("roms/your_rom.ch8");
```

## Project Structure

```
src/
  main.rs         # window, input, render loop
  core/
    chip8.rs      # CPU — memory, registers, opcodes
    sound.rs      # audio beeper
roms/             # CHIP-8 ROM files
```

## Architecture

The emulator runs a fetch/decode/execute cycle at ~600Hz (10 cycles per frame at 60fps):

```
fetch   → read 2 bytes from memory at PC
decode  → extract opcode, registers, values via bitmasking
execute → match opcode and update state
render  → draw 64x32 bool array to scaled pixel buffer
```

## Resources

- [Cowgod's CHIP-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [Octo — browser based CHIP-8 IDE for writing your own ROMs](https://johnearnest.github.io/Octo/)
