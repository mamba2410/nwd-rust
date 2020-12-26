
// PROJECT_NAME.S

#include <PROJECT_NAME.h>

	.text
	.global main
main:
	pushq	%rbp
	movq	%rsp, %rbp
	subq	$48, %rsp

	movl	$0, %eax
	leave
	ret


