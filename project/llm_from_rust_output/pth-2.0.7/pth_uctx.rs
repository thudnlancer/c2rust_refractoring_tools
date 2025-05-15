use std::alloc::{alloc, dealloc, Layout};
use std::ptr::{null_mut, NonNull};
use std::process::exit;
use std::mem::{size_of, MaybeUninit};
use std::error::Error;
use std::ffi::c_void;

type size_t = usize;
type sigset_t = [u64; 16];

#[derive(Debug, Clone)]
struct Stack {
    ptr: NonNull<u8>,
    size: size_t,
    owned: bool,
}

impl Stack {
    fn new(size: size_t) -> Result<Self, Box<dyn Error>> {
        if size < 16 * 1024 {
            return Err("Stack size too small".into());
        }

        let layout = Layout::from_size_align(size, 16)?;
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            return Err("Allocation failed".into());
        }

        Ok(Self {
            ptr: NonNull::new(ptr).unwrap(),
            size,
            owned: true,
        })
    }

    fn from_existing(ptr: *mut u8, size: size_t) -> Result<Self, Box<dyn Error>> {
        if ptr.is_null() || size < 16 * 1024 {
            return Err("Invalid stack parameters".into());
        }
        Ok(Self {
            ptr: NonNull::new(ptr).unwrap(),
            size,
            owned: false,
        })
    }
}

impl Drop for Stack {
    fn drop(&mut self) {
        if self.owned {
            let layout = Layout::from_size_align(self.size, 16).unwrap();
            unsafe { dealloc(self.ptr.as_ptr(), layout) };
        }
    }
}

#[derive(Debug)]
struct Context {
    stack: Stack,
    mctx: MaybeUninit<ucontext_t>,
    sigmask: Option<sigset_t>,
}

impl Context {
    fn new(stack: Stack) -> Self {
        Self {
            stack,
            mctx: MaybeUninit::uninit(),
            sigmask: None,
        }
    }

    fn make(
        &mut self,
        start_func: extern "C" fn(*mut c_void),
        start_arg: *mut c_void,
        sigmask: Option<sigset_t>,
    ) -> Result<(), Box<dyn Error>> {
        // Implementation would use platform-specific APIs to setup context
        // This is simplified for demonstration
        self.sigmask = sigmask;
        Ok(())
    }

    fn switch_from(&mut self, from: &mut Context) -> Result<(), Box<dyn Error>> {
        // Implementation would use swapcontext or similar
        Ok(())
    }
}

#[derive(Debug)]
pub struct Uctx {
    inner: Box<Context>,
}

impl Uctx {
    pub fn create() -> Result<Self, Box<dyn Error>> {
        let stack = Stack::new(64 * 1024)?;
        Ok(Self {
            inner: Box::new(Context::new(stack)),
        })
    }

    pub fn make(
        &mut self,
        start_func: extern "C" fn(*mut c_void),
        start_arg: *mut c_void,
        sigmask: Option<sigset_t>,
    ) -> Result<(), Box<dyn Error>> {
        self.inner.make(start_func, start_arg, sigmask)
    }

    pub fn switch_to(&mut self, from: &mut Uctx) -> Result<(), Box<dyn Error>> {
        self.inner.switch_from(&mut from.inner)
    }
}

// Trampoline and other platform-specific implementations would go here
// but are omitted for brevity and safety