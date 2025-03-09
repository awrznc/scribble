
#include "piyo.h"

class Piyo
{
public:
    static int return_num();
};

int Piyo::return_num()
{
    return 42;
}

int return_42()
{
    return Piyo::return_num();
}
