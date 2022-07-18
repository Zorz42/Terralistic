#include <chrono>
#include <thread>
#include "testing.hpp"
#include "networking.hpp"

TEST_CLASS(TestNetworking)

void waitABit() {
    std::this_thread::sleep_for(std::chrono::milliseconds(1));
}

TEST_CASE(testNetworkingConnects) {
    TcpSocket c_socket, s_socket;
    TcpListener listener;

    int port = 45923;
    listener.listen(port);

    ASSERT(c_socket.connect("127.0.0.1", port));
    waitABit();
    ASSERT(listener.accept(s_socket));

    s_socket.disconnect();

    listener.close();
    c_socket.disconnect();
}

TEST_CASE(testSocketConnectedState) {
    TcpSocket c_socket, s_socket;
    TcpListener listener;

    ASSERT(!c_socket.isConnected());
    ASSERT(!s_socket.isConnected());

    int port = 45924;
    listener.listen(port);

    c_socket.connect("127.0.0.1", port);
    ASSERT(c_socket.isConnected());
    waitABit();
    listener.accept(s_socket);
    ASSERT(s_socket.isConnected());

    s_socket.disconnect();
    ASSERT(!s_socket.isConnected());

    listener.close();
    c_socket.disconnect();
    ASSERT(!c_socket.isConnected());
}

TEST_CASE(testListenerHandleError) {
    TcpSocket c_socket, s_socket;
    TcpListener listener;

    int port = 45925;
    listener.listen(port);
    
    ASSERT(!listener.accept(s_socket));
    
    c_socket.connect("127.0.0.1", port);
    c_socket.disconnect();

    listener.close();
}

TEST_CASE(testSocketSendsData) {
    TcpSocket c_socket, s_socket;
    TcpListener listener;

    int port = 4592;
    listener.listen(port);

    c_socket.connect("127.0.0.1", port);
    waitABit();
    listener.accept(s_socket);

    Packet sent_packet, received_packet;
    sent_packet << 1552;
    
    c_socket.send(sent_packet);
    c_socket.flushPacketBuffer();
    
    waitABit();
    
    ASSERT(s_socket.receive(received_packet));
    int received;
    received_packet >> received;
    
    ASSERT(received == 1552);
    
    s_socket.disconnect();

    listener.close();
    c_socket.disconnect();
}

TEST_CASE(testSocketGetIpAddress) {
    TcpSocket c_socket, s_socket;
    TcpListener listener;

    int port = 45927;
    listener.listen(port);

    c_socket.connect("127.0.0.1", port);
    waitABit();
    listener.accept(s_socket);

    ASSERT(c_socket.getIpAddress() == "127.0.0.1");
    ASSERT(s_socket.getIpAddress() == "127.0.0.1");
    
    s_socket.disconnect();
    
    listener.close();
    c_socket.disconnect();
    
    ASSERT_THROWS(NotConnectedError, c_socket.getIpAddress());
    ASSERT_THROWS(NotConnectedError, s_socket.getIpAddress());
}

TEST_CASE(testSocketThrowsBasedOnConnectedState) {
    TcpSocket c_socket, s_socket;
    TcpListener listener;
    
    Packet packet;
    ASSERT_THROWS(NotConnectedError, c_socket.disconnect());
    ASSERT_THROWS(NotConnectedError, c_socket.getIpAddress());
    ASSERT_THROWS(NotConnectedError, c_socket.flushPacketBuffer());
    ASSERT_THROWS(NotConnectedError, c_socket.receive(packet));
    ASSERT_THROWS(NotConnectedError, c_socket.send(packet));
    
    int port = 45928;
    listener.listen(port);

    c_socket.connect("127.0.0.1", port);
    waitABit();
    listener.accept(s_socket);
    
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
    
    c_socket.disconnect();
    
    listener.close();
    
    ASSERT_THROWS(NotConnectedError, c_socket.disconnect());
    ASSERT_THROWS(NotConnectedError, c_socket.getIpAddress());
    ASSERT_THROWS(NotConnectedError, c_socket.flushPacketBuffer());
    ASSERT_THROWS(NotConnectedError, c_socket.receive(packet));
    ASSERT_THROWS(NotConnectedError, c_socket.send(packet));
}

TEST_CASE(testClientDisconnects) {
    TcpSocket c_socket, s_socket;
    TcpListener listener;

    int port = 45929;
    listener.listen(port);

    c_socket.connect("127.0.0.1", port);
    waitABit();
    listener.accept(s_socket);

    c_socket.disconnect();
    waitABit();
    
    Packet packet;
    s_socket.receive(packet);
    
    ASSERT(!s_socket.isConnected());

    listener.close();
}

TEST_CASE(testServerDisconnects) {
    TcpSocket c_socket, s_socket;
    TcpListener listener;

    int port = 45930;
    listener.listen(port);

    c_socket.connect("127.0.0.1", port);
    waitABit();
    listener.accept(s_socket);

    s_socket.disconnect();
    waitABit();
    
    Packet packet;
    c_socket.receive(packet);
    
    ASSERT(!c_socket.isConnected());

    listener.close();
}

END_TEST_CLASS(TestNetworking)
