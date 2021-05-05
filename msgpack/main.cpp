#include <msgpack.hpp>
#include <vector>
#include <string>
#include <iostream>

typedef std::vector<std::pair<uint64_t, uint64_t>> PairVector;

int pair_sample() {
    // pair
    std::pair<uint64_t, uint64_t> original_pair_pbject = std::make_pair(123, 456);
    std::cout << "pair_pbject:  " << std::endl;
    std::cout << "  first:  " << original_pair_pbject.first << std::endl;
    std::cout << "  second: " << original_pair_pbject.second << std::endl;

    // serialize
    msgpack::sbuffer sbuf;
    msgpack::pack(sbuf, original_pair_pbject);

    // deserialize
    msgpack::object_handle pair_pbject_handle = msgpack::unpack(sbuf.data(), sbuf.size());
    std::pair<uint64_t, uint64_t> deserialized_pair_object;
    pair_pbject_handle.get().convert(deserialized_pair_object);
    
    std::cout << "pair_pbject:  " << std::endl;
    std::cout << "  first:  " << deserialized_pair_object.first << std::endl;
    std::cout << "  second: " << deserialized_pair_object.second << std::endl;

    return 0;
}

int pair_vector_sample() {
    // vector
    PairVector vector_object;
    vector_object.push_back(std::make_pair(12, 34));
    vector_object.push_back(std::make_pair(56, 78));

    // serialize
    msgpack::sbuffer sbuf;
    msgpack::pack(sbuf, vector_object);

    // deserialize
    msgpack::object_handle vector_object_handle = msgpack::unpack(sbuf.data(), sbuf.size());
    msgpack::object object = vector_object_handle.get();
    PairVector deserialized_vector_object;
    object.convert(deserialized_vector_object);

    // print msgpack object
    std::cout << object << std::endl;

    return 0;
}

#include <fstream>

int load_pair_vector_sample() {
    // serialize
    {
        PairVector vector_object;
        vector_object.push_back(std::make_pair(12, 34));
        vector_object.push_back(std::make_pair(56, 78));

        std::ofstream file("data.mpac");
        msgpack::pack(&file, vector_object);
    }

    // deserialize
    {
        // std::ifstream file("data.mpac");
        std::ifstream file("pair_vector.mpack");
        std::istreambuf_iterator<char> first(file);
        std::istreambuf_iterator<char> last;
        const std::string data(first, last);

        msgpack::object_handle vector_object_handle = msgpack::unpack(data.data(), data.size());
        msgpack::object object = vector_object_handle.get();
        std::cout << object << std::endl;
        const PairVector vec = object.as<PairVector>();
    }
    return 0;
}

struct Sample {
  std::vector<std::pair<uint64_t, uint64_t>> pairs;
  uint32_t index;

  MSGPACK_DEFINE(pairs, index);
};

int struct_sample() {
    Sample sample;
    sample.pairs.push_back(std::make_pair(12, 34));
    sample.index = 1;
    std::vector<Sample> sample_vector = { sample };

    msgpack::sbuffer sbuf;
    msgpack::pack(sbuf, sample_vector);
    msgpack::object_handle vector_object_handle = msgpack::unpack(sbuf.data(), sbuf.size());
    msgpack::object data = vector_object_handle.get();
    std::cout << data << std::endl;
    std::vector<Sample> deserialize_datas;
    data.convert(deserialize_datas);

    return 0;
}

int main() {

    std::cout << std::endl << "=== pair_sample === ===" << std::endl;
    pair_sample();

    std::cout << std::endl << "=== pair_vector_sample === ===" << std::endl;
    pair_vector_sample();

    std::cout << std::endl << "=== load_pair_vector_sample === ===" << std::endl;
    load_pair_vector_sample();

    std::cout << std::endl << "=== struct_sample === ===" << std::endl;
    struct_sample();

    return 0;
}


