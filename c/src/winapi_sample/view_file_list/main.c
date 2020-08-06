#include <windows.h>
#include <stdio.h>
int main()
{
	HANDLE hFind;
	WIN32_FIND_DATA win32fd;

	hFind = FindFirstFile("C:\\*.*", &win32fd);

	if (hFind == INVALID_HANDLE_VALUE) {
		return 1;
	}

	do {
		if (win32fd.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY) {
			printf("%s (DIR)\n", win32fd.cFileName);
		} else {
			printf("%s\n", win32fd.cFileName);
		}
	} while (FindNextFile(hFind, &win32fd));

	FindClose(hFind);

	return 0;
}
