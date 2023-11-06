#pragma once

#include <memory>
#include "index.hpp"

#ifdef __cplusplus
extern "C" {
#endif
  typedef struct {
    std::shared_ptr<voyager::Index> index_;
  } IndexVm;

  IndexVm* init_index();

  void add_item(IndexVm* vm, float* item, size_t len, size_t _id);

  void dispose(IndexVm* vm);

  void query(IndexVm* vm, float* item, size_t len, size_t *result, float *distances,
    int k = 1, long queryEf = -1);

#ifdef __cplusplus
}
#endif