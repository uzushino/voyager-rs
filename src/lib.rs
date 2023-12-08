use libc::{c_float, c_int, c_uint};

pub enum Index {}

mod ffi {
    use super::*;

    #[link(name = "binding", kind = "static")]
    extern "C" {
        pub fn init_index() -> *mut Index;
        pub fn add_item(index: *mut Index, item: *const c_float, len: usize, size: c_uint);
        pub fn dispose(index: *mut Index);

        #[allow(clippy::all, dead_code)]
        pub fn query(
            index: *mut Index,
            input: *const c_float,
            len: usize,
            result: *mut usize,
            distances: *mut c_float,
            k: c_int,
            query_ef: c_int,
        );
    }
}

pub struct Voyager(usize, *mut Index);

impl Voyager {
    pub fn new(n: usize) -> Self {
        let index = unsafe { ffi::init_index() };
        Voyager(n, index)
    }

    pub fn add_item(&self, w: &[f32]) {
        let len = w.len();
        let size = self.0;

        unsafe {
            ffi::add_item(self.1, w.as_ptr(), len, size as c_uint);
        }
    }
}

impl Drop for Voyager {
    fn drop(&mut self) {
        unsafe {
            ffi::dispose(self.1);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_runtime() {
        unsafe {
            let index = ffi::init_index();
            let v1 = &[1.0, 2.0, 3.0, 4.0, 5.0];
            let v2 = &[6.0, 7.0, 8.0, 9.0, 10.0];

            let mut result = Vec::with_capacity(2);
            let result_ptr = result.as_mut_ptr();

            let mut distance = Vec::with_capacity(2);
            let distance_ptr = distance.as_mut_ptr();

            ffi::add_item(index, v1.as_ptr(), v1.len(), 0);
            ffi::add_item(index, v2.as_ptr(), v2.len(), 1);

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
