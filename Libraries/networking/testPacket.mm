#include "testing.hpp"
#include "networking.hpp"

TEST_CLASS(TestPacket)

TEST_CASE(testPacketSavesBool) {
    bool value = true, result = false;
    Packet packet;
    packet << value;
    packet >> result;
    ASSERT(value == result);
    
    value = false;
    result = true;
    Packet packet2;
    packet2 << value;
    packet2 >> result;
    ASSERT(value == result);
}

TEST_CASE(testPacketSavesChar) {
    char value = 'a', result = 'b';
    Packet packet;
    packet << value;
    packet >> result;
    ASSERT(value == result);
}

TEST_CASE(testPacketSavesUnsignedChar) {
    unsigned char value = 'D', result = '-';
    Packet packet;
    packet << value;
    packet >> result;
    ASSERT(value == result);
}

TEST_CASE(testPacketSavesShort) {
    short value = 42, result = -21;
    Packet packet;
    packet << value;
    packet >> result;
    ASSERT(value == result);
}

TEST_CASE(testPacketSavesUnsignedShort) {
    unsigned short value = 240, result = 12;
    Packet packet;
    packet << value;
    packet >> result;
    ASSERT(value == result);
}

TEST_CASE(testPacketSavesInt) {
    int value = -175872, result = -122682;
    Packet packet;
    packet << value;
    packet >> result;
    ASSERT(value == result);
}

TEST_CASE(testPacketSavesUnsignedInt) {
    unsigned int value = 157389, result = 165782;
    Packet packet;
    packet << value;
    packet >> result;
    ASSERT(value == result);
}

TEST_CASE(testPacketSavesLongLong) {
    long long value = -157979256197LL, result = 17956907624LL;
    Packet packet;
    packet << value;
    packet >> result;
    ASSERT(value == result);
}

TEST_CASE(testPacketSavesUnsignedLongLong) {
    unsigned long long value = 165797827975972LL, result = 12316846926597LL;
    Packet packet;
    packet << value;
    packet >> result;
    ASSERT(value == result);
}

TEST_CASE(testPacketSavesFloat) {
    float value = 124.1423, result = 12.233527;
    Packet packet;
    packet << value;
    packet >> result;
    ASSERT(value == result);
}

TEST_CASE(testPacketSavesDouble) {
    double value = 15234.23424, result = 14.23422;
    Packet packet;
    packet << value;
    packet >> result;
    ASSERT(value == result);
}

TEST_CASE(testPacketSavesString) {
    std::string value = "ghjfdjhsdfjh$%^&*(", result = "hgjkfdgjhksdx24$";
    Packet packet;
    packet << value;
    packet >> result;
    ASSERT(value == result);
}

TEST_CASE(testPacketSavesCharVector) {
    std::vector<char> value = {'a', 'a', 'b', '%', 'g', 'H', 'E', '\0', 'H', '%', '1'}, result = {'f', 'a', 'G', '4', 'g', 'h', '(', 'G', 'G'};
    Packet packet;
    packet << value;
    packet >> result;
    ASSERT(value == result);
}

TEST_CASE(testPacketSavesUnsignedCharVector) {
    std::vector<unsigned char> value = {'a', 'a', 'b', '%', 'g', 'H', 'E', '\0', 'H', '%', '1'}, result = {'f', 'a', 'G', '4', 'g', 'h', '(', 'G', 'G'};
    Packet packet;
    packet << value;
    packet >> result;
    ASSERT(value == result);
}

TEST_CASE(testPacketSavesMixed) {
    Packet packet;
    packet << (std::string)"this is a test." << 100 << 1562LL << 'a';
    
    std::string a;
    int b;
    long long c;
    char d;
    packet >> a >> b >> c >> d;
    
    ASSERT(a == "this is a test.");
    ASSERT(b == 100);
    ASSERT(c == 1562LL);
    ASSERT(d == 'a');
}

END_TEST_CLASS(TestPacket)
