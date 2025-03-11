import socket


def make_tcp_request(host: str, port: int) -> None:
    ip_version: int # ipv4 vs v6
    conn_type: socket.SocketKind # tcp vs udp

    # getaddrinfo: tcp & ipv4+v6
    for family, type, _, _, addr in socket.getaddrinfo(host, port, socket.AF_UNSPEC, socket.SOCK_STREAM):
        # create a socket
        if addr is not None:
            ip_version, conn_type = family, type
            break

    # connect
    with socket.socket(ip_version, conn_type) as sock:
        sock.connect((host, port))
        sock.sendall(b"Hi from client!")
        
        if received := sock.recv(1024):
            print(received.decode())


if __name__ == "__main__":
    make_tcp_request("localhost", 8080)
