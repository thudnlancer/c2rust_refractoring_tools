
















































	.file "aes192-decrypt.asm"

	
	
	

	.text
	.align 16

.globl _nettle_aes192_decrypt_aesni
.type _nettle_aes192_decrypt_aesni,%function
_nettle_aes192_decrypt_aesni: 
	
  

	shr	$4, %rsi
	test	%rsi, %rsi
	jz	.Lend

	movups	192(%rdi), %xmm0
	movups	176(%rdi), %xmm1
	movups	160(%rdi), %xmm2
	movups	144(%rdi), %xmm3
	movups	128(%rdi), %xmm4
	movups	112(%rdi), %xmm5
	movups	96(%rdi), %xmm6
	movups	80(%rdi), %xmm7
	movups	64(%rdi), %xmm8
	movups	48(%rdi), %xmm9
	movups	32(%rdi), %xmm10
	movups	16(%rdi), %xmm11
	movups	(%rdi), %xmm12
	shr	%rsi
	jnc	.Lblock_loop

	movups	(%rcx), %xmm13
	pxor	%xmm0, %xmm13
	aesdec	%xmm1, %xmm13
	aesdec	%xmm2, %xmm13
	aesdec	%xmm3, %xmm13
	aesdec	%xmm4, %xmm13
	aesdec	%xmm5, %xmm13
	aesdec	%xmm6, %xmm13
	aesdec	%xmm7, %xmm13
	aesdec	%xmm8, %xmm13
	aesdec	%xmm9, %xmm13
	aesdec	%xmm10, %xmm13
	aesdec	%xmm11, %xmm13
	aesdeclast %xmm12, %xmm13

	movups	%xmm13, (%rdx)
	add	$16, %rcx
	add	$16, %rdx
	test	%rsi, %rsi
	jz	.Lend

.Lblock_loop:
	movups	(%rcx), %xmm13
	movups	16(%rcx), %xmm14
	pxor	%xmm0, %xmm13
	pxor	%xmm0, %xmm14
	aesdec	%xmm1, %xmm13
	aesdec	%xmm1, %xmm14
	aesdec	%xmm2, %xmm13
	aesdec	%xmm2, %xmm14
	aesdec	%xmm3, %xmm13
	aesdec	%xmm3, %xmm14
	aesdec	%xmm4, %xmm13
	aesdec	%xmm4, %xmm14
	aesdec	%xmm5, %xmm13
	aesdec	%xmm5, %xmm14
	aesdec	%xmm6, %xmm13
	aesdec	%xmm6, %xmm14
	aesdec	%xmm7, %xmm13
	aesdec	%xmm7, %xmm14
	aesdec	%xmm8, %xmm13
	aesdec	%xmm8, %xmm14
	aesdec	%xmm9, %xmm13
	aesdec	%xmm9, %xmm14
	aesdec	%xmm10, %xmm13
	aesdec	%xmm10, %xmm14
	aesdec	%xmm11, %xmm13
	aesdec	%xmm11, %xmm14
	aesdeclast %xmm12, %xmm13
	aesdeclast %xmm12, %xmm14

	movups	%xmm13, (%rdx)
	movups	%xmm14, 16(%rdx)
	add	$32, %rcx
	add	$32, %rdx
	dec	%rsi
	jnz	.Lblock_loop

.Lend:
	
  

	ret
.size _nettle_aes192_decrypt_aesni, . - _nettle_aes192_decrypt_aesni



.section .note.GNU-stack,"",%progbits
