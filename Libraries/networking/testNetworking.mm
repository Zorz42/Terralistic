#include "testing.hpp"
#include "networking.hpp"

TEST_CLASS(TestNetworking)

TEST_CASE(testNetworkingConnects) {
    TcpSocket c_socket, s_socket;
    TcpListener listener;

    int port = 45923;
    listener.listen(port);

    ASSERT(c_socket.connect("127.0.0.1", port));
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

    int port = 45926;
    listener.listen(port);

    c_socket.connect("127.0.0.1", port);
    
    listener.accept(s_socket);

    Packet sent_packet, received_packet;
    sent_packet << 1552;
    
    c_socket.send(sent_packet);
    c_socket.flushPacketBuffer();
    
    ASSERT(s_socket.receive(received_packet));
    int received;
    received_packet >> received;
    
    ASSERT(received == 1552);
    
    s_socket.disconnect();

    listener.close();
    c_socket.disconnect();
}

END_TEST_CLASS(TestNetworking)
