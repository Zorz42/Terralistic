#include "testing.hpp"
#include "networking.hpp"

TEST_CLASS(TestNetworking)

TEST_CASE(testNetworkingConnects) {
    TcpSocket c_socket, s_socket;
    TcpListener listener;

    ASSERT(!c_socket.isConnected());
    ASSERT(!s_socket.isConnected());

    int port = 45923;
    listener.listen(port);

    ASSERT(c_socket.connect("127.0.0.1", port));
    ASSERT(c_socket.isConnected());
    ASSERT(listener.accept(s_socket));
    ASSERT(s_socket.isConnected());

    s_socket.disconnect();
    ASSERT(!s_socket.isConnected());

    listener.close();
    c_socket.disconnect();
    ASSERT(!c_socket.isConnected());
}

END_TEST_CLASS(TestNetworking)
