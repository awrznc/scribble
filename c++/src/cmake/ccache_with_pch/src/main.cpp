#include "app.hpp"
#include "color.hpp"
#include "deka.hpp"

int main() {
    int status = app::return_zero();
    std::unique_ptr<app::Color> c(new app::Color());
    auto deka = deka::DekaC<unsigned char>().value(255);

    return app::get_init_red_value() == c->GetRed() && deka == c->GetRed();
}
