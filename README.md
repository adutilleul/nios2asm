# NIOS2ASM

A tiny NIOS2 assembler written in Rust.

## Example

```
$ ./nios2asm [input.s] [output.bin] (text_min_address) (data_min_address)
```

### input.s

```
	.data
data1:	.word	10
data2:	.word	-20
	.text
main:
	addi	sp, r0, 0x20000
	addi	r4, r0, 10
	call	fact
	br	boucle
fact: 
	addi	sp, sp, -8
	stw	ra, 4(sp)
	bne	r4, zero,.L2
	addi	r2, r0, 1
	br	.L4
.L2: 
	stw	r4,0(sp)
	addi	r4, r4, -1
	call	fact
	ldw	r4,0(sp)
	mul	r2, r4, r2
.L4: 
	ldw	ra, 4(sp)
	addi	sp, sp, 8
	ret 
boucle:
	br	boucle
```

### output.o

```
v2.0 raw
0*0 07000004 01000284 00000100 00000346 e73ffe04 e7c00115 2000009e 00800044 00000146 e1000015 213fffc4 00000100 e1000017 2085383a e7c00117 e7000204 0000283a 003fffc6 
82*0 0000000a ffffffec
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
