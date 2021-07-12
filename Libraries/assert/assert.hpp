#ifndef assert_hpp
#define assert_hpp

// Enabling developer mode will show any errors that program made that are not handled, because they shouldn't occur in release version.

#ifdef DEVELOPER_MODE

#define ASSERT(expression, message) if(!(expression)) { throw message; }

#else

#define ASSERT(x, message) x;

#endif

#endif /* assert_hpp */
