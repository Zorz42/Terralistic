#include <chrono>
#include <thread>
#include <iostream>
#include "testing.hpp"
#include "networking.hpp"

TEST_CLASS(TestNetworking)

void waitABit(int ms=1) {
    std::this_thread::sleep_for(std::chrono::milliseconds(ms));
}

int port = 45923;

TcpSocket c_socket, s_socket;
TcpListener listener;

CASE_CONSTRUCTOR {
    port++;
    c_socket = TcpSocket();
    s_socket = TcpSocket();
    listener = TcpListener();
    
    listener.listen(port);
    
    c_socket.connect("127.0.0.1", port);
    waitABit();
    listener.accept(s_socket);
}

CASE_DESTRUCTOR {
    if(s_socket.isConnected())
        s_socket.disconnect();
    
    listener.close();
    if(c_socket.isConnected())
        c_socket.disconnect();
}

TEST_CASE(testNetworkingConnects) {
    TcpSocket client_socket, server_socket;
    TcpListener server_listener;

    int curr_port = 45920;
    server_listener.listen(curr_port);

    ASSERT(client_socket.connect("127.0.0.1", curr_port));
    waitABit();
    ASSERT(server_listener.accept(server_socket));

    server_socket.disconnect();

    server_listener.close();
    client_socket.disconnect();
}

TEST_CASE(testSocketConnectedState) {
    TcpSocket client_socket, server_socket;
    TcpListener server_listener;
    
    ASSERT(!client_socket.isConnected());
    ASSERT(!server_socket.isConnected());

    int curr_port = 45921;
    server_listener.listen(curr_port);

    client_socket.connect("127.0.0.1", curr_port);
    ASSERT(client_socket.isConnected());
    waitABit();
    server_listener.accept(server_socket);
    ASSERT(server_socket.isConnected());

    server_socket.disconnect();
    ASSERT(!server_socket.isConnected());

    server_listener.close();
    client_socket.disconnect();
    ASSERT(!client_socket.isConnected());
}

TEST_CASE(testListenerHandleError) {
    TcpSocket client_socket, server_socket;
    TcpListener server_listener;

    int curr_port = 45922;
    server_listener.listen(curr_port);
    
    ASSERT(!server_listener.accept(s_socket));
    
    client_socket.connect("127.0.0.1", curr_port);
    client_socket.disconnect();

    server_listener.close();
}

TEST_CASE(testSocketSendsData) {
    Packet sent_packet, received_packet;
    sent_packet << 1552;
    
    c_socket.send(sent_packet);
    c_socket.flushPacketBuffer();
    
    waitABit();
    
    ASSERT(s_socket.receive(received_packet));
    int received;
    received_packet >> received;
    
    ASSERT(received == 1552);
}

TEST_CASE(testSocketSendsLargeData) {
    Packet sent_packet, received_packet;
    std::vector<char> data, received_data;
    for(int i = 0; i < 1000000; i++)
        data.push_back(rand());
    sent_packet << data;
    
    c_socket.send(sent_packet);
    c_socket.flushPacketBuffer();
    
    waitABit();
    
    ASSERT(s_socket.receive(received_packet));
    received_packet >> received_data;
    
    ASSERT(received_data == data);
}

TEST_CASE(testSocketReceivesNothing) {
    Packet received_packet;
    ASSERT(!s_socket.receive(received_packet));
    ASSERT(!c_socket.receive(received_packet));
}

TEST_CASE(testSocketGetIpAddress) {
    TcpSocket socket;
    ASSERT_THROWS(NotConnectedError, socket.getIpAddress());
        
    ASSERT(c_socket.getIpAddress() == "127.0.0.1");
    ASSERT(s_socket.getIpAddress() == "127.0.0.1");
}

TEST_CASE(testSocketThrowsBasedOnConnectedState) {
    TcpSocket socket;
    
    Packet packet;
    ASSERT_THROWS(NotConnectedError, socket.disconnect());
    ASSERT_THROWS(NotConnectedError, socket.getIpAddress());
    ASSERT_THROWS(NotConnectedError, socket.flushPacketBuffer());
    ASSERT_THROWS(NotConnectedError, socket.receive(packet));
    ASSERT_THROWS(NotConnectedError, socket.send(packet));
    
    c_socket.getIpAddress();
    c_socket.flushPacketBuffer();
    ASSERT_THROWS(AlreadyConnectedError, c_socket.connect("", 0));
    
    s_socket.getIpAddress();
    s_socket.flushPacketBuffer();
    ASSERT_THROWS(AlreadyConnectedError, s_socket.connect("", 0));
    
    s_socket.disconnect();
    
    ASSERT_THROWS(NotConnectedError, s_socket.disconnect());
    ASSERT_THROWS(NotConnectedError, s_socket.getIpAddress());
    ASSERT_THROWS(NotConnectedError, s_socket.flushPacketBuffer());
    ASSERT_THROWS(NotConnectedError, s_socket.receive(packet));
    ASSERT_THROWS(NotConnectedError, s_socket.send(packet));
    
    c_socket.disconnect();
    
    c_socket.connect("127.0.0.1", port);
    ASSERT_THROWS(AlreadyConnectedError, listener.accept(c_socket));
}

TEST_CASE(testClientDisconnects) {
    c_socket.disconnect();
    waitABit();
    
    Packet packet;
    s_socket.receive(packet);
    
    ASSERT(!s_socket.isConnected());
}

TEST_CASE(testServerDisconnects) {
    s_socket.disconnect();
    waitABit();
    
    Packet packet;
    c_socket.receive(packet);
    
    ASSERT(!c_socket.isConnected());
}

TEST_CASE(testWronglyFormattedAddress) {
    TcpSocket socket;
     
    ASSERT_THROWS(AddressFormatError, socket.connect("3543.54;AA", 120));
}

TEST_CASE(testConnectDenied) {
    TcpSocket socket;

    socket.connect("127.0.0.1", 2342);

    ASSERT(!socket.isConnected());
}

TEST_CASE(testSocketError) {
    TcpSocket socket;
    
    ASSERT_THROWS(SocketError, socket.connect("127.0.0.1", 0));
}

TEST_CASE(testSocketAutoFlushes) {
    Packet sent_packet, received_packet;
    for(int i = 0; i < 50000; i++)
        sent_packet << 42;
    
    c_socket.send(sent_packet);
    
    waitABit(10);
    
    ASSERT(s_socket.receive(received_packet));
}


END_TEST_CLASS(TestNetworking)
