include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::os::raw::c_int;
    use opencv::prelude::*;
    use opencv::imgcodecs;
    use opencv::core::{Mat, Scalar};
    use opencv::core::{CV_8UC3};

    // RUST_BACKTRACE=1 cargo test test_compile_tensorrt_engine --lib -- --nocapture
    #[test]
    fn test_compile_tensorrt_engine() {
        unsafe{
            let mode = SimpleYolo_Mode_FP32;
            let typ = SimpleYolo_Type_V5;
            let max_batch_size = 16;
            let onnx_file = CString::new("/nfs/users/chenquan/table_detection/yolov5m_v6_d5_clean/export/best.onnx").unwrap();
            let onnx_file = onnx_file.as_ptr();
            let model_file = CString::new("/nfs/users/chenquan/table_detection/yolov5m_v6_d5_clean/export/table_detection.trtmodel").unwrap();
            let model_file = model_file.as_ptr();
            let int8_image_folder = CString::new("").unwrap().as_ptr();
            let int8_cache_file = CString::new("").unwrap().as_ptr();
            let max_workspace_size = 1 << 30;
            let flag = SimpleYolo_compile(
                mode,
                typ,
                max_batch_size,
                onnx_file,
                model_file,
                max_workspace_size,
                int8_image_folder,
                int8_cache_file
            );
            println!("flag: {}", flag);
        }
    }

    // RUST_BACKTRACE=1 cargo test test_bbox --lib -- --nocapture
    #[test]
    fn test_bbox() {
        unsafe{
            let bbox = SimpleYolo_Box{
                left: 0.0,
                right: 0.0,
                top: 100.0,
                bottom: 50.0,
                confidence: 0.9,
                class_label: 0,
            };
            let mut bboxes = [bbox];
            SimpleYolo_show_boxes(bboxes.as_mut_ptr(), bboxes.len() as c_int);
        }
    }

    // RUST_BACKTRACE=1 cargo test test_opencv --lib -- --nocapture
    #[test]
    fn test_opencv() {
        unsafe{
            let mat = Mat::new_rows_cols_with_default(
                2, 3, CV_8UC3, Scalar::from((3.0, 2.0, 1.0))
            ).unwrap();
            let mat = mat.into_raw() as *const cv_Mat;
            SimpleYolo_show_mat_shape(mat);
        }
    }

    // RUST_BACKTRACE=1 cargo test test_run_engine --lib -- --nocapture
    #[test]
    fn test_run_engine() {
        unsafe{
            let engine_file = CString::new("/nfs/users/chenquan/tensorRT_Pro/ffi-test/table_detection.trtmodel").unwrap();
            let engine_file = engine_file.as_ptr();
            let typ = SimpleYolo_Type_V5;
            let infer = SimpleYolo_create_infer(
                engine_file,
                typ,
                0,
                0.4,
                0.5
            );
            let path = "/nfs/users/chenquan/tensorRT_Pro/simple_yolo/workspace/inference2/南京银行18_68.jpg";
            let image = imgcodecs::imread(path, imgcodecs::IMREAD_COLOR).unwrap();
            let image = image.into_raw() as *const cv_Mat;
            let prediction = SimpleYolo_predict(infer, image);
            let results = std::slice::from_raw_parts((*prediction).results, (*prediction).length as usize);
            for result in results{
                let pred = std::slice::from_raw_parts(*result, 6);
                println! ("pred: {:?}", pred);
            }
            SimpleYolo_reset_engine(infer);
        }
    }
}
