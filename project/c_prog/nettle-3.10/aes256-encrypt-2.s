

















































	.file "aes256-encrypt.asm"

	
	
	

	.text
	.align 16

.globl _nettle_aes256_encrypt_aesni
.type _nettle_aes256_encrypt_aesni,%function
_nettle_aes256_encrypt_aesni: 
	
  

	shr	$4, %rsi
	test	%rsi, %rsi
	jz	.Lend

	movups	(%rdi), %xmm0
	movups	16(%rdi), %xmm1
	movups	32(%rdi), %xmm2
	movups	48(%rdi), %xmm3
	movups	64(%rdi), %xmm4
	movups	80(%rdi), %xmm5
	movups	96(%rdi), %xmm6
	movups	128(%rdi), %xmm7
	movups	144(%rdi), %xmm8
	movups	160(%rdi), %xmm9
	movups	176(%rdi), %xmm10
	movups	192(%rdi), %xmm11
	movups	208(%rdi), %xmm12
	movups	224(%rdi), %xmm13
	shr	%rsi
	jnc	.Lblock_loop

	movups	(%rcx), %xmm14
	pxor	%xmm0, %xmm14
	movups	112(%rdi), %xmm0
	aesenc	%xmm1, %xmm14
	aesenc	%xmm2, %xmm14
	aesenc	%xmm3, %xmm14
	aesenc	%xmm4, %xmm14
	aesenc	%xmm5, %xmm14
	aesenc	%xmm6, %xmm14
	aesenc	%xmm0, %xmm14
	movups	(%rdi), %xmm0
	aesenc	%xmm7, %xmm14
	aesenc	%xmm8, %xmm14
	aesenc	%xmm9, %xmm14
	aesenc	%xmm10, %xmm14
	aesenc	%xmm11, %xmm14
	aesenc	%xmm12, %xmm14
	aesenclast %xmm13, %xmm14

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
	aesenc	%xmm1, %xmm14
	aesenc	%xmm1, %xmm15
	aesenc	%xmm2, %xmm14
	aesenc	%xmm2, %xmm15
	aesenc	%xmm3, %xmm14
	aesenc	%xmm3, %xmm15
	aesenc	%xmm4, %xmm14
	aesenc	%xmm4, %xmm15
	aesenc	%xmm5, %xmm14
	aesenc	%xmm5, %xmm15
	aesenc	%xmm6, %xmm14
	aesenc	%xmm6, %xmm15
	aesenc	%xmm0, %xmm14
	aesenc	%xmm0, %xmm15
	movups	(%rdi), %xmm0
	aesenc	%xmm7, %xmm14
	aesenc	%xmm7, %xmm15
	aesenc	%xmm8, %xmm14
	aesenc	%xmm8, %xmm15
	aesenc	%xmm9, %xmm14
	aesenc	%xmm9, %xmm15
	aesenc	%xmm10, %xmm14
	aesenc	%xmm10, %xmm15
	aesenc	%xmm11, %xmm14
	aesenc	%xmm11, %xmm15
	aesenc	%xmm12, %xmm14
	aesenc	%xmm12, %xmm15
	aesenclast %xmm13, %xmm14
	aesenclast %xmm13, %xmm15

	movups	%xmm14, (%rdx)
	movups	%xmm15, 16(%rdx)
	add	$32, %rcx
	add	$32, %rdx
	dec	%rsi
	jnz	.Lblock_loop

.Lend:
	
  

	ret
.size _nettle_aes256_encrypt_aesni, . - _nettle_aes256_encrypt_aesni



.section .note.GNU-stack,"",%progbits
