use libc::{c_float, c_int, c_uint, c_void};
use std::ffi::CString;
use std::path::Path;

pub enum Index {}

mod ffi {
    use super::*;

    #[link(name = "binding", kind = "static")]
    extern "C" {
        pub fn init_index(n: c_uint) -> *mut Index;

        pub fn add_item(
            index: *mut Index,
            item: *const c_float,
            len: usize,
            is_some: c_int,
            id: c_uint,
        );

        pub fn dispose(index: *mut Index);

        pub fn query(
            index: *mut Index,
            input: *const c_float,
            len: usize,
            result: *mut usize,
            distances: *mut c_float,
            k: c_int,
            query_ef: c_int,
        );

        pub fn get_distance(
            index: *mut Index,
            item1: *const c_float,
            item2: *const c_float,
            len: usize,
        ) -> c_float;

        pub fn save_index(index: *mut Index, output_path: *const c_void) -> c_float;

        pub fn load_index_from_stream(index: *mut Index, input_path: *const c_void) -> c_float;
    }
}

pub struct Voyager<const N: usize> {
    ix: *mut Index,
}

impl<const N: usize> Voyager<N> {
    pub fn new() -> Self {
        let n = Voyager::<N>::dimension();
        let index = unsafe { ffi::init_index(n as u32) };

        Voyager { ix: index }
    }

    pub const fn dimension() -> usize {
        N
    }

    pub fn add_item(&self, w: [f32; N], id: Option<u32>) {
        let len = w.len();
        let is_some: c_int = id.is_some() as c_int;

        unsafe {
            ffi::add_item(self.ix, w.as_ptr(), len, is_some, id.unwrap_or(0));
        }
    }

    pub fn query(&self, w: [f32; N], k: i32, ef: Option<i32>) -> (Vec<usize>, Vec<f32>) {
        let len = w.len();

        let mut result = Vec::with_capacity(k as usize);
        let result_ptr = result.as_mut_ptr();

        let mut distance = Vec::with_capacity(k as usize);
        let distance_ptr = distance.as_mut_ptr();

        unsafe {
            ffi::query(
                self.ix,
                w.as_ptr(),
                len,
                result_ptr,
                distance_ptr,
                k,
                ef.unwrap_or(-1),
            );
        }

        let a = unsafe { std::slice::from_raw_parts_mut(result_ptr, k as usize) };
        let b = unsafe { std::slice::from_raw_parts_mut(distance_ptr, k as usize) };

        (a.to_vec(), b.to_vec())
    }

    pub fn get_distance(&self, w1: [f32; N], w2: [f32; N]) -> f32 {
        let len = w1.len();

        unsafe { ffi::get_distance(self.ix, w1.as_ptr(), w2.as_ptr(), len) }
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) {
        unsafe {
            if let Some(f) = path.as_ref().as_os_str().to_str() {
                let path_str_c = CString::new(f).unwrap();
                ffi::save_index(self.ix, path_str_c.as_ptr() as *const c_void);
            }
        }
    }

    pub fn load<P: AsRef<Path>>(&self, path: P) {
        unsafe {
            if let Some(f) = path.as_ref().as_os_str().to_str() {
                let path_str_c = CString::new(f).unwrap();
                ffi::load_index_from_stream(self.ix, path_str_c.as_ptr() as *const c_void);
            }
        }
    }
}

impl<const N: usize> Drop for Voyager<N> {
    fn drop(&mut self) {
        unsafe {
            ffi::dispose(self.ix);
        }
    }
}

impl<const N: usize> Default for Voyager<N> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_voyager() {
        let v = Voyager::new();

        let v1 = [1.0, 2.0, 3.0, 4.0, 5.0];
        let v2 = [6.0, 7.0, 8.0, 9.0, 10.0];

        v.add_item(v1, Some(1));
        v.add_item(v2, Some(2));

        let (result, distance) = v.query(v1, 2, None);

        assert!(result == vec![1, 2]);
        assert!(distance == vec![0.0, 125.0]);
    }

    #[test]
    fn test_distance() {
        let v = Voyager::new();

        let v1 = [1.0, 2.0, 3.0, 4.0, 5.0];
        let v2 = [6.0, 7.0, 8.0, 9.0, 10.0];

        let distance = v.get_distance(v1, v2);

        assert!(distance == 125.0);
    }

    #[test]
    fn test_save() {
        let v = Voyager::new();

        let v1 = [1.0, 2.0, 3.0, 4.0, 5.0];
        let v2 = [6.0, 7.0, 8.0, 9.0, 10.0];

        v.add_item(v1, Some(1));
        v.add_item(v2, Some(2));

        v.save("test.index");
    }

    #[test]
    fn test_load() {
        let v = Voyager::new();

        let v1 = [1.0, 2.0, 3.0, 4.0, 5.0];
        let v2 = [6.0, 7.0, 8.0, 9.0, 10.0];

        v.add_item(v1, Some(1));
        v.add_item(v2, Some(2));

        v.save("test.index");

        let new_v = Voyager::new();
        new_v.load("test.index");

        let (result, distance) = new_v.query(v1, 2, None);
        assert!(result == vec![1, 2]);
        assert!(distance == vec![0.0, 125.0]);
    }

    #[test]
    fn test_runtime() {
        unsafe {
            let index = ffi::init_index(5);

            let v1 = &[1.0, 2.0, 3.0, 4.0, 5.0];
            let v2 = &[6.0, 7.0, 8.0, 9.0, 10.0];

            let mut result = Vec::with_capacity(2);
            let result_ptr = result.as_mut_ptr();

            let mut distance = Vec::with_capacity(2);
            let distance_ptr = distance.as_mut_ptr();

            ffi::add_item(index, v1.as_ptr(), v1.len(), 1, 0);
            ffi::add_item(index, v2.as_ptr(), v2.len(), 1, 1);

            ffi::query(
                index,
                v1.as_ptr(),
                v1.len(),
                result_ptr,
                distance_ptr,
                2,
                -1,
            );

            let a = std::slice::from_raw_parts_mut(result_ptr, 2 as usize);
            let b = std::slice::from_raw_parts_mut(distance_ptr, 2 as usize);

            assert!(a.to_vec() == vec![0, 1]);
            assert!(b.to_vec() == vec![0.0, 125.0]);

            ffi::dispose(index);
        }
    }
}
