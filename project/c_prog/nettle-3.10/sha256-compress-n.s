

























	.file "sha256-compress-n.asm"
















	





















	
	
	

	.text
	.align 16


.globl _nettle_sha256_compress_n_x86_64
.type _nettle_sha256_compress_n_x86_64,%function
_nettle_sha256_compress_n_x86_64: 
	
  

	test	%rdx, %rdx
	jz	.Lend

	sub	$120, %rsp
	mov	%rdi, 64(%rsp)	
	mov	%rbx, 72(%rsp)
	mov	%rbp, 80(%rsp)
	mov	%r12, 88(%rsp)
	mov	%r13, 96(%rsp)
	mov	%r14, 104(%rsp)
	mov	%r15, 112(%rsp)

	movl	(%rdi),   %eax
	movl	4(%rdi),  %ebx
	movl	8(%rdi),  %ebp
	movl	12(%rdi), %r8d
	movl	16(%rdi), %r9d
	movl	20(%rdi), %r10d
	movl	24(%rdi), %r11d
	movl	28(%rdi), %r12d

.Loop_block:
	xorl	%edi, %edi
	.align 16


.Loop1:
	
	movl	(%rcx, %rdi, 4), %r15d
	bswapl	%r15d
	movl	%r15d, (%rsp, %rdi, 4)
 
	movl	%r9d, %r13d
	movl	%r9d, %r14d
	roll	$7, %r13d
	roll	$21, %r14d
	xorl	%r13d, %r14d
	roll	$19, %r13d
	xorl	%r13d, %r14d
	addl	%r15d, %r12d
	addl	%r14d, %r12d
	movl	%r11d, %r13d
	xorl	%r10d, %r13d
	andl	%r9d, %r13d
	xorl	%r11d, %r13d
	addl	(%rsi,%rdi,4), %r12d
	addl	%r13d, %r12d
	addl	%r12d, %r8d

	movl	%eax, %r13d
	movl	%eax, %r14d
	roll	$10, %r13d
	roll	$19, %r14d
	xorl	%r13d, %r14d
	roll	$20, %r13d
	xorl	%r13d, %r14d
	addl	%r14d, %r12d
	movl	%eax, %r13d
	movl	%eax, %r14d
	andl	%ebx, %r13d
	xorl	%ebx, %r14d
	addl	%r13d, %r12d
	andl	%ebp, %r14d
	addl	%r14d, %r12d

	
	movl	4(%rcx, %rdi, 4), %r15d
	bswapl	%r15d
	movl	%r15d, 4(%rsp, %rdi, 4)
 
	movl	%r8d, %r13d
	movl	%r8d, %r14d
	roll	$7, %r13d
	roll	$21, %r14d
	xorl	%r13d, %r14d
	roll	$19, %r13d
	xorl	%r13d, %r14d
	addl	%r15d, %r11d
	addl	%r14d, %r11d
	movl	%r10d, %r13d
	xorl	%r9d, %r13d
	andl	%r8d, %r13d
	xorl	%r10d, %r13d
	addl	4(%rsi,%rdi,4), %r11d
	addl	%r13d, %r11d
	addl	%r11d, %ebp

	movl	%r12d, %r13d
	movl	%r12d, %r14d
	roll	$10, %r13d
	roll	$19, %r14d
	xorl	%r13d, %r14d
	roll	$20, %r13d
	xorl	%r13d, %r14d
	addl	%r14d, %r11d
	movl	%r12d, %r13d
	movl	%r12d, %r14d
	andl	%eax, %r13d
	xorl	%eax, %r14d
	addl	%r13d, %r11d
	andl	%ebx, %r14d
	addl	%r14d, %r11d

	
	movl	8(%rcx, %rdi, 4), %r15d
	bswapl	%r15d
	movl	%r15d, 8(%rsp, %rdi, 4)
 
	movl	%ebp, %r13d
	movl	%ebp, %r14d
	roll	$7, %r13d
	roll	$21, %r14d
	xorl	%r13d, %r14d
	roll	$19, %r13d
	xorl	%r13d, %r14d
	addl	%r15d, %r10d
	addl	%r14d, %r10d
	movl	%r9d, %r13d
	xorl	%r8d, %r13d
	andl	%ebp, %r13d
	xorl	%r9d, %r13d
	addl	8(%rsi,%rdi,4), %r10d
	addl	%r13d, %r10d
	addl	%r10d, %ebx

	movl	%r11d, %r13d
	movl	%r11d, %r14d
	roll	$10, %r13d
	roll	$19, %r14d
	xorl	%r13d, %r14d
	roll	$20, %r13d
	xorl	%r13d, %r14d
	addl	%r14d, %r10d
	movl	%r11d, %r13d
	movl	%r11d, %r14d
	andl	%r12d, %r13d
	xorl	%r12d, %r14d
	addl	%r13d, %r10d
	andl	%eax, %r14d
	addl	%r14d, %r10d

	
	movl	12(%rcx, %rdi, 4), %r15d
	bswapl	%r15d
	movl	%r15d, 12(%rsp, %rdi, 4)
 
	movl	%ebx, %r13d
	movl	%ebx, %r14d
	roll	$7, %r13d
	roll	$21, %r14d
	xorl	%r13d, %r14d
	roll	$19, %r13d
	xorl	%r13d, %r14d
	addl	%r15d, %r9d
	addl	%r14d, %r9d
	movl	%r8d, %r13d
	xorl	%ebp, %r13d
	andl	%ebx, %r13d
	xorl	%r8d, %r13d
	addl	12(%rsi,%rdi,4), %r9d
	addl	%r13d, %r9d
	addl	%r9d, %eax

	movl	%r10d, %r13d
	movl	%r10d, %r14d
	roll	$10, %r13d
	roll	$19, %r14d
	xorl	%r13d, %r14d
	roll	$20, %r13d
	xorl	%r13d, %r14d
	addl	%r14d, %r9d
	movl	%r10d, %r13d
	movl	%r10d, %r14d
	andl	%r11d, %r13d
	xorl	%r11d, %r14d
	addl	%r13d, %r9d
	andl	%r12d, %r14d
	addl	%r14d, %r9d

	
	movl	16(%rcx, %rdi, 4), %r15d
	bswapl	%r15d
	movl	%r15d, 16(%rsp, %rdi, 4)
 
	movl	%eax, %r13d
	movl	%eax, %r14d
	roll	$7, %r13d
	roll	$21, %r14d
	xorl	%r13d, %r14d
	roll	$19, %r13d
	xorl	%r13d, %r14d
	addl	%r15d, %r8d
	addl	%r14d, %r8d
	movl	%ebp, %r13d
	xorl	%ebx, %r13d
	andl	%eax, %r13d
	xorl	%ebp, %r13d
	addl	16(%rsi,%rdi,4), %r8d
	addl	%r13d, %r8d
	addl	%r8d, %r12d

	movl	%r9d, %r13d
	movl	%r9d, %r14d
	roll	$10, %r13d
	roll	$19, %r14d
	xorl	%r13d, %r14d
	roll	$20, %r13d
	xorl	%r13d, %r14d
	addl	%r14d, %r8d
	movl	%r9d, %r13d
	movl	%r9d, %r14d
	andl	%r10d, %r13d
	xorl	%r10d, %r14d
	addl	%r13d, %r8d
	andl	%r11d, %r14d
	addl	%r14d, %r8d

	
	movl	20(%rcx, %rdi, 4), %r15d
	bswapl	%r15d
	movl	%r15d, 20(%rsp, %rdi, 4)
 
	movl	%r12d, %r13d
	movl	%r12d, %r14d
	roll	$7, %r13d
	roll	$21, %r14d
	xorl	%r13d, %r14d
	roll	$19, %r13d
	xorl	%r13d, %r14d
	addl	%r15d, %ebp
	addl	%r14d, %ebp
	movl	%ebx, %r13d
	xorl	%eax, %r13d
	andl	%r12d, %r13d
	xorl	%ebx, %r13d
	addl	20(%rsi,%rdi,4), %ebp
	addl	%r13d, %ebp
	addl	%ebp, %r11d

	movl	%r8d, %r13d
	movl	%r8d, %r14d
	roll	$10, %r13d
	roll	$19, %r14d
	xorl	%r13d, %r14d
	roll	$20, %r13d
	xorl	%r13d, %r14d
	addl	%r14d, %ebp
	movl	%r8d, %r13d
	movl	%r8d, %r14d
	andl	%r9d, %r13d
	xorl	%r9d, %r14d
	addl	%r13d, %ebp
	andl	%r10d, %r14d
	addl	%r14d, %ebp

	
	movl	24(%rcx, %rdi, 4), %r15d
	bswapl	%r15d
	movl	%r15d, 24(%rsp, %rdi, 4)
 
	movl	%r11d, %r13d
	movl	%r11d, %r14d
	roll	$7, %r13d
	roll	$21, %r14d
	xorl	%r13d, %r14d
	roll	$19, %r13d
	xorl	%r13d, %r14d
	addl	%r15d, %ebx
	addl	%r14d, %ebx
	movl	%eax, %r13d
	xorl	%r12d, %r13d
	andl	%r11d, %r13d
	xorl	%eax, %r13d
	addl	24(%rsi,%rdi,4), %ebx
	addl	%r13d, %ebx
	addl	%ebx, %r10d

	movl	%ebp, %r13d
	movl	%ebp, %r14d
	roll	$10, %r13d
	roll	$19, %r14d
	xorl	%r13d, %r14d
	roll	$20, %r13d
	xorl	%r13d, %r14d
	addl	%r14d, %ebx
	movl	%ebp, %r13d
	movl	%ebp, %r14d
	andl	%r8d, %r13d
	xorl	%r8d, %r14d
	addl	%r13d, %ebx
	andl	%r9d, %r14d
	addl	%r14d, %ebx

	
	movl	28(%rcx, %rdi, 4), %r15d
	bswapl	%r15d
	movl	%r15d, 28(%rsp, %rdi, 4)
 
	movl	%r10d, %r13d
	movl	%r10d, %r14d
	roll	$7, %r13d
	roll	$21, %r14d
	xorl	%r13d, %r14d
	roll	$19, %r13d
	xorl	%r13d, %r14d
	addl	%r15d, %eax
	addl	%r14d, %eax
	movl	%r12d, %r13d
	xorl	%r11d, %r13d
	andl	%r10d, %r13d
	xorl	%r12d, %r13d
	addl	28(%rsi,%rdi,4), %eax
	addl	%r13d, %eax
	addl	%eax, %r9d

	movl	%ebx, %r13d
	movl	%ebx, %r14d
	roll	$10, %r13d
	roll	$19, %r14d
	xorl	%r13d, %r14d
	roll	$20, %r13d
	xorl	%r13d, %r14d
	addl	%r14d, %eax
	movl	%ebx, %r13d
	movl	%ebx, %r14d
	andl	%ebp, %r13d
	xorl	%ebp, %r14d
	addl	%r13d, %eax
	andl	%r8d, %r14d
	addl	%r14d, %eax

	addl	$8, %edi
	cmpl	$16, %edi
	jne	.Loop1

.Loop2:
	
	movl	(%rsp), %r15d
	movl	56(%rsp), %r13d
	movl	%r13d, %r14d
	shrl	$10, %r13d
	roll	$13, %r14d
	xorl	%r14d, %r13d
	roll	$2, %r14d
	xorl	%r14d, %r13d
	addl	%r13d, %r15d
	movl	4(%rsp), %r13d
	movl	%r13d, %r14d
	shrl	$3, %r13d
	roll	$14, %r14d
	xorl	%r14d, %r13d
	roll	$11, %r14d
	xorl	%r14d, %r13d
	addl	%r13d, %r15d
	addl	36(%rsp), %r15d
	movl	%r15d, (%rsp)
 
	movl	%r9d, %r13d
	movl	%r9d, %r14d
	roll	$7, %r13d
	roll	$21, %r14d
	xorl	%r13d, %r14d
	roll	$19, %r13d
	xorl	%r13d, %r14d
	addl	%r15d, %r12d
	addl	%r14d, %r12d
	movl	%r11d, %r13d
	xorl	%r10d, %r13d
	andl	%r9d, %r13d
	xorl	%r11d, %r13d
	addl	(%rsi,%rdi,4), %r12d
	addl	%r13d, %r12d
	addl	%r12d, %r8d

	movl	%eax, %r13d
	movl	%eax, %r14d
	roll	$10, %r13d
	roll	$19, %r14d
	xorl	%r13d, %r14d
	roll	$20, %r13d
	xorl	%r13d, %r14d
	addl	%r14d, %r12d
	movl	%eax, %r13d
	movl	%eax, %r14d
	andl	%ebx, %r13d
	xorl	%ebx, %r14d
	addl	%r13d, %r12d
	andl	%ebp, %r14d
	addl	%r14d, %r12d

	
	movl	4(%rsp), %r15d
	movl	60(%rsp), %r13d
	movl	%r13d, %r14d
	shrl	$10, %r13d
	roll	$13, %r14d
	xorl	%r14d, %r13d
	roll	$2, %r14d
	xorl	%r14d, %r13d
	addl	%r13d, %r15d
	movl	8(%rsp), %r13d
	movl	%r13d, %r14d
	shrl	$3, %r13d
	roll	$14, %r14d
	xorl	%r14d, %r13d
	roll	$11, %r14d
	xorl	%r14d, %r13d
	addl	%r13d, %r15d
	addl	40(%rsp), %r15d
	movl	%r15d, 4(%rsp)
 
	movl	%r8d, %r13d
	movl	%r8d, %r14d
	roll	$7, %r13d
	roll	$21, %r14d
	xorl	%r13d, %r14d
	roll	$19, %r13d
	xorl	%r13d, %r14d
	addl	%r15d, %r11d
	addl	%r14d, %r11d
	movl	%r10d, %r13d
	xorl	%r9d, %r13d
	andl	%r8d, %r13d
	xorl	%r10d, %r13d
	addl	4(%rsi,%rdi,4), %r11d
	addl	%r13d, %r11d
	addl	%r11d, %ebp

	movl	%r12d, %r13d
	movl	%r12d, %r14d
	roll	$10, %r13d
	roll	$19, %r14d
	xorl	%r13d, %r14d
	roll	$20, %r13d
	xorl	%r13d, %r14d
	addl	%r14d, %r11d
	movl	%r12d, %r13d
	movl	%r12d, %r14d
	andl	%eax, %r13d
	xorl	%eax, %r14d
	addl	%r13d, %r11d
	andl	%ebx, %r14d
	addl	%r14d, %r11d

	
	movl	8(%rsp), %r15d
	movl	(%rsp), %r13d
	movl	%r13d, %r14d
	shrl	$10, %r13d
	roll	$13, %r14d
	xorl	%r14d, %r13d
	roll	$2, %r14d
	xorl	%r14d, %r13d
	addl	%r13d, %r15d
	movl	12(%rsp), %r13d
	movl	%r13d, %r14d
	shrl	$3, %r13d
	roll	$14, %r14d
	xorl	%r14d, %r13d
	roll	$11, %r14d
	xorl	%r14d, %r13d
	addl	%r13d, %r15d
	addl	44(%rsp), %r15d
	movl	%r15d, 8(%rsp)
 
	movl	%ebp, %r13d
	movl	%ebp, %r14d
	roll	$7, %r13d
	roll	$21, %r14d
	xorl	%r13d, %r14d
	roll	$19, %r13d
	xorl	%r13d, %r14d
	addl	%r15d, %r10d
	addl	%r14d, %r10d
	movl	%r9d, %r13d
	xorl	%r8d, %r13d
	andl	%ebp, %r13d
	xorl	%r9d, %r13d
	addl	8(%rsi,%rdi,4), %r10d
	addl	%r13d, %r10d
	addl	%r10d, %ebx

	movl	%r11d, %r13d
	movl	%r11d, %r14d
	roll	$10, %r13d
	roll	$19, %r14d
	xorl	%r13d, %r14d
	roll	$20, %r13d
	xorl	%r13d, %r14d
	addl	%r14d, %r10d
	movl	%r11d, %r13d
	movl	%r11d, %r14d
	andl	%r12d, %r13d
	xorl	%r12d, %r14d
	addl	%r13d, %r10d
	andl	%eax, %r14d
	addl	%r14d, %r10d

	
	movl	12(%rsp), %r15d
	movl	4(%rsp), %r13d
	movl	%r13d, %r14d
	shrl	$10, %r13d
	roll	$13, %r14d
	xorl	%r14d, %r13d
	roll	$2, %r14d
	xorl	%r14d, %r13d
	addl	%r13d, %r15d
	movl	16(%rsp), %r13d
	movl	%r13d, %r14d
	shrl	$3, %r13d
	roll	$14, %r14d
	xorl	%r14d, %r13d
	roll	$11, %r14d
	xorl	%r14d, %r13d
	addl	%r13d, %r15d
	addl	48(%rsp), %r15d
	movl	%r15d, 12(%rsp)
 
	movl	%ebx, %r13d
	movl	%ebx, %r14d
	roll	$7, %r13d
	roll	$21, %r14d
	xorl	%r13d, %r14d
	roll	$19, %r13d
	xorl	%r13d, %r14d
	addl	%r15d, %r9d
	addl	%r14d, %r9d
	movl	%r8d, %r13d
	xorl	%ebp, %r13d
	andl	%ebx, %r13d
	xorl	%r8d, %r13d
	addl	12(%rsi,%rdi,4), %r9d
	addl	%r13d, %r9d
	addl	%r9d, %eax

	movl	%r10d, %r13d
	movl	%r10d, %r14d
	roll	$10, %r13d
	roll	$19, %r14d
	xorl	%r13d, %r14d
	roll	$20, %r13d
	xorl	%r13d, %r14d
	addl	%r14d, %r9d
	movl	%r10d, %r13d
	movl	%r10d, %r14d
	andl	%r11d, %r13d
	xorl	%r11d, %r14d
	addl	%r13d, %r9d
	andl	%r12d, %r14d
	addl	%r14d, %r9d

	
	movl	16(%rsp), %r15d
	movl	8(%rsp), %r13d
	movl	%r13d, %r14d
	shrl	$10, %r13d
	roll	$13, %r14d
	xorl	%r14d, %r13d
	roll	$2, %r14d
	xorl	%r14d, %r13d
	addl	%r13d, %r15d
	movl	20(%rsp), %r13d
	movl	%r13d, %r14d
	shrl	$3, %r13d
	roll	$14, %r14d
	xorl	%r14d, %r13d
	roll	$11, %r14d
	xorl	%r14d, %r13d
	addl	%r13d, %r15d
	addl	52(%rsp), %r15d
	movl	%r15d, 16(%rsp)
 
	movl	%eax, %r13d
	movl	%eax, %r14d
	roll	$7, %r13d
	roll	$21, %r14d
	xorl	%r13d, %r14d
	roll	$19, %r13d
	xorl	%r13d, %r14d
	addl	%r15d, %r8d
	addl	%r14d, %r8d
	movl	%ebp, %r13d
	xorl	%ebx, %r13d
	andl	%eax, %r13d
	xorl	%ebp, %r13d
	addl	16(%rsi,%rdi,4), %r8d
	addl	%r13d, %r8d
	addl	%r8d, %r12d

	movl	%r9d, %r13d
	movl	%r9d, %r14d
	roll	$10, %r13d
	roll	$19, %r14d
	xorl	%r13d, %r14d
	roll	$20, %r13d
	xorl	%r13d, %r14d
	addl	%r14d, %r8d
	movl	%r9d, %r13d
	movl	%r9d, %r14d
	andl	%r10d, %r13d
	xorl	%r10d, %r14d
	addl	%r13d, %r8d
	andl	%r11d, %r14d
	addl	%r14d, %r8d

	
	movl	20(%rsp), %r15d
	movl	12(%rsp), %r13d
	movl	%r13d, %r14d
	shrl	$10, %r13d
	roll	$13, %r14d
	xorl	%r14d, %r13d
	roll	$2, %r14d
	xorl	%r14d, %r13d
	addl	%r13d, %r15d
	movl	24(%rsp), %r13d
	movl	%r13d, %r14d
	shrl	$3, %r13d
	roll	$14, %r14d
	xorl	%r14d, %r13d
	roll	$11, %r14d
	xorl	%r14d, %r13d
	addl	%r13d, %r15d
	addl	56(%rsp), %r15d
	movl	%r15d, 20(%rsp)
 
	movl	%r12d, %r13d
	movl	%r12d, %r14d
	roll	$7, %r13d
	roll	$21, %r14d
	xorl	%r13d, %r14d
	roll	$19, %r13d
	xorl	%r13d, %r14d
	addl	%r15d, %ebp
	addl	%r14d, %ebp
	movl	%ebx, %r13d
	xorl	%eax, %r13d
	andl	%r12d, %r13d
	xorl	%ebx, %r13d
	addl	20(%rsi,%rdi,4), %ebp
	addl	%r13d, %ebp
	addl	%ebp, %r11d

	movl	%r8d, %r13d
	movl	%r8d, %r14d
	roll	$10, %r13d
	roll	$19, %r14d
	xorl	%r13d, %r14d
	roll	$20, %r13d
	xorl	%r13d, %r14d
	addl	%r14d, %ebp
	movl	%r8d, %r13d
	movl	%r8d, %r14d
	andl	%r9d, %r13d
	xorl	%r9d, %r14d
	addl	%r13d, %ebp
	andl	%r10d, %r14d
	addl	%r14d, %ebp

	
	movl	24(%rsp), %r15d
	movl	16(%rsp), %r13d
	movl	%r13d, %r14d
	shrl	$10, %r13d
	roll	$13, %r14d
	xorl	%r14d, %r13d
	roll	$2, %r14d
	xorl	%r14d, %r13d
	addl	%r13d, %r15d
	movl	28(%rsp), %r13d
	movl	%r13d, %r14d
	shrl	$3, %r13d
	roll	$14, %r14d
	xorl	%r14d, %r13d
	roll	$11, %r14d
	xorl	%r14d, %r13d
	addl	%r13d, %r15d
	addl	60(%rsp), %r15d
	movl	%r15d, 24(%rsp)
 
	movl	%r11d, %r13d
	movl	%r11d, %r14d
	roll	$7, %r13d
	roll	$21, %r14d
	xorl	%r13d, %r14d
	roll	$19, %r13d
	xorl	%r13d, %r14d
	addl	%r15d, %ebx
	addl	%r14d, %ebx
	movl	%eax, %r13d
	xorl	%r12d, %r13d
	andl	%r11d, %r13d
	xorl	%eax, %r13d
	addl	24(%rsi,%rdi,4), %ebx
	addl	%r13d, %ebx
	addl	%ebx, %r10d

	movl	%ebp, %r13d
	movl	%ebp, %r14d
	roll	$10, %r13d
	roll	$19, %r14d
	xorl	%r13d, %r14d
	roll	$20, %r13d
	xorl	%r13d, %r14d
	addl	%r14d, %ebx
	movl	%ebp, %r13d
	movl	%ebp, %r14d
	andl	%r8d, %r13d
	xorl	%r8d, %r14d
	addl	%r13d, %ebx
	andl	%r9d, %r14d
	addl	%r14d, %ebx

	
	movl	28(%rsp), %r15d
	movl	20(%rsp), %r13d
	movl	%r13d, %r14d
	shrl	$10, %r13d
	roll	$13, %r14d
	xorl	%r14d, %r13d
	roll	$2, %r14d
	xorl	%r14d, %r13d
	addl	%r13d, %r15d
	movl	32(%rsp), %r13d
	movl	%r13d, %r14d
	shrl	$3, %r13d
	roll	$14, %r14d
	xorl	%r14d, %r13d
	roll	$11, %r14d
	xorl	%r14d, %r13d
	addl	%r13d, %r15d
	addl	(%rsp), %r15d
	movl	%r15d, 28(%rsp)
 
	movl	%r10d, %r13d
	movl	%r10d, %r14d
	roll	$7, %r13d
	roll	$21, %r14d
	xorl	%r13d, %r14d
	roll	$19, %r13d
	xorl	%r13d, %r14d
	addl	%r15d, %eax
	addl	%r14d, %eax
	movl	%r12d, %r13d
	xorl	%r11d, %r13d
	andl	%r10d, %r13d
	xorl	%r12d, %r13d
	addl	28(%rsi,%rdi,4), %eax
	addl	%r13d, %eax
	addl	%eax, %r9d

	movl	%ebx, %r13d
	movl	%ebx, %r14d
	roll	$10, %r13d
	roll	$19, %r14d
	xorl	%r13d, %r14d
	roll	$20, %r13d
	xorl	%r13d, %r14d
	addl	%r14d, %eax
	movl	%ebx, %r13d
	movl	%ebx, %r14d
	andl	%ebp, %r13d
	xorl	%ebp, %r14d
	addl	%r13d, %eax
	andl	%r8d, %r14d
	addl	%r14d, %eax

	
	movl	32(%rsp), %r15d
	movl	24(%rsp), %r13d
	movl	%r13d, %r14d
	shrl	$10, %r13d
	roll	$13, %r14d
	xorl	%r14d, %r13d
	roll	$2, %r14d
	xorl	%r14d, %r13d
	addl	%r13d, %r15d
	movl	36(%rsp), %r13d
	movl	%r13d, %r14d
	shrl	$3, %r13d
	roll	$14, %r14d
	xorl	%r14d, %r13d
	roll	$11, %r14d
	xorl	%r14d, %r13d
	addl	%r13d, %r15d
	addl	4(%rsp), %r15d
	movl	%r15d, 32(%rsp)
 
	movl	%r9d, %r13d
	movl	%r9d, %r14d
	roll	$7, %r13d
	roll	$21, %r14d
	xorl	%r13d, %r14d
	roll	$19, %r13d
	xorl	%r13d, %r14d
	addl	%r15d, %r12d
	addl	%r14d, %r12d
	movl	%r11d, %r13d
	xorl	%r10d, %r13d
	andl	%r9d, %r13d
	xorl	%r11d, %r13d
	addl	32(%rsi,%rdi,4), %r12d
	addl	%r13d, %r12d
	addl	%r12d, %r8d

	movl	%eax, %r13d
	movl	%eax, %r14d
	roll	$10, %r13d
	roll	$19, %r14d
	xorl	%r13d, %r14d
	roll	$20, %r13d
	xorl	%r13d, %r14d
	addl	%r14d, %r12d
	movl	%eax, %r13d
	movl	%eax, %r14d
	andl	%ebx, %r13d
	xorl	%ebx, %r14d
	addl	%r13d, %r12d
	andl	%ebp, %r14d
	addl	%r14d, %r12d

	
	movl	36(%rsp), %r15d
	movl	28(%rsp), %r13d
	movl	%r13d, %r14d
	shrl	$10, %r13d
	roll	$13, %r14d
	xorl	%r14d, %r13d
	roll	$2, %r14d
	xorl	%r14d, %r13d
	addl	%r13d, %r15d
	movl	40(%rsp), %r13d
	movl	%r13d, %r14d
	shrl	$3, %r13d
	roll	$14, %r14d
	xorl	%r14d, %r13d
	roll	$11, %r14d
	xorl	%r14d, %r13d
	addl	%r13d, %r15d
	addl	8(%rsp), %r15d
	movl	%r15d, 36(%rsp)
 
	movl	%r8d, %r13d
	movl	%r8d, %r14d
	roll	$7, %r13d
	roll	$21, %r14d
	xorl	%r13d, %r14d
	roll	$19, %r13d
	xorl	%r13d, %r14d
	addl	%r15d, %r11d
	addl	%r14d, %r11d
	movl	%r10d, %r13d
	xorl	%r9d, %r13d
	andl	%r8d, %r13d
	xorl	%r10d, %r13d
	addl	36(%rsi,%rdi,4), %r11d
	addl	%r13d, %r11d
	addl	%r11d, %ebp

	movl	%r12d, %r13d
	movl	%r12d, %r14d
	roll	$10, %r13d
	roll	$19, %r14d
	xorl	%r13d, %r14d
	roll	$20, %r13d
	xorl	%r13d, %r14d
	addl	%r14d, %r11d
	movl	%r12d, %r13d
	movl	%r12d, %r14d
	andl	%eax, %r13d
	xorl	%eax, %r14d
	addl	%r13d, %r11d
	andl	%ebx, %r14d
	addl	%r14d, %r11d

	
	movl	40(%rsp), %r15d
	movl	32(%rsp), %r13d
	movl	%r13d, %r14d
	shrl	$10, %r13d
	roll	$13, %r14d
	xorl	%r14d, %r13d
	roll	$2, %r14d
	xorl	%r14d, %r13d
	addl	%r13d, %r15d
	movl	44(%rsp), %r13d
	movl	%r13d, %r14d
	shrl	$3, %r13d
	roll	$14, %r14d
	xorl	%r14d, %r13d
	roll	$11, %r14d
	xorl	%r14d, %r13d
	addl	%r13d, %r15d
	addl	12(%rsp), %r15d
	movl	%r15d, 40(%rsp)
 
	movl	%ebp, %r13d
	movl	%ebp, %r14d
	roll	$7, %r13d
	roll	$21, %r14d
	xorl	%r13d, %r14d
	roll	$19, %r13d
	xorl	%r13d, %r14d
	addl	%r15d, %r10d
	addl	%r14d, %r10d
	movl	%r9d, %r13d
	xorl	%r8d, %r13d
	andl	%ebp, %r13d
	xorl	%r9d, %r13d
	addl	40(%rsi,%rdi,4), %r10d
	addl	%r13d, %r10d
	addl	%r10d, %ebx

	movl	%r11d, %r13d
	movl	%r11d, %r14d
	roll	$10, %r13d
	roll	$19, %r14d
	xorl	%r13d, %r14d
	roll	$20, %r13d
	xorl	%r13d, %r14d
	addl	%r14d, %r10d
	movl	%r11d, %r13d
	movl	%r11d, %r14d
	andl	%r12d, %r13d
	xorl	%r12d, %r14d
	addl	%r13d, %r10d
	andl	%eax, %r14d
	addl	%r14d, %r10d

	
	movl	44(%rsp), %r15d
	movl	36(%rsp), %r13d
	movl	%r13d, %r14d
	shrl	$10, %r13d
	roll	$13, %r14d
	xorl	%r14d, %r13d
	roll	$2, %r14d
	xorl	%r14d, %r13d
	addl	%r13d, %r15d
	movl	48(%rsp), %r13d
	movl	%r13d, %r14d
	shrl	$3, %r13d
	roll	$14, %r14d
	xorl	%r14d, %r13d
	roll	$11, %r14d
	xorl	%r14d, %r13d
	addl	%r13d, %r15d
	addl	16(%rsp), %r15d
	movl	%r15d, 44(%rsp)
 
	movl	%ebx, %r13d
	movl	%ebx, %r14d
	roll	$7, %r13d
	roll	$21, %r14d
	xorl	%r13d, %r14d
	roll	$19, %r13d
	xorl	%r13d, %r14d
	addl	%r15d, %r9d
	addl	%r14d, %r9d
	movl	%r8d, %r13d
	xorl	%ebp, %r13d
	andl	%ebx, %r13d
	xorl	%r8d, %r13d
	addl	44(%rsi,%rdi,4), %r9d
	addl	%r13d, %r9d
	addl	%r9d, %eax

	movl	%r10d, %r13d
	movl	%r10d, %r14d
	roll	$10, %r13d
	roll	$19, %r14d
	xorl	%r13d, %r14d
	roll	$20, %r13d
	xorl	%r13d, %r14d
	addl	%r14d, %r9d
	movl	%r10d, %r13d
	movl	%r10d, %r14d
	andl	%r11d, %r13d
	xorl	%r11d, %r14d
	addl	%r13d, %r9d
	andl	%r12d, %r14d
	addl	%r14d, %r9d

	
	movl	48(%rsp), %r15d
	movl	40(%rsp), %r13d
	movl	%r13d, %r14d
	shrl	$10, %r13d
	roll	$13, %r14d
	xorl	%r14d, %r13d
	roll	$2, %r14d
	xorl	%r14d, %r13d
	addl	%r13d, %r15d
	movl	52(%rsp), %r13d
	movl	%r13d, %r14d
	shrl	$3, %r13d
	roll	$14, %r14d
	xorl	%r14d, %r13d
	roll	$11, %r14d
	xorl	%r14d, %r13d
	addl	%r13d, %r15d
	addl	20(%rsp), %r15d
	movl	%r15d, 48(%rsp)
 
	movl	%eax, %r13d
	movl	%eax, %r14d
	roll	$7, %r13d
	roll	$21, %r14d
	xorl	%r13d, %r14d
	roll	$19, %r13d
	xorl	%r13d, %r14d
	addl	%r15d, %r8d
	addl	%r14d, %r8d
	movl	%ebp, %r13d
	xorl	%ebx, %r13d
	andl	%eax, %r13d
	xorl	%ebp, %r13d
	addl	48(%rsi,%rdi,4), %r8d
	addl	%r13d, %r8d
	addl	%r8d, %r12d

	movl	%r9d, %r13d
	movl	%r9d, %r14d
	roll	$10, %r13d
	roll	$19, %r14d
	xorl	%r13d, %r14d
	roll	$20, %r13d
	xorl	%r13d, %r14d
	addl	%r14d, %r8d
	movl	%r9d, %r13d
	movl	%r9d, %r14d
	andl	%r10d, %r13d
	xorl	%r10d, %r14d
	addl	%r13d, %r8d
	andl	%r11d, %r14d
	addl	%r14d, %r8d

	
	movl	52(%rsp), %r15d
	movl	44(%rsp), %r13d
	movl	%r13d, %r14d
	shrl	$10, %r13d
	roll	$13, %r14d
	xorl	%r14d, %r13d
	roll	$2, %r14d
	xorl	%r14d, %r13d
	addl	%r13d, %r15d
	movl	56(%rsp), %r13d
	movl	%r13d, %r14d
	shrl	$3, %r13d
	roll	$14, %r14d
	xorl	%r14d, %r13d
	roll	$11, %r14d
	xorl	%r14d, %r13d
	addl	%r13d, %r15d
	addl	24(%rsp), %r15d
	movl	%r15d, 52(%rsp)
 
	movl	%r12d, %r13d
	movl	%r12d, %r14d
	roll	$7, %r13d
	roll	$21, %r14d
	xorl	%r13d, %r14d
	roll	$19, %r13d
	xorl	%r13d, %r14d
	addl	%r15d, %ebp
	addl	%r14d, %ebp
	movl	%ebx, %r13d
	xorl	%eax, %r13d
	andl	%r12d, %r13d
	xorl	%ebx, %r13d
	addl	52(%rsi,%rdi,4), %ebp
	addl	%r13d, %ebp
	addl	%ebp, %r11d

	movl	%r8d, %r13d
	movl	%r8d, %r14d
	roll	$10, %r13d
	roll	$19, %r14d
	xorl	%r13d, %r14d
	roll	$20, %r13d
	xorl	%r13d, %r14d
	addl	%r14d, %ebp
	movl	%r8d, %r13d
	movl	%r8d, %r14d
	andl	%r9d, %r13d
	xorl	%r9d, %r14d
	addl	%r13d, %ebp
	andl	%r10d, %r14d
	addl	%r14d, %ebp

	
	movl	56(%rsp), %r15d
	movl	48(%rsp), %r13d
	movl	%r13d, %r14d
	shrl	$10, %r13d
	roll	$13, %r14d
	xorl	%r14d, %r13d
	roll	$2, %r14d
	xorl	%r14d, %r13d
	addl	%r13d, %r15d
	movl	60(%rsp), %r13d
	movl	%r13d, %r14d
	shrl	$3, %r13d
	roll	$14, %r14d
	xorl	%r14d, %r13d
	roll	$11, %r14d
	xorl	%r14d, %r13d
	addl	%r13d, %r15d
	addl	28(%rsp), %r15d
	movl	%r15d, 56(%rsp)
 
	movl	%r11d, %r13d
	movl	%r11d, %r14d
	roll	$7, %r13d
	roll	$21, %r14d
	xorl	%r13d, %r14d
	roll	$19, %r13d
	xorl	%r13d, %r14d
	addl	%r15d, %ebx
	addl	%r14d, %ebx
	movl	%eax, %r13d
	xorl	%r12d, %r13d
	andl	%r11d, %r13d
	xorl	%eax, %r13d
	addl	56(%rsi,%rdi,4), %ebx
	addl	%r13d, %ebx
	addl	%ebx, %r10d

	movl	%ebp, %r13d
	movl	%ebp, %r14d
	roll	$10, %r13d
	roll	$19, %r14d
	xorl	%r13d, %r14d
	roll	$20, %r13d
	xorl	%r13d, %r14d
	addl	%r14d, %ebx
	movl	%ebp, %r13d
	movl	%ebp, %r14d
	andl	%r8d, %r13d
	xorl	%r8d, %r14d
	addl	%r13d, %ebx
	andl	%r9d, %r14d
	addl	%r14d, %ebx

	
	movl	60(%rsp), %r15d
	movl	52(%rsp), %r13d
	movl	%r13d, %r14d
	shrl	$10, %r13d
	roll	$13, %r14d
	xorl	%r14d, %r13d
	roll	$2, %r14d
	xorl	%r14d, %r13d
	addl	%r13d, %r15d
	movl	(%rsp), %r13d
	movl	%r13d, %r14d
	shrl	$3, %r13d
	roll	$14, %r14d
	xorl	%r14d, %r13d
	roll	$11, %r14d
	xorl	%r14d, %r13d
	addl	%r13d, %r15d
	addl	32(%rsp), %r15d
	movl	%r15d, 60(%rsp)
 
	movl	%r10d, %r13d
	movl	%r10d, %r14d
	roll	$7, %r13d
	roll	$21, %r14d
	xorl	%r13d, %r14d
	roll	$19, %r13d
	xorl	%r13d, %r14d
	addl	%r15d, %eax
	addl	%r14d, %eax
	movl	%r12d, %r13d
	xorl	%r11d, %r13d
	andl	%r10d, %r13d
	xorl	%r12d, %r13d
	addl	60(%rsi,%rdi,4), %eax
	addl	%r13d, %eax
	addl	%eax, %r9d

	movl	%ebx, %r13d
	movl	%ebx, %r14d
	roll	$10, %r13d
	roll	$19, %r14d
	xorl	%r13d, %r14d
	roll	$20, %r13d
	xorl	%r13d, %r14d
	addl	%r14d, %eax
	movl	%ebx, %r13d
	movl	%ebx, %r14d
	andl	%ebp, %r13d
	xorl	%ebp, %r14d
	addl	%r13d, %eax
	andl	%r8d, %r14d
	addl	%r14d, %eax

	addl	$16, %edi
	cmpl	$64, %edi
	jne	.Loop2

	mov	64(%rsp), %rdi

	addl	(%rdi), %eax
	addl	4(%rdi), %ebx
	addl	8(%rdi), %ebp
	addl	12(%rdi), %r8d
	addl	16(%rdi), %r9d
	addl	20(%rdi), %r10d
	addl	24(%rdi), %r11d
	addl	28(%rdi), %r12d

	movl	%eax, (%rdi)
	movl	%ebx, 4(%rdi)
	movl	%ebp, 8(%rdi)
	movl	%r8d, 12(%rdi)
	movl	%r9d, 16(%rdi)
	movl	%r10d, 20(%rdi)
	movl	%r11d, 24(%rdi)
	movl	%r12d, 28(%rdi)

	add	$64, %rcx
	dec	%rdx
	jnz	.Loop_block

	mov	72(%rsp), %rbx
	mov	80(%rsp), %rbp
	mov	88(%rsp), %r12
	mov	96(%rsp), %r13
	mov	104(%rsp),%r14
	mov	112(%rsp),%r15

	add	$120, %rsp
.Lend:
	mov	%rcx, %rax
	
  

	ret
.size _nettle_sha256_compress_n_x86_64, . - _nettle_sha256_compress_n_x86_64



.section .note.GNU-stack,"",%progbits
