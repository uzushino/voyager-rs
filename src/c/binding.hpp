#pragma once

#include <memory>
#include "index.hpp"

#ifdef __cplusplus
extern "C" {
#endif
  typedef struct {
    std::shared_ptr<voyager::Index> index_;
  } IndexVm;

  IndexVm* init_index(size_t num_dimensions);

  void add_item(IndexVm* vm, float* item, size_t len, size_t is_some, size_t _id);

  void dispose(IndexVm* vm);

  void query(IndexVm* vm, float* item, size_t len, size_t *result, float *distances,
    int k = 1, long queryEf = -1);

  float get_distance(IndexVm* vm, float* a, float* b, size_t len);

  void save_index(IndexVm* vm, const char* path);

#ifdef __cplusplus
}
#endif