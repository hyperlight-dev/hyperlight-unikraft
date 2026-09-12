#include <stdio.h>
#include <uk/print.h>

int main(void)
{
	uk_pr_warn("native-kernel-boot-marker\n");
	printf("Hello, World from a NATIVE mini kernel!\n");
	return 0;
}
