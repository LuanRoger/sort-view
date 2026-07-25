use re_memory::AccountingAllocator;

#[global_allocator]
pub static GLOBAL: AccountingAllocator<std::alloc::System> =
    AccountingAllocator::new(std::alloc::System);
