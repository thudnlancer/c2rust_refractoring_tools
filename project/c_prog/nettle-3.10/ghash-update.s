











































	.file "ghash-update.asm"

	
	
	

	.text
	.align 16

.globl _nettle_ghash_update_table
.type _nettle_ghash_update_table,%function
_nettle_ghash_update_table: 
	
  

	sub	$1, %rdx
	movups	(%rsi), %xmm0
	jc	.Ldone
	
	lea	1024(%rdi), %r8

.align 16

.Lblock_loop:
	
	movups	(%rcx), %xmm2
	pxor	%xmm2, %xmm0
	pxor	%xmm1, %xmm1
	mov	$992, %rax
.align 16

.Loop_bit:
	movdqa	%xmm0, %xmm5
	psrad	$31, %xmm5
	pshufd	$0x00, %xmm5, %xmm2
	pshufd	$0x55, %xmm5, %xmm3
	pshufd	$0xaa, %xmm5, %xmm4
	pshufd	$0xff, %xmm5, %xmm5
	pslld	$1, %xmm0
	
	
	
	
	movups	(%rdi, %rax), %xmm6
	movups	(%r8, %rax), %xmm7
	pand	%xmm6, %xmm2
	pand	%xmm7, %xmm3
	movups	16(%rdi, %rax), %xmm6
	movups	16(%r8, %rax), %xmm7
	pand	%xmm6, %xmm4
	pand	%xmm7, %xmm5
	pxor	%xmm2, %xmm3
	pxor	%xmm4, %xmm5
	pxor	%xmm3, %xmm1
	pxor	%xmm5, %xmm1

	sub	$32, %rax
	jnc	.Loop_bit

	movaps	%xmm1, %xmm0

	add	$16, %rcx
	sub	$1, %rdx
	jnc	.Lblock_loop

.Ldone:
	movups	%xmm0, (%rsi)
	mov	%rcx, %rax
	
  

	ret
.size _nettle_ghash_update_table, . - _nettle_ghash_update_table



.section .note.GNU-stack,"",%progbits
