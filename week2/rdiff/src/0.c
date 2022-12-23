#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <stdio.h>

char buf[20] = {0};

int main() {
  int fd1 = open("cookie.txt", O_RDONLY);
  int fd2 = open("cookie.txt", O_RDONLY);
  dup2(fd2, fd1);
  read(fd1, buf, 4);
  close(fd1);
  read(fd2, &buf[3], 6);
  close(fd2);
  printf("buf = %s\n", buf);
  return 0;
}
