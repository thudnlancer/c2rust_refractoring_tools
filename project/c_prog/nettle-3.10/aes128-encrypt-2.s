














































	.file "aes128-encrypt.asm"

	
	
	

	.text
	.align 16

.globl _nettle_aes128_encrypt_aesni
.type _nettle_aes128_encrypt_aesni,%function
_nettle_aes128_encrypt_aesni: 
	
  

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
	movups	112(%rdi), %xmm7
	movups	128(%rdi), %xmm8
	movups	144(%rdi), %xmm9
	movups	160(%rdi), %xmm10
	shr	%rsi
	jnc	.Lblock_loop

	movups	(%rcx), %xmm11
	pxor	%xmm0, %xmm11
	aesenc	%xmm1, %xmm11
	aesenc	%xmm2, %xmm11
	aesenc	%xmm3, %xmm11
	aesenc	%xmm4, %xmm11
	aesenc	%xmm5, %xmm11
	aesenc	%xmm6, %xmm11
	aesenc	%xmm7, %xmm11
	aesenc	%xmm8, %xmm11
	aesenc	%xmm9, %xmm11
	aesenclast %xmm10, %xmm11

	movups	%xmm11, (%rdx)
	add	$16, %rcx
	add	$16, %rdx
	test	%rsi, %rsi
	jz	.Lend

.Lblock_loop:
	movups	(%rcx), %xmm11
	movups	16(%rcx), %xmm12
	pxor	%xmm0, %xmm11
	pxor	%xmm0, %xmm12
	aesenc	%xmm1, %xmm11
	aesenc	%xmm1, %xmm12
	aesenc	%xmm2, %xmm11
	aesenc	%xmm2, %xmm12
	aesenc	%xmm3, %xmm11
	aesenc	%xmm3, %xmm12
	aesenc	%xmm4, %xmm11
	aesenc	%xmm4, %xmm12
	aesenc	%xmm5, %xmm11
	aesenc	%xmm5, %xmm12
	aesenc	%xmm6, %xmm11
	aesenc	%xmm6, %xmm12
	aesenc	%xmm7, %xmm11
	aesenc	%xmm7, %xmm12
	aesenc	%xmm8, %xmm11
	aesenc	%xmm8, %xmm12
	aesenc	%xmm9, %xmm11
	aesenc	%xmm9, %xmm12
	aesenclast %xmm10, %xmm11
	aesenclast %xmm10, %xmm12

	movups	%xmm11, (%rdx)
	movups	%xmm12, 16(%rdx)
	add	$32, %rcx
	add	$32, %rdx
	dec	%rsi
	jnz	.Lblock_loop

.Lend:
	
  

	ret
.size _nettle_aes128_encrypt_aesni, . - _nettle_aes128_encrypt_aesni



.section .note.GNU-stack,"",%progbits
