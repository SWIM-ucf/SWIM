# SWIM (Simple Web Interface for MIPS)

This was originally developed by Kevin Cahalan, Jerrett Longworth, Huy Nguyen, Evan Raiford, and Jimmie Smith at UCF as a senior design project.

![Screenshot of Swim V1](media/swim-screenshot.png)

A web-based emulator for MIPS64 made for educational purposes. Its emulation core supports over 60 real instructions and 20 pseudo-instuctions and an user interface that provides the following features:
- Step execute and execute code down to the individual [stages](https://en.wikipedia.org/wiki/Instruction_cycle)
- Upload files to SWIM and Copy code to the user's clipboard to be saved locally 
	- Note: For Chromium-based browsers on Mac, the user will have to manually copy-paste the code onto a text editor. This is done as followed:
		1. Click on the Editor window.
		2. Press `Cmd + A` to select all text.
		3. Press `Cmd + C` to copy the text on your clipboard.
		4. Press `Cmd + V` to paste the text in your text editor to save the code.
- A register viewer that displays General Purpose and Floating Point registers with toggling to different views (decimal, binary, hexadecimal, float, double)
- A console viewer to display errors and suggestions on fixing them
- A memory viewer to see the code compiled and updated as it executes
- A visualization of the datapath to see the individual parts that make up the general and floating-point coprocessors and the values inside each wire
- Utilizes the [Monaco Editor](https://microsoft.github.io/monaco-editor/) code library to provide:
	- Syntax highlighting of our custom language
	- Highlighting the previously executed line
	- Providing mouse hover information on instructions and errors
	- Expands the pseudo-instructions into their hardware equivalent upon assembling code

Supported Instructions:
- Conventional Instructions:
	- add
	- addi
	- addiu
	- addu
	- and
	- andi
	- aui
	- b
	- beq
	- bne
	- dadd
	- daddi
	- daddiu
	- daddu
	- dahi
	- dati
	- ddiv
	- ddivu
	- div
	- dmul
	- dmulu
	- dsub
	- dsubu
	- j
	- jal
	- jalr
	- jr
	- lui
	- lw
	- mul
	- nop
	- or
	- ori
	- sll
	- slt
	- sltu
	- sub
	- sw
	- syscall `Note: This is currently a stubbed instruction to halt emulation.`

- Floating-Point Instructions:
	- add.d
	- add.s
	- bc1f
	- bc1t
	- c.eq.d
	- c.eq.s
	- c.le.d
	- c.le.s
	- c.lt.d
	- c.lt.s
	- c.nge.d
	- c.nge.s
	- c.ngt.d
	- c.ngt.s
	- div.d
	- div.s
	- dmfc1
	- dmtc1
	- lwc1
	- mfc1
	- mtc1
	- mul.d
	- mul.s
	- sub.d
	- sub.s
	- swc1

- Pseudo-instructions:
	- ddivi
	- ddiviu
	- divi
	- dmuli
	- dmuliu
	- dsubi
	- dsubiu
	- li
	- lw `(followed by a label)`
	- move
	- muli
	- seq
	- sge
	- sgeu
	- sgt
	- sgtu
	- sle
	- sleu
	- sne
	- subi
	- sw `(followed by a label)`

Supported .data directives:
- .ascii
- .asciiz
- .byte
- .double
- .float
- .half
- .space
- .word

All of this wholly developed with the [Rust](https://www.rust-lang.org/) language with the interface built with the [Yew](https://yew.rs/) framework which uses [WebAssembly](https://webassembly.org/) and JavaScript to house the emulation core and parser/assembler.

## Compiling

While SWIM is currently being hosted [here](https://swim-ucf.github.io/SWIM/), you can compile and run it locally on your browser as long as it supports WebAssembly.

1. Install the latest stable rust toolchain with `rustup` at https://www.rust-lang.org/tools/install
	- If you plan on compiling the source yourself, make sure to add WebAssembly as a compile target by typing `rustup target add wasm32-unknown-unknown` in your terminal
2. Install [trunk](https://trunkrs.dev/#install)
3. `git clone` the repository or download the source [here](https://github.com/SWIM-ucf/SWIM/releases)
4. When you are in the root directory of the project, type `trunk serve --open` in your terminal to load it locally

## Licensing

SWIM is licensed under GNU's GPL-3.0 as shown [here](LICENSE)
