import nativesockets
import htmlparser
import xmltree
import strtabs
import strutils

var errno* {.importc: "errno", header: "<errno.h>".}: int

var unix_socket = createNativeSocket(ord(AF_UNIX), ord(SOCK_STREAM), 0)

var s: Sockaddr

var file = open("./download.txt",fmAppend)

# ['.','/','s','o','c','k','e','t']

s.sa_family = ord(AF_UNIX)
s.sa_data = cast[array[0..13, char]](['.', '/', 's', 'o', 'c', 'k', 'e', 't',
        '\x00', '\x00', '\x00', '\x00', '\x00'])


var html_string = ""

var f: SockLen = cast[uint32](sizeof(Sockaddr_un))

var buffer: array[0..27910, char]

echo unix_socket.bindAddr(s.addr, f)

echo errno

if errno != 0:
    quit(errno)

echo unix_socket.listen()

echo errno

if errno != 0:
    quit(errno)

while true:

    var (sock_handle, address) = unix_socket.accept()

    echo address
    while true:
        echo sock_handle.recv(buffer.addr, 27910, 0)

        for v in buffer.items:
            add(html_string, v)
        var html = parseHtml(html_string)

        for a in html.findAll("a"):
            if a.attrs.hasKey "href":
               var href = a.attrs["href"]

               if "/perl/chessgame" in href:
                   add(href,"\n")
                   file.write(href)
