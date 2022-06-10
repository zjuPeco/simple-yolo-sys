cmake_minimum_required(VERSION 2.6)
project(pro)
add_definitions(-std=c++11)

option(CUDA_USE_STATIC_CUDA_RUNTIME OFF)
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_BUILD_TYPE Debug)

# 如果你是不同显卡，请设置为显卡对应的号码参考这里：https://developer.nvidia.com/zh-cn/cuda-gpus#compute
set(CUDA_GEN_CODE "-gencode=arch=compute_61,code=sm_61")

# 如果你的opencv找不到，可以自己指定目录
set(OpenCV_DIR   "/nfs/users/chenquan/packages/tensorrt_pro/data/lean/opencv-4.2.0/include/opencv4")
set(CUDA_DIR     "/nfs/users/chenquan/packages/tensorrt_pro/data/lean/cuda-11.2")
set(CUDNN_DIR    "/nfs/users/chenquan/packages/tensorrt_pro/data/lean/cudnn8.2.2.26")
set(TENSORRT_DIR "/nfs/users/chenquan/packages/tensorrt_pro/data/lean/TensorRT-8.0.3.4.cuda11.3.cudnn8.2")

find_package(CUDA REQUIRED)
find_package(OpenCV)

include_directories(
    ${PROJECT_SOURCE_DIR}/src
    ${OpenCV_INCLUDE_DIRS}
    ${CUDA_DIR}/include
    ${TENSORRT_DIR}/include
    ${CUDNN_DIR}/include
)

# 切记，protobuf的lib目录一定要比tensorRT目录前面，因为tensorRTlib下带有protobuf的so文件
# 这可能带来错误
link_directories(
    ${TENSORRT_DIR}/lib
    ${CUDA_DIR}/lib64
    ${CUDNN_DIR}/lib
)

set(CMAKE_CXX_FLAGS  "${CMAKE_CXX_FLAGS} -std=c++11 -Wall -O0 -Wfatal-errors -pthread -w -g")
set(CUDA_NVCC_FLAGS "${CUDA_NVCC_FLAGS} -std=c++11 -O0 -Xcompiler -fPIC -g -w ${CUDA_GEN_CODE}")
file(GLOB_RECURSE cuda_srcs ${PROJECT_SOURCE_DIR}/src/*.cu)
cuda_add_library(cucodes SHARED ${cuda_srcs})

# 如果提示插件找不到，请使用dlopen(xxx.so, NOW)的方式手动加载可以解决插件找不到问题
target_link_libraries(cucodes nvinfer nvonnxparser)
target_link_libraries(cucodes cuda cublas cudart cudnn)
target_link_libraries(cucodes ${OpenCV_LIBS})

INSTALL(FILES src/simple_yolo.hpp DESTINATION include/simple_yolo)
INSTALL(TARGETS cucodes LIBRARY DESTINATION lib)
# export LD_LIBRARY_PATH=/nfs/users/chenquan/packages/tensorrt_pro/data/lean/cudnn8.2.2.26/lib:$LD_LIBRARY_PATH