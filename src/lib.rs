#![no_std]
#![no_main]
/// This macro includes a bump allocator and imports of PolkaVM host-functions:
/// 1. `balance() -> u64` returns Balance of smart-contract address.
///
/// 2. `balance_of() -> u64` returns of some address.
///
/// 3. `transfer(address_idx: u32, balance_idx: u32) -> u64` gets:
/// - `address_idx` is index in `addresses`.
/// - `balance_idx` is index in `balances`.
/// This function does transfer from smart-contract address to some address, returns 0 if transfer is
/// successful.
///
/// 4. `block_number() -> u64` returns block number.
///
/// 5. `account_id() -> u64` returns index of smart-contract address in `addresses`.
///
/// 6. `caller() -> u64` returns index of caller address in `addresses`.
///
/// 7. `get_user_data(pointer: u32) -> u64` gets:
/// - `pointer` is pointer to `user_data`.
/// This function puts `user_user` data by the `pointer` and returns 0 if there are not errors.
///
/// 8. `get(storage_key_pointer: u32, pointer: u32) -> u64` gets:
/// - `storage_key_pointer` is a pointer to some key in storage.
/// - `pointer` is a pointer to some buffer where data will be stored.
/// This function reads data from storage by the pointer of key (`storage_key_pointer`) and
/// stores it by `pointer`, returns 0 if there are not errors.
///
/// 9. `set(storage_key_pointer: u32, pointer: u32) -> u64` gets:
/// - `storage_key_pointer` is a pointer to some key in storage.
/// - `pointer` is a pointer to some buffer with data.
/// This function writes data from `pointer` to storage by the pointer of key
/// (`storage_key_pointer`), returns 0 if there are not errors.
///
/// 10. `delete(storage_key_pointer: u32) -> u64` gets:
/// - `storage_key_pointer` is pointer to some key in storage.
/// This function delete date from storage by pointer `storage_key_pointer`, 
/// returns 0 if there are not errors.
///
/// [Instruction to compile a smart-contract](https://github.com/QuantumFusion-network/qf-polkavm-sdk/?tab=readme-ov-file#compiling-example-smart-contract)
///
#[macro_export]
macro_rules! host_functions {
    () => {
        use core::alloc::{GlobalAlloc, Layout};

        static mut INNER: Option<InnerAlloc> = None;

        static mut HEAP: [u8; 1024 * 1024] = [0; 1024 * 1024];

        #[global_allocator]
        static ALLOCATOR: BumpAllocator = BumpAllocator {};

        pub struct BumpAllocator;

        unsafe impl GlobalAlloc for BumpAllocator {
            #[inline]
            #[allow(static_mut_refs)]
            unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
                if INNER.is_none() {
                    INNER = Some(InnerAlloc::new());
                };
                match INNER
                    .as_mut()
                    .expect("We just set the value above; qed")
                    .alloc(layout)
                {
                    Some(start) => start as *mut u8,
                    None => core::ptr::null_mut(),
                }
            }

            #[inline]
            unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
                // todo
                // A new page is guaranteed to already be zero initialized, so we can just
                // use our regular `alloc` call here and save a bit of work.
                //
                // See: https://webassembly.github.io/spec/core/exec/modules.html#growing-memories
                self.alloc(layout)
            }

            #[inline]
            unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
        }

        #[derive(Debug, Copy, Clone)]
        struct InnerAlloc {
            /// Points to the start of the next available allocation.
            next: usize,

            /// The address of the upper limit of our heap.
            upper_limit: usize,
        }

        impl InnerAlloc {
            fn new() -> Self {
                Self {
                    next: Self::heap_start(),
                    upper_limit: Self::heap_end(),
                }
            }

            fn heap_start() -> usize {
                #[allow(static_mut_refs)]
                unsafe {
                    HEAP.as_mut_ptr() as usize
                }
            }

            #[allow(static_mut_refs)]
            fn heap_end() -> usize {
                Self::heap_start() + unsafe { HEAP.len() }
            }

            #[allow(dead_code)]
            fn request_pages(&mut self, _pages: usize) -> Option<usize> {
                // On riscv the memory can't be grown
                core::panic!("no request possible");
            }

            /// Tries to allocate enough memory on the heap for the given `Layout`. If there is
            /// not enough room on the heap it'll try and grow it by a page.
            ///
            /// Note: This implementation results in internal fragmentation when allocating across
            /// pages.
            fn alloc(&mut self, layout: Layout) -> Option<usize> {
                let alloc_start = self.align_ptr(&layout);

                let aligned_size = layout.size();

                let alloc_end = alloc_start.checked_add(aligned_size)?;

                if alloc_end > self.upper_limit {
                    panic!("exhausted heap limit");
                } else {
                    self.next = alloc_end;
                    Some(alloc_start)
                }
            }

            /// Aligns the start pointer of the next allocation.
            ///
            /// We inductively calculate the start index
            /// of a layout in the linear memory.
            /// - Initially `self.next` is `0`` and aligned
            /// - `layout.align() - 1` accounts for `0` as the first index.
            /// - the binary with the inverse of the align creates a bitmask that is used to zero
            ///   out bits, ensuring alignment according to type requirements and ensures that the
            ///   next allocated pointer address is of the power of 2.
            #[allow(clippy::arithmetic_side_effects)] // todo
            fn align_ptr(&self, layout: &Layout) -> usize {
                (self.next + layout.align() - 1) & !(layout.align() - 1)
            }
        }

        /// Calculates the number of pages of memory needed for an allocation of `size` bytes.
        ///
        /// This function rounds up to the next page.
        #[inline]
        #[allow(dead_code)]
        fn required_pages(_size: usize) -> Option<usize> {
            core::panic!("required_pages");
        }

        #[panic_handler]
        fn panic(_info: &core::panic::PanicInfo) -> ! {
            unsafe {
                core::arch::asm!("unimp", options(noreturn));
            }
        }

        /// Host-functions available to call inside a smart-contract.
        #[polkavm_derive::polkavm_import]
        extern "C" {
            /// Balance of smart-contract address.
            fn balance() -> u64;

            /// Balance of some address.
            fn balance_of() -> u64;

            /// Transfer from smart-contract address to some address.
            /// - `address_idx` is index in addresses.
            /// - `balance_idx` is index in balances.
            fn transfer(address_idx: u32, balance_idx: u32) -> u64;

            /// Block number.
            fn block_number() -> u64;

            /// Smart-contract address.
            fn account_id() -> u64;

            /// Caller address.
            fn caller() -> u64;

            /// Get user_data.
            /// `pointer` is pointer to `user_data`.
            fn get_user_data(pointer: u32) -> u64;

            /// Get data from storage.
            /// - `storage_key_pointer` is a pointer to some key in storage.
            /// - `pointer` is a pointer to some buffer where data will be stored.
            fn get(storage_key_pointer: u32, pointer: u32) -> u64;

            /// Set data to storage.
            /// - `storage_key_pointer` is a pointer to some key in storage.
            /// - `pointer` is a pointer to some buffer with data.
            fn set(storage_key_pointer: u32, pointer: u32) -> u64;

            /// Delete data from storage.
            /// - `storage_key_pointer` is pointer to some key in storage.
            fn delete(storage_key_pointer: u32) -> u64;
        }
    };
}
