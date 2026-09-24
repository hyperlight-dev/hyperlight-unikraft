#include <stdio.h>
#include <sys/utsname.h>

int main(void)
{
	struct utsname u;
	uname(&u);
	printf("Hello from {{name}}!\n");
	printf("C on %s %s, inside a Hyperlight micro-VM\n", u.sysname, u.machine);
	return 0;
}
