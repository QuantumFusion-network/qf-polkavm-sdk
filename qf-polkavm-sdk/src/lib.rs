#![no_std]
#![no_main]

/// This macro includes a bump allocator and imports PolkaVM host functions into the code.
///
/// List of Host Functions:
///
/// 1. `balance() -> u64`
/// Returns the balance of the smart contract's address.
///
/// 2. `balance_of() -> u64`
/// Returns the balance of a specified address.
///
/// 3. `print(msg_pointer: u32, len: u32) -> u64`
///   - `msg_pointer`: Pointer to the message buffer.
///   - `len`: Length of the message.
/// Prints a message to the runtime logs.
///
/// 4. `transfer(address_idx: u32, balance_idx: u32) -> u64`
///   - `address_idx`: Index in the `addresses` array.
///   - `balance_idx`: Index in the `balances` array.
/// Transfers balance from the smart contract's address to the specified address. Returns 0 on success.
///
/// 5. `block_number() -> u64`
/// Returns the current block number.
///
/// 6. `account_id() -> u64`
/// Returns the index of the smart contract's address in the `addresses` array.
///
/// 7. `caller() -> u64`
/// Returns the index of the caller's address in the `addresses` array.
///
/// 8. `storage_size() -> u64`
/// Returns the size of the storage.
///
/// 9. `get_address_len(address_idx: u32) -> u64`
///   - `address_idx`: Index in the `addresses` array.
/// Returns the length of the address at the specified index.
///
/// 10. `get_address(address_idx: u32, write_pointer: u32) -> u64`
///   - `address_idx`: Index in the `addresses` array.
///   - `write_pointer`: Pointer to write the address to.
/// Retrieves the address at the specified index and writes it to the given pointer.
///
/// 11. `get_user_data(pointer: u32) -> u64`
///   - `pointer`: Pointer to the user_data buffer.
/// Writes `user_data` to the specified pointer. Returns 0 on success.
///
/// 12. `get(storage_key_pointer: u32, pointer: u32) -> u64`
///   - `storage_key_pointer`: Pointer to a storage key.
///   - `pointer`: Pointer to a buffer where the data will be stored.
/// Reads data from storage using the key at `storage_key_pointer` and stores it at pointer. Returns 0 on success.
///
/// 13. `set(storage_key_pointer: u32, pointer: u32) -> u64`
///   - `storage_key_pointer`: Pointer to a storage key.
///   - `pointer`: Pointer to the data buffer.
/// Writes data from pointer to storage using the key at `storage_key_pointer`. Returns 0 on success.
///
/// 14. `delete(storage_key_pointer: u32) -> u64`
///   - `storage_key_pointer`: Pointer to a storage key.
/// Deletes data from storage using the specified key. Returns 0 on success.
///
/// [Instruction to compile a smart-contract](https://github.com/QuantumFusion-network/qf-polkavm-sdk/?tab=readme-ov-file#compiling-example-smart-contract)
///
#[macro_export]
macro_rules! host_functions {
    () => {
        use core::alloc::{GlobalAlloc, Layout};

        static mut INNER: Option<InnerAlloc> = None;

        /// Memory heap
        static mut HEAP: [u8; 1024 * 1024] = [0; 1024 * 1024];

        /// Simple bump allocator. Never deallocate memory
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

        // Host-functions available to call inside a smart-contract.
        #[polkavm_derive::polkavm_import]
        extern "C" {
            /// Returns the balance of the smart contract's address.
            fn balance() -> u64;

            /// Returns the balance of a specified address.
            fn balance_of() -> u64;

            /// Prints a message to the runtime logs.
            ///   - `msg_pointer`: Pointer to the message buffer.
            ///   - `len`: Length of the message.
            fn print(msg_pointer: u32, len: u32) -> u64;

            /// Transfers balance from the smart contract's address to the specified address. Returns 0 on success.
            ///   - `address_idx`: Index in the `addresses` array.
            ///   - `balance_idx`: Index in the `balances` array.
            fn transfer(address_idx: u32, balance_idx: u32) -> u64;

            /// Returns the current block number.
            fn block_number() -> u64;

            /// Returns the index of the smart contract's address in the addresses array.
            fn account_id() -> u64;

            /// Returns the index of the caller's address in the addresses array.
            fn caller() -> u64;

            /// Returns the size of the storage.
            fn storage_size() -> u64;

            /// Returns the length of the address at the specified index.
            ///   - `address_idx`: Index in the `addresses` array.
            fn get_address_len(address_idx: u32) -> u64;

            /// Retrieves the address at the specified index and writes it to the given pointer.
            ///   - `address_idx`: Index in the `addresses` array.
            ///   - `write_pointer`: Pointer to write the address to.
            fn get_address(address_idx: u32, write_pointer: u32) -> u64;

            /// Writes `user_data` to the specified pointer. Returns 0 on success.
            ///   - `pointer`: Pointer to the user_data buffer.
            fn get_user_data(pointer: u32) -> u64;

            /// Reads data from storage using the key at `storage_key_pointer` and stores it at pointer. Returns 0 on success.
            ///   - `storage_key_pointer`: Pointer to a storage key.
            ///   - `pointer`: Pointer to a buffer where the data will be stored.
            fn get(storage_key_pointer: u32, pointer: u32) -> u64;

            /// Writes data from pointer to storage using the key at `storage_key_pointer`. Returns 0 on success.
            ///   - `storage_key_pointer`: Pointer to a storage key.
            ///   - `pointer`: Pointer to the data buffer.
            fn set(storage_key_pointer: u32, pointer: u32) -> u64;

            /// Deletes data from storage using the specified key. Returns 0 on success.
            ///   - `storage_key_pointer`: Pointer to a storage key.
            fn delete(storage_key_pointer: u32) -> u64;
        }
    };
}
