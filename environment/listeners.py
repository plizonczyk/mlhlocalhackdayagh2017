import socketserver
import threading


class TcpHandler(socketserver.BaseRequestHandler):
    def handle(self):
        print('Connection received ({})'.format(self.client_address))
        try:
            data = self.request.recv(8192).decode()
            print('{} Recived data\n{}\n\n'.format(type(self.server), data))
        except Exception as e:
            print('Exception caught ({})'.format(repr(e)))


class TCPServer(socketserver.ThreadingMixIn, socketserver.TCPServer):
    pass


class UdpHandler(socketserver.BaseRequestHandler):
    def handle(self):
        print('Connection received ({})'.format(self.client_address))
        try:
            data = self.request[0]
            print('{} Recived data\n{}\n\n'.format(type(self.server), data))
        except Exception as e:
            print('Exception caught ({})'.format(repr(e)))


class UDPServer(socketserver.ThreadingMixIn, socketserver.UDPServer):
    pass

udp_addr = ('', 50123)
tcp_addr = ('', 50321)

class UDPThread(threading.Thread):
    def run(self):
        print('UDP started')
        with UDPServer(udp_addr, UdpHandler) as udp:
            udp.serve_forever()

class TCPThread(threading.Thread):
    def run(self):
        print('TCP started')
        with TCPServer(tcp_addr, TcpHandler) as tcp:
            tcp.serve_forever()

udp = UDPThread()
udp.start()

tcp = TCPThread()
tcp.start()

udp.join()
tcp.join()
