#include <iostream>
#include <vector>
#include "app.hpp"
#include "color.hpp"
#include "deka.hpp"

namespace app {
    int return_zero() {
        return 0;
    }
    unsigned char get_init_red_value() {
        std::unique_ptr<app::Color> c(new app::Color());
        return c->GetRed();
    }
    unsigned char return_255() {
        return deka::DekaC<unsigned char>().value(255);
    }
}
