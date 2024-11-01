
























	.file "cpuid.asm"

	

	.text
	.align 16

.globl _nettle_cpuid
.type _nettle_cpuid,%function
_nettle_cpuid: 
	
  

	push	%rbx

	movl	%edi, %eax
	xorl	%ecx, %ecx      
	cpuid
	mov	%eax, (%rsi)
	mov	%ebx, 4(%rsi)
	mov	%ecx, 8(%rsi)
	mov	%edx, 12(%rsi)

	pop	%rbx
	
  

	ret
.size _nettle_cpuid, . - _nettle_cpuid



.section .note.GNU-stack,"",%progbits
