#ifndef INCLUDED_DEKA
#define INCLUDED_DEKA

#include <regex>
#include <filesystem>
#include <future>

namespace deka {

template <typename T> class DekaBase {
    public: T value(T value) { return value; }
};
template <typename T> class DekaSub {
    public: T sub_value(T value) { return value; }
};

template <typename T> class DekaA : public DekaBase <T> { };
template <typename T> class DekaB : public DekaA <T> { };
template <typename T> class DekaC : public DekaB <T> { };

template <typename T> class DekaSubA : public DekaSub <T> { };
template <typename T> class DekaSubB : public DekaSubA <T> { };
template <typename T> class DekaSubC : public DekaSubB <T> { };

}

#endif
