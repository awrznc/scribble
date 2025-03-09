#include <stdio.h>
#include <string>
#include "piyo.h"

class Piyo
{
public:
    static int return_emoji(char16_t *result, unsigned int size);
};

int Piyo::return_emoji(char16_t *result, unsigned int size)
{
    static char16_t *value = u"🌟a🌟a🌟";
    auto len = std::char_traits<char16_t>::length(value);
    // printf("%zu", len);
    if (size < len)
    {
        return len;
    }

    std::char_traits<char16_t>::copy(result, value, len);
    return len;
}

int return_star(unsigned short *result, unsigned int size)
{
    static_assert(sizeof(unsigned short) == sizeof(char16_t), "size error.");
    char16_t *message = reinterpret_cast<char16_t *>(result);
    return Piyo::return_emoji(message, size);
}
