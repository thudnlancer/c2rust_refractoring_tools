



























































	.file "aes-encrypt-internal.asm"
	
	
	
	
	
	.text
	.align 16

.globl _nettle_aes_encrypt
.type _nettle_aes_encrypt,%function
_nettle_aes_encrypt: 
	
  

	test	%rcx, %rcx
	jz	.Lend

        
	push	%rbx
	push	%rbp
	push	%r12
	push	%r13
	push	%r14
	push	%r15	

	subl	$1, %edi
	push	%rdi		
	
	mov	%rdx, %r13
	mov	%rcx, %r14
	shr	$4, %r14
.Lblock_loop:
	mov	%rsi, %r15
	
	
	movl	(%r9),%eax
	movl	4(%r9),%ebx
	movl	8(%r9),%ecx
	movl	12(%r9),%edx
	
	xorl	(%r15),%eax
	xorl	4(%r15),%ebx
	xorl	8(%r15),%ecx
	xorl	12(%r15),%edx
	add	$16, %r9	

	movl	(%rsp), %edi

	add	$16, %r15	
	.align 16

.Lround_loop:
	
	movzb	%al, %rbp
	movl	256 (%r13, %rbp, 4),%r10d
	movzb	%bh, %ebp
	xorl	1280 (%r13, %rbp, 4),%r10d
	movl	%ecx,%ebp
	shr	$16,%rbp
	and	$0xff,%rbp
	xorl	2304 (%r13, %rbp, 4),%r10d
	movl	%edx,%ebp
	shr	$24,%rbp
	xorl	3328 (%r13, %rbp, 4),%r10d
	
	movzb	%bl, %rbp
	movl	256 (%r13, %rbp, 4),%r11d
	movzb	%ch, %ebp
	xorl	1280 (%r13, %rbp, 4),%r11d
	movl	%edx,%ebp
	shr	$16,%rbp
	and	$0xff,%rbp
	xorl	2304 (%r13, %rbp, 4),%r11d
	movl	%eax,%ebp
	shr	$24,%rbp
	xorl	3328 (%r13, %rbp, 4),%r11d
	
	movzb	%cl, %rbp
	movl	256 (%r13, %rbp, 4),%r12d
	movzb	%dh, %ebp
	xorl	1280 (%r13, %rbp, 4),%r12d
	movl	%eax,%ebp
	shr	$16,%rbp
	and	$0xff,%rbp
	xorl	2304 (%r13, %rbp, 4),%r12d
	movl	%ebx,%ebp
	shr	$24,%rbp
	xorl	3328 (%r13, %rbp, 4),%r12d
	
	movzb	%dl, %rbp
	movl	256 (%r13, %rbp, 4),%edx
	movzb	%ah, %ebp
	xorl	1280 (%r13, %rbp, 4),%edx
	movl	%ebx,%ebp
	shr	$16,%rbp
	and	$0xff,%rbp
	xorl	2304 (%r13, %rbp, 4),%edx
	movl	%ecx,%ebp
	shr	$24,%rbp
	xorl	3328 (%r13, %rbp, 4),%edx

	movl	%r10d, %eax
	movl	%r11d, %ebx
	movl	%r12d, %ecx

	xorl	(%r15),%eax	
	xorl	4(%r15),%ebx
	xorl	8(%r15),%ecx
	xorl	12(%r15),%edx

	add	$16, %r15	
	decl	%edi
	jnz	.Lround_loop

	
	
	movzb	%al,%rbp
	movzbl	(%r13, %rbp), %r10d
	movl	%ebx,%ebp
	andl	$0x0000ff00,%ebp
	orl	%ebp, %r10d
	movl	%ecx,%ebp
	andl	$0x00ff0000,%ebp
	orl	%ebp, %r10d
	movl	%edx,%ebp
	andl	$0xff000000,%ebp
	orl	%ebp, %r10d
	roll	$8, %r10d
	
	movzb	%bl,%rbp
	movzbl	(%r13, %rbp), %r11d
	movl	%ecx,%ebp
	andl	$0x0000ff00,%ebp
	orl	%ebp, %r11d
	movl	%edx,%ebp
	andl	$0x00ff0000,%ebp
	orl	%ebp, %r11d
	movl	%eax,%ebp
	andl	$0xff000000,%ebp
	orl	%ebp, %r11d
	roll	$8, %r11d
	
	movzb	%cl,%rbp
	movzbl	(%r13, %rbp), %r12d
	movl	%edx,%ebp
	andl	$0x0000ff00,%ebp
	orl	%ebp, %r12d
	movl	%eax,%ebp
	andl	$0x00ff0000,%ebp
	orl	%ebp, %r12d
	movl	%ebx,%ebp
	andl	$0xff000000,%ebp
	orl	%ebp, %r12d
	roll	$8, %r12d
	
	movzb	%dl,%rbp
	movzbl	(%r13, %rbp), %edx
	movl	%eax,%ebp
	andl	$0x0000ff00,%ebp
	orl	%ebp, %edx
	movl	%ebx,%ebp
	andl	$0x00ff0000,%ebp
	orl	%ebp, %edx
	movl	%ecx,%ebp
	andl	$0xff000000,%ebp
	orl	%ebp, %edx
	roll	$8, %edx

	
	mov	$3, %edi
.Lsubst:
	
	movzb	%r10b,%rbp
	movb	(%r13, %rbp),%r10b
	roll	$8,%r10d

	movzb  %r11b,%rbp
	movb	(%r13, %rbp),%r11b
	roll	$8,%r11d

	movzb  %r12b,%rbp
	movb	(%r13, %rbp),%r12b
	roll	$8,%r12d

	movzb  %dl,%rbp
	movb	(%r13, %rbp),%dl
	roll	$8,%edx

	decl	%edi
	jnz	.Lsubst

	
	
	xorl	(%r15),%r10d
	xorl	4(%r15),%r11d
	xorl	8(%r15),%r12d
	xorl	12(%r15),%edx

	movl	%r10d,(%r8)
	movl	%r11d,4(%r8)
	movl	%r12d,8(%r8)
	movl	%edx,12(%r8)
	
	add	$16, %r8
	dec	%r14

	jnz	.Lblock_loop

	lea	8(%rsp), %rsp	
	pop	%r15
	pop	%r14
	pop	%r13
	pop	%r12
	pop	%rbp
	pop	%rbx
.Lend:
	
  

	ret
.size _nettle_aes_encrypt, . - _nettle_aes_encrypt


.section .note.GNU-stack,"",%progbits
