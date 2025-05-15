define(`srcdir', ``.'')dnl
define(`SYMBOL_PREFIX', `'`$1')dnl
define(`ELF_STYLE', `yes')dnl
define(`COFF_STYLE', `no')dnl
define(`TYPE_FUNCTION', `%function')dnl
define(`TYPE_PROGBITS', `%progbits')dnl
define(`ALIGN_LOG', `no')dnl
define(`ELFV2_ABI', `no')dnl
define(`W64_ABI', `no')dnl
define(`RODATA', `.section .rodata')dnl
define(`WORDS_BIGENDIAN', `no')dnl
define(`ASM_X86_ENDBR',`')dnl
define(`ASM_X86_MARK_CET_ALIGN',`')dnl
define(`ASM_PPC_WANT_R_REGISTERS',`n/a')dnl
divert(1)

.section .note.GNU-stack,"",TYPE_PROGBITS
divert
