#pragma once

#include <memory>
#include <TypedIndex.h>

namespace voyager {
  class Index {
    public:
      Index(SpaceType space, const int num_dimensions)
        : _space(space), 
        _num_dimensions(num_dimensions), 
        _storageDataType(StorageDataType::Float32) {
        index_ = std::make_shared<::TypedIndex<float, ::E4M3>>(
          space, 
          num_dimensions,
          12, 
          200, 
          1, 
          1
        );
      }

      ~Index() = default;

      void AddItem(std::vector<float> v, std::optional<size_t> i) {
        index_->addItem(v, i);
      }

      std::tuple<std::vector<hnswlib::labeltype>, std::vector<float>> 
        Query(std::vector<float> queryVectors, int k = 1, long queryEf = -1) {
        return index_->query(queryVectors, k, queryEf);
      }

      float GetDistance(std::vector<float> v1, std::vector<float> v2) {
        return index_->getDistance(v1, v2);
      }

      void ResizeIndex(size_t size) {
        index_->resizeIndex(size);
      }

      void SaveIndex(const char* path) {
        index_->saveIndex(path);
      }
      
      void LoadIndex(const char* filename) {
        index_ = loadTypedIndexFromStream(
            std::make_shared<FileInputStream>(filename));        
      }

    private:
      std::shared_ptr<::Index> index_;

      SpaceType _space;
      StorageDataType _storageDataType;

      int _num_dimensions;
  };
}
;
