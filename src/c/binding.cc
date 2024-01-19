#include <vector>
#include <TypedIndex.h>
#include "binding.hpp"

IndexVm* init_index() {
  IndexVm* vm = new IndexVm {};
  vm->index_ = std::make_shared<voyager::Index>(SpaceType::Euclidean, 5);
  return vm;
}

void add_item(IndexVm* vm, float* vec_, size_t len, size_t is_some, size_t _id) {
  std::vector<float> v(vec_, vec_ + len);
  std::optional<size_t> id = is_some ? std::optional<size_t>(_id) : std::nullopt;

  vm->index_->AddItem(v, id);

  return ;
}

void dispose(IndexVm* vm) {
  delete vm;
}

void query(
    IndexVm* vm, float* item, size_t len, 
    size_t *result, 
    float *distances,
    int k, long queryEf
) {
    std::vector<float> input(item, item + len);

    auto idsAndDistances =
        vm->index_->Query(input, k, queryEf);

    auto resultV = std::vector<hnswlib::labeltype>(std::get<0>(idsAndDistances));
    auto distancesV = std::vector<float>(std::get<1>(idsAndDistances));

    std::copy(resultV.begin(), resultV.end(), result);
    std::copy(distancesV.begin(), distancesV.end(), distances);

    return ;
}

float get_distance(IndexVm* vm, float* a, float* b, size_t len) {
  std::vector<float> v1(a, a + len);
  std::vector<float> v2(b, b + len);

  return vm->index_->GetDistance(v1, v2);
}

void resize(IndexVm* vm, size_t size) {
  vm->index_->ResizeIndex(size);
}

void save_index(IndexVm* vm, const char* path) {
  vm->index_->SaveIndex(path);
}