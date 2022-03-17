pub const INPUT_CASE_1: &str = "
	.data
data1:	.word	10
data2:	.word	-20
	.text
main:
	addi	sp, r0, 0x200
	addi	r4, r0, 4
	call	fact
	br	loop
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
loop:
	br	loop
";

pub const OUTPUT_CASE_1: &str = "v2.0 raw\n0*0 07008004 01000104 00000100 00000346 e73ffe04 e7c00115 2000009e 00800044 00000146 e1000015 213fffc4 00000100 e1000017 2085383a e7c00117 e7000204 0000283a 003fffc6 \n238*0 0000000a ffffffec";
