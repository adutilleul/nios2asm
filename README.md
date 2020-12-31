# MIPSsembler

A tiny MIPS assembler written in Rust.

![memo](https://user-images.githubusercontent.com/6410412/103358909-1d5f5b00-4afa-11eb-81e8-0aec397aa515.png)

## MIPS(Microprocessor without Interlocked Pipeline Stages)
 
### Basic instruction formats

#### R (Register)

```
and $17, $17, $0

+-----------+-------+-------+-------+----------+----------+
| opcode(6) | rs(5) | rt(5) | rd(5) | shamt(5) | funct(6) |
+-----------+-------+-------+-------+----------+----------+
| 000000    | 10001 | 00000 | 10001 | 00000    | 100100   |
+-----------+-------+-------+-------+----------+----------+
```

#### I (Immediate)

```
addiu $17, $17, 0x1

+-----------+-------+-------+-----------------------------+
| opcode(6) | rs(5) | rt(5) | immediate(16)               |
+-----------+-------+-------+-----------------------------+
| 001001    | 10001 | 10001 | 0000000000000001            |
+-----------+-------+-------+-----------------------------+
```

#### J (Jump)

```
j lab1

+-----------+---------------------------------------------+
| opcode(6) | address(26)                                 |
+-----------+---------------------------------------------+
| 000010    | 00000100000000000000000110                  |
+-----------+---------------------------------------------+
```

### Memory allocation

```
$sp +---------------+ 0x7ffffffc
    | Stack         | 
    +-------+-------+
    |       |       |
    |       v       |
    |               |
    |       ^       |
    |       |       |
    +-------+-------+
    | Heap          |
$gp +---------------+ 0x10008000
    | Static(Data)  |
    +---------------+ 0x10000000
    | Text(Code)    |
 PC +---------------+ 0x00400000
    | Reserved      |
    +---------------+ 0x00000000
```

## Example

```
$ mipssembler input.s output.o
```

### input.s

```
	.data
data1:	.word	100
data2:	.word	200
data3:	.word	0x12345678
	.text
main:
	and	$17, $17, $0
	and	$18, $18, $0
	la	$8, data1
	la	$9, data2
	and	$10, $10, $0
lab1:
	and	$11, $11, $0
lab2:
	addiu	$17, $17, 0x1
	addiu	$11, $11, 0x1
	or	$9, $9, $0
	bne	$11, $8, lab2
lab3:
	addiu	$18, $18, 0x2
	addiu	$11, $11, 1
	sll	$18, $17, 1
	srl	$17, $18, 1
	and	$19, $17, $18
	bne	$11, $9, lab3
lab4:
	addu	$5, $5, $31
	nor	$16, $17, $18
	beq	$10, $8, lab5
	j	lab1
lab5:
	ori	$16, $16, 0xf0f0
```

### output.o

```
00000000000000000000000001011000
00000000000000000000000000001100
00000010001000001000100000100100
00000010010000001001000000100100
00111100000010000001000000000000
00111100000010010001000000000000
00110101001010010000000000000100
00000001010000000101000000100100
00000001011000000101100000100100
00100110001100010000000000000001
00100101011010110000000000000001
00000001001000000100100000100101
00010101011010001111111111111100
00100110010100100000000000000010
00100101011010110000000000000001
00000000000100011001000001000000
00000000000100101000100001000010
00000010001100101001100000100100
00010101011010011111111111111010
00000000101111110010100000100001
00000010001100101000000000100111
00010001010010000000000000000001
00001000000100000000000000000110
00110110000100001111000011110000
00000000000000000000000001100100
00000000000000000000000011001000
00010010001101000101011001111000
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
