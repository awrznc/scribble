#ifndef INCLUDED_COLOR
#define INCLUDED_COLOR

#include <memory>

namespace app {
    class Color {
        private:
            unsigned char r, g, b;
        public:
            Color() {
                this->r = 255;
                this->g = 255;
                this->b = 255;
            };
            ~Color() {};
            char GetRed() {
                return this->r;
            }
    };
}

#endif
