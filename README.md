# simple-yolo-sys

Rust FFI bindings for [shouxieai/tensorRT_Pro/example-simple_yolo](https://github.com/shouxieai/tensorRT_Pro/tree/main/example-simple_yolo). This is an example of creating an rust wrapper for [shouxieai/tensorRT_Pro](https://github.com/shouxieai/tensorRT_Pro/tree/main/). More idiomatic rust bindings could then be developed on top of this.

[libyolo](./libyolo/) is modified from [shouxieai/tensorRT_Pro/example-simple_yolo](https://github.com/shouxieai/tensorRT_Pro/tree/main/example-simple_yolo).

You can use this lib to call yolov5-tensorrt from rust.

[中文博客](http://t.csdn.cn/MmXA3)

# Instructions

## 1. get docker environment

Build your docker environment under the instructions of [https://hub.docker.com/r/hopef/tensorrt-pro](https://hub.docker.com/r/hopef/tensorrt-pro).

## 2. set config

Modify [libyolo/CMakeLists.txt](./libyolo/CMakeLists.txt) according to your own environment.

Set the correct CUDA_GEN_CODE with your own gpu. If you don't know you cuda gencode, you can look for it at [https://developer.nvidia.com/zh-cn/cuda-gpus#compute](https://developer.nvidia.com/zh-cn/cuda-gpus#compute). The gpu compute capability for my NVIDIA TITAN X is 6.1.
```
set(CUDA_GEN_CODE "-gencode=arch=compute_61,code=sm_61")
```

Set the correct paths of opencv, cuda, cudnn and tensorrt that you downloaded under the instructions of [https://hub.docker.com/r/hopef/tensorrt-pro](https://hub.docker.com/r/hopef/tensorrt-pro).
```
set(OpenCV_DIR   "/nfs/users/chenquan/packages/tensorrt_pro/data/lean/opencv-4.2.0/include/opencv4")
set(CUDA_DIR     "/nfs/users/chenquan/packages/tensorrt_pro/data/lean/cuda-11.2")
set(CUDNN_DIR    "/nfs/users/chenquan/packages/tensorrt_pro/data/lean/cudnn8.2.2.26")
set(TENSORRT_DIR "/nfs/users/chenquan/packages/tensorrt_pro/data/lean/TensorRT-8.0.3.4.cuda11.3.cudnn8.2")
```

## 3. build

Change the opencv path in [build.rs](./build.rs) used by `clang_arg`.

Then, simply run

```
cargo build
```

## 4. test

Modify the model and image paths in [src/ib.rs](./src/lib.rs).

Yolov5 onnx file are obtained under the instuctions of [shouxieai/tensorRT_Pro](https://github.com/shouxieai/tensorRT_Pro/tree/main/).

Now run the following command to compile the tensorrt engine

```
RUST_BACKTRACE=1 cargo test test_compile_tensorrt_engine --lib -- --nocapture
```

If you met a cudnn path not found error, export the path of cudnn in `LD_LIBRARY_PATH`

```
export LD_LIBRARY_PATH=/nfs/users/chenquan/packages/tensorrt_pro/data/lean/cudnn8.2.2.26/lib:$LD_LIBRARY_PATH
```

And run the following command to run the tensorrt engine

```
RUST_BACKTRACE=1 cargo test test_run_engine --lib -- --nocapture
```

# Reference

[1] [https://github.com/shouxieai/tensorRT_Pro](https://github.com/shouxieai/tensorRT_Pro)

[2] [https://github.com/alianse777/darknet-sys-rust](https://github.com/alianse777/darknet-sys-rust)
