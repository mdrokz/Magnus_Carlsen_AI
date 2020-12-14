#include <sys/socket.h>
#include <sys/un.h>

#include <errno.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

#define SV_SOCK_PATH "./socket"

int main(int argc, char *argv[]) {
  struct sockaddr_un addr;

  // Create a new server socket with domain: AF_UNIX, type: SOCK_STREAM, protocol: 0
  int sfd = socket(AF_UNIX, SOCK_STREAM, 0);
  printf("Server socket fd = %d\n", sfd);

  // Make sure socket's file descriptor is legit.
  if (sfd == -1) {
    // errExit("socket");
    printf("socket");
  }

  // Make sure the address we're planning to use isn't too long.
  if (strlen(SV_SOCK_PATH) > sizeof(addr.sun_path) - 1) {
    fatal("Server socket path too long: %s", SV_SOCK_PATH);
  }

  // Delete any file that already exists at the address. Make sure the deletion
  // succeeds. If the error is just that the file/directory doesn't exist, it's fine.
  if (remove(SV_SOCK_PATH) == -1 && errno != ENOENT) {
    // errExit("remove-%s", SV_SOCK_PATH);
    printf("remove-%s", SV_SOCK_PATH);
  }

  // Zero out the address, and set family and path.
  memset(&addr, 0, sizeof(struct sockaddr_un));
  addr.sun_family = AF_UNIX;
  strncpy(addr.sun_path, SV_SOCK_PATH, sizeof(addr.sun_path) - 1);

  // Bind the socket to the address. Note that we're binding the server socket
  // to a well-known address so that clients know where to connect.
  printf("V %ld",sizeof(struct sockaddr_un));
  if (bind(sfd, (struct sockaddr *) &addr, sizeof(struct sockaddr_un)) == -1) {
    // errExit("bind");
    printf("bind");
  }

}
