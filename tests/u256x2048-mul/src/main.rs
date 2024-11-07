#![no_main]
sp1_zkvm::entrypoint!(main);

use num::BigUint;
use sp1_zkvm::syscalls::syscall_u256x2048_mul;

fn u256_to_bytes_le(x: &BigUint) -> [u8; 32] {
    let mut bytes = x.to_bytes_le();
    bytes.resize(32, 0);
    bytes.try_into().unwrap()
}

fn u2048_to_bytes_le(x: &BigUint) -> [u8; 256] {
    let mut bytes = x.to_bytes_le();
    bytes.resize(256, 0);
    bytes.try_into().unwrap()
}

pub fn main() {
    let mut a_max: [u8; 32] = [0xff; 32];
    let mut b_max: [u8; 256] = [0xff; 256];

    let a_max_big = BigUint::from_bytes_le(&a_max);
    a_max = u256_to_bytes_le(&a_max_big);
    let b_max_big = BigUint::from_bytes_le(&b_max);
    b_max = u2048_to_bytes_le(&b_max_big);

    let mut lo_max: [u32; 64] = [0; 64];
    let mut hi_max: [u32; 8] = [0; 8];

    syscall_u256x2048_mul(
        a_max.as_ptr() as *const [u32; 8],
        b_max.as_ptr() as *const [u32; 64],
        lo_max.as_mut_ptr() as *mut [u32; 64],
        hi_max.as_mut_ptr() as *mut [u32; 8],
    );

    let lo_max_bytes: [u8; 256] = bytemuck::cast::<[u32; 64], [u8; 256]>(lo_max);
    let hi_max_bytes: [u8; 32] = bytemuck::cast::<[u32; 8], [u8; 32]>(hi_max);

    let lo_max_big = BigUint::from_bytes_le(&lo_max_bytes);
    let hi_max_big = BigUint::from_bytes_le(&hi_max_bytes);

    let result_max_syscall = (hi_max_big << 2048) + lo_max_big;
    let result_max = a_max_big * b_max_big;
    assert_eq!(result_max, result_max_syscall);

    println!("All tests passed successfully!");
}
