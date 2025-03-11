import socket
import sys


def make_tcp_request(host: str, port: int) -> None:
    sock: socket.socket
    ip_version: int # ipv4 vs v6
    tcp_or_udp: socket.SocketKind # tcp vs udp

    # getaddrinfo: tcp & ipv4+v6
    for ip_version, tcp_or_udp, _, _, _  in socket.getaddrinfo(host, port, socket.AF_UNSPEC, socket.SOCK_STREAM):
        try:
            # try to create a socket and connect for the given addr info
            sock = socket.socket(ip_version, tcp_or_udp)
            sock.connect((host, port))

        except OSError as error:
            print(error)
            if sock:
                sock.close()
                continue

        break

    if not sock:
        print(f"no connection available for (host, port): {(host, port)}")
        sys.exit(1)

    # send & receive data
    with sock:
        sock.sendall(b"Hi from client!")
        
        if received := sock.recv(1024):
            print(received.decode())


if __name__ == "__main__":
    make_tcp_request("localhost", 8080)
