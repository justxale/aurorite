use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct HeapCounter {
    heap_size: AtomicUsize
}

impl HeapCounter {
    pub fn heap_size(&self) -> usize {
        self.heap_size.load(Ordering::Relaxed)
    }
}

unsafe impl GlobalAlloc for HeapCounter {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = unsafe { System.alloc(layout) };
        if !ret.is_null() {
            self.heap_size.fetch_add(layout.size(), Ordering::Relaxed);
        }
        ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) };
        self.heap_size.fetch_sub(layout.size(), Ordering::Relaxed);
    }
}

#[global_allocator]
pub static GLOBAL: HeapCounter = HeapCounter { heap_size: AtomicUsize::new(0) };