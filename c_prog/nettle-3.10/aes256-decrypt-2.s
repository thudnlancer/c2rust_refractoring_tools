

















































	.file "aes256-decrypt.asm"

	
	
	

	.text
	.align 16

.globl _nettle_aes256_decrypt_aesni
.type _nettle_aes256_decrypt_aesni,%function
_nettle_aes256_decrypt_aesni: 
	
  

	shr	$4, %rsi
	test	%rsi, %rsi
	jz	.Lend

	movups	224(%rdi), %xmm0
	movups	208(%rdi), %xmm1
	movups	192(%rdi), %xmm2
	movups	176(%rdi), %xmm3
	movups	160(%rdi), %xmm4
	movups	144(%rdi), %xmm5
	movups	128(%rdi), %xmm6
	movups	96(%rdi), %xmm7
	movups	80(%rdi), %xmm8
	movups	64(%rdi), %xmm9
	movups	48(%rdi), %xmm10
	movups	32(%rdi), %xmm11
	movups	16(%rdi), %xmm12
	movups	(%rdi), %xmm13

	shr	%rsi
	jnc	.Lblock_loop

	movups	(%rcx), %xmm14
	pxor	%xmm0, %xmm14
	movups	112(%rdi), %xmm0
	aesdec	%xmm1, %xmm14
	aesdec	%xmm2, %xmm14
	aesdec	%xmm3, %xmm14
	aesdec	%xmm4, %xmm14
	aesdec	%xmm5, %xmm14
	aesdec	%xmm6, %xmm14
	aesdec	%xmm0, %xmm14
	movups	224(%rdi), %xmm0
	aesdec	%xmm7, %xmm14
	aesdec	%xmm8, %xmm14
	aesdec	%xmm9, %xmm14
	aesdec	%xmm10, %xmm14
	aesdec	%xmm11, %xmm14
	aesdec	%xmm12, %xmm14
	aesdeclast %xmm13, %xmm14

	movups	%xmm14, (%rdx)
	add	$16, %rcx
	add	$16, %rdx
	test	%rsi, %rsi
	jz	.Lend

.Lblock_loop:
	movups	(%rcx), %xmm14
	movups	16(%rcx), %xmm15
	pxor	%xmm0, %xmm14
	pxor	%xmm0, %xmm15
	movups	112(%rdi), %xmm0
	aesdec	%xmm1, %xmm14
	aesdec	%xmm1, %xmm15
	aesdec	%xmm2, %xmm14
	aesdec	%xmm2, %xmm15
	aesdec	%xmm3, %xmm14
	aesdec	%xmm3, %xmm15
	aesdec	%xmm4, %xmm14
	aesdec	%xmm4, %xmm15
	aesdec	%xmm5, %xmm14
	aesdec	%xmm5, %xmm15
	aesdec	%xmm6, %xmm14
	aesdec	%xmm6, %xmm15
	aesdec	%xmm0, %xmm14
	aesdec	%xmm0, %xmm15
	movups	224(%rdi), %xmm0
	aesdec	%xmm7, %xmm14
	aesdec	%xmm7, %xmm15
	aesdec	%xmm8, %xmm14
	aesdec	%xmm8, %xmm15
	aesdec	%xmm9, %xmm14
	aesdec	%xmm9, %xmm15
	aesdec	%xmm10, %xmm14
	aesdec	%xmm10, %xmm15
	aesdec	%xmm11, %xmm14
	aesdec	%xmm11, %xmm15
	aesdec	%xmm12, %xmm14
	aesdec	%xmm12, %xmm15
	aesdeclast %xmm13, %xmm14
	aesdeclast %xmm13, %xmm15

	movups	%xmm14, (%rdx)
	movups	%xmm15, 16(%rdx)
	add	$32, %rcx
	add	$32, %rdx
	dec	%rsi
	jnz	.Lblock_loop

.Lend:
	
  

	ret
.size _nettle_aes256_decrypt_aesni, . - _nettle_aes256_decrypt_aesni



.section .note.GNU-stack,"",%progbits
