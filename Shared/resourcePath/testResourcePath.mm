#include "testing.hpp"
#include "resourcePath.hpp"

TEST_CLASS(TestResourcePath)

TEST_CASE(testResourcePath) {
    ASSERT(getResourcePath("/test/folder/exec") == "/test/folder/Resources/");
    ASSERT(getResourcePath("/test/folder/MacOS/exec") == "/test/folder/Resources/");
    
    ASSERT(getResourcePath("/testing/1234/exec") == "/testing/1234/Resources/");
    ASSERT(getResourcePath("/testing/1234/MacOS/exec") == "/testing/1234/Resources/");
    
    ASSERT(getResourcePath("/folder/folder/folder/folder/folder/folder/notfolder") == "/folder/folder/folder/folder/folder/folder/Resources/");
    ASSERT(getResourcePath("/folder/folder/folder/folder/folder/folder/MacOS/notfolder") == "/folder/folder/folder/folder/folder/folder/Resources/");
    
    ASSERT(getResourcePath("./exec") == "./Resources/");
    
    ASSERT(getResourcePath("exec") == "./Resources/");
}

END_TEST_CLASS(TestResourcePath)
