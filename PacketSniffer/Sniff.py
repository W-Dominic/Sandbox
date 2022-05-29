import socket
from ipaddress import ip_address
import sys
import os

#TODO:
'''
create payload data for tcp protocols and others
begin deconstructing payloads into human readable info
    *allow user to select what type of payload data should be displayed:
    *ex bytes, binary, hex, string (human readable)
add functionality for http packets
'''

class Packet:
    def __init__(self,DATA):
        self.length = len(DATA)
        #layer 2
        self.dst_mac = str(DATA[:6].hex())
        self.src_mac = str(DATA[6:12].hex())
        self.data = DATA[14:]
        #layer 3
        self.src_ip = ip_address(self.data[12:16])
        self.dst_ip = ip_address(self.data[16:20])
        #layer 4
        self.protocol = self.data[9]
        
        #unpack tcp or udp data
        if self.protocol == 6:
            self.tcp()
        elif self.protocol == 17:
            self.udp()
        elif self.protocol == 1:
            self.icmp()
        else:
            self.src_port = -1
            self.dst_port = -1
            self.payload = "N/A"

    def icmp(self):
        self.src_port = -1
        self.dst_port = -1
        self.payload = self.data[36:].decode("unicode-escape")
    def tcp(self):
        self.src_port = int(self.data[20:22].hex(), 16)
        self.dst_port = int(self.data[22:24].hex(), 16)
        
        self.payload = "N/A"
    def udp(self):  
        self.src_port = int(self.data[20:22].hex(), 16)
        self.dst_port = int(self.data[22:24].hex(), 16)

        self.payload = self.data[28:]

    #string representation of the packet
    def __str__(self):
        sm = self.src_mac
        dm = self.dst_mac
        view = f'''
                -------------------------------------------------
                packet length: {self.length}
                destination mac: {dm[:2]}:{dm[2:4]}:{dm[4:6]}:{dm[6:8]}:{dm[8:10]}:{dm[10:12]}
                source mac: {sm[:2]}:{sm[2:4]}:{sm[4:6]}:{sm[6:8]}:{sm[8:10]}:{sm[10:12]}
                destination IP: {self.dst_ip}
                source IP: {self.src_ip}
                protocol: {self.protocol}
                source port: {self.src_port}
                destination port: {self.dst_port}
                payload: {self.payload}
                -------------------------------------------------
                '''
        return view
    

def sniff(interface):
    sock = socket.socket(socket.AF_PACKET, socket.SOCK_RAW)
    sock.bind((interface, 3))
    directions = '''
    Accepted Filter Types: 
    _____Layer 2_____
        src_mac
        dst_mac
        mac
    _____Layer 3_____
        src_ip
        dst_ip
        ip
    _____Layer 4_____
        icmp
        tcp
        udp
        src_port
        dst_port
        port
    '''
    print(directions)
    filterType = input("Filter Type: ")
    filterVal = input("Value: ")
    #listen for a conncetion
    while True: 
        try: 
            data = sock.recv(1024)
        except OSError:
            pass
        else: 
            newPacket = Packet(data)
            filter(filterType,filterVal,newPacket)
#only print packets that match the filter type
#ex. protocol, src_port, dst_port, ip, mac, etc.
def filter(filterType,filterVal, packet):
    #protocol filters 
    if (filterType == "icmp") and (packet.protocol == 1):
        print(packet)
    elif (filterType == "tcp") and (packet.protocol == 6):
        print(packet)
    elif (filterType == "udp") and (packet.protocol == 17):
        print(packet)
    elif filterType == "": 
        print(packet)
    #MAC address filters
    elif filterType == "src_mac":
        MAC_ADDRESS = filterVal.replace("-","").replace(":","").replace(" ","")
        if MAC_ADDRESS == str(packet.src_mac):
            print(packet)
    elif filterType == "dst_mac":
        MAC_ADDRESS = filterVal.replace("-","").replace(":","").replace(" ","") 
        if MAC_ADDRESS == str(packet.dst_mac):
            print(packet)
    elif filterType == "mac":
        MAC_ADDRESS = filterVal.replace("-","").replace(":","").replace(" ","")
        if MAC_ADDRESS == str(packet.src_mac) or MAC_ADDRESS == str(packet.src_mac):
            print(packet)
    #IP filters
    elif filterType == "src_ip":
        IP = ip_address(filterVal)
        if (IP == packet.src_ip):
            print(packet)
    elif filterType == "dst_ip":
        IP = ip_address(filterVal)
        if (IP == packet.dst_ip):
            print(packet)
    elif filterType == "ip":
        IP = ip_address(filterVal)
        if (IP == packet.src_ip) or (IP == packet.dst_ip):
            print(packet)
    #port filters  
    elif filterType == "src_port":
        PORT = int(filterVal)
        if (PORT == packet.src_port):
            print(packet)
    elif filterType == "dst_port":
        PORT = int(filterVal)
        if (PORT == packet.dst_port):
            print(packet)
    elif filterType == "port":
        PORT = int(filterVal)
        if (PORT == packet.src_port) or (PORT == packet.dst_port):
            print(packet)
    else : 
        return
    print("\n")

if __name__ == "__main__":
    print(  '''
------------Welcome to Packet Sniffer----------------
            Directions:
            1) A list of your network interfaces will be shown
            2) Please select a network interface to sniff on (Default: 'wlp3s0') 
            3) A list of filters will be presented, and you will be prompted to select a filter, if any
                * for filters which require a value: ex src_port : 22 you will be asked to enter a filter value,
                  otherwise do not enter anything for the filter value prompt 
            4) Sniffing will begin and packets will be presented in real time
-----------------------------------------------------            
            ''')
    os.system("ip link")
    interface = input("Network Interface: ")
    if interface == "":
        sniff('wlp3s0')
    else: 
        try: 
            sniff(interface)
        except:
            print("Interface not recognized")
