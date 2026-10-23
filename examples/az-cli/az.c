/*
 * az — launches the Azure CLI: python3 -m azure.cli "$@".
 *
 * An ELF rather than pip's shell script because the guest's execve loads
 * ELF images only (no #! scripts).
 */
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

int main(int argc, char **argv)
{
	char **args = calloc((size_t)argc + 3, sizeof(*args));

	if (!args)
		return 1;
	args[0] = "/usr/local/bin/python3";
	args[1] = "-m";
	args[2] = "azure.cli";
	for (int i = 1; i < argc; i++)
		args[i + 2] = argv[i];
	execv(args[0], args);
	perror("az: execv /usr/local/bin/python3");
	return 127;
}
