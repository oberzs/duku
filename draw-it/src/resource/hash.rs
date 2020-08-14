use std::mem;
use std::slice;

const MOD_ADLER: u32 = 65521;

pub(crate) fn adler32<T>(data: &[T]) -> u32 {
    let size = mem::size_of::<T>() * data.len();
    let bytes = unsafe { slice::from_raw_parts(data as *const [T] as *const u8, size) };

    let mut a = 1;
    let mut b = 0;

    for byte in bytes {
        a = (a + *byte as u32) % MOD_ADLER;
        b = (b + a) % MOD_ADLER;
    }

    (b << 16) | a
}
