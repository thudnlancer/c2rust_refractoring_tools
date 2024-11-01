








































    

.globl _nettle_ghash_set_key_pclmul
.type _nettle_ghash_set_key_pclmul,%function
_nettle_ghash_set_key_pclmul: 
	
  

	movdqa	.Lpolynomial(%rip), %xmm0
	movdqa	.Lbswap(%rip), %xmm1
	movups	(%rsi), %xmm2
	pshufb	%xmm1, %xmm2
	
	movdqa	%xmm2, %xmm4
	psllq	$1, %xmm4
	psrlq	$63, %xmm2		
	pshufd	$0xaa, %xmm2, %xmm7	
	pslldq	$8, %xmm2		
	por	%xmm4, %xmm2
	pxor	%xmm4, %xmm4
	psubd	%xmm7, %xmm4		
	pand	%xmm0, %xmm4
	pxor	%xmm4, %xmm2
	movups	%xmm2, (%rdi)

	
	movdqa	%xmm2, %xmm4
	pshufd	$0x4e, %xmm2, %xmm3	
	pclmullqhqdq %xmm0, %xmm4
	pxor	%xmm4, %xmm3
	movups	%xmm3, 16(%rdi)

	movdqa		%xmm2, %xmm6
	movdqa		%xmm2, %xmm7
	movdqa		%xmm2, %xmm4
	pclmulhqlqdq	%xmm2, %xmm4	
	pclmulhqhqdq	%xmm2, %xmm6	
	pclmullqlqdq	%xmm3, %xmm7 	
	pclmullqhqdq	%xmm3, %xmm2	
	pxor		%xmm4, %xmm7
	pxor		%xmm6, %xmm2

	pshufd		$0x4e, %xmm7, %xmm4		
	pxor		%xmm4, %xmm2
	pclmullqhqdq	%xmm0, %xmm7
	pxor		%xmm7, %xmm2
	movups	%xmm2, 32(%rdi)

	
	pshufd	$0x4e, %xmm2, %xmm3	
	pclmullqhqdq %xmm0, %xmm2
	pxor	%xmm2, %xmm3
	movups	%xmm3, 48(%rdi)

	
  

	ret
.size _nettle_ghash_set_key_pclmul, . - _nettle_ghash_set_key_pclmul

	.section .rodata
	
	
	
	
	
	.align 16

.Lpolynomial:
	.byte 1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0xC2
.Lbswap:
	.byte 15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0



.section .note.GNU-stack,"",%progbits
