# Pep/8 Assembler

Command line assembler written in rust for the educational assembly language [Pep/8](https://github.com/StanWarford/pep8).

## Install from source

Compile the project using cargo:

```sh
cargo build -r
```

## Usage

To run the program, you will need an input file. Here is an example:

```asm
         BR      main        ;Branch around data
num:     .BLOCK  2           ;Global variable
main:    DECI    num,d       ;Input decimal value
         DECO    num,d       ;Output decimal value
         CHARO   '\n',i
         STRO    msg,d       ;Output message
         STOP
msg:     .ASCII  "That's all.\n\x00"
a:       .ADDRSS num
         .END
```

You can then assemble it like so:

```sh
assembler example.pep -o output.pepo
```
