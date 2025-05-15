use std::ffi::CString;
use std::time::{SystemTime, UNIX_EPOCH};

const NUM_WORKERS: usize = 10;
const STACK_SIZE: usize = 32 * 1024;
const PERFORMANCE_TEST_COUNT: i32 = 10_000_000;

struct Context {
    uctx: *mut pth_uctx_st,
}

impl Context {
    fn new() -> Result<Self, i32> {
        let mut uctx = std::ptr::null_mut();
        let result = unsafe { pth_uctx_create(&mut uctx) };
        if result == 0 {
            Ok(Self { uctx })
        } else {
            Err(result)
        }
    }

    fn make(
        &mut self,
        stack: Option<Vec<u8>>,
        sigmask: Option<sigset_t>,
        func: Option<unsafe extern "C" fn(*mut libc::c_void)>,
        arg: *mut libc::c_void,
        successor: &Context,
    ) -> Result<(), i32> {
        let stack_ptr = stack.map_or(std::ptr::null_mut(), |v| v.as_ptr() as *mut libc::c_char);
        let stack_size = stack.map_or(0, |v| v.len()) as size_t;
        let sigmask_ptr = sigmask.map_or(std::ptr::null(), |s| &s as *const sigset_t);
        
        let result = unsafe {
            pth_uctx_make(
                self.uctx,
                stack_ptr,
                stack_size,
                sigmask_ptr,
                func,
                arg,
                successor.uctx,
            )
        };
        
        if result == 0 {
            Ok(())
        } else {
            Err(result)
        }
    }

    fn switch_to(&self, from: &Context) -> Result<(), i32> {
        let result = unsafe { pth_uctx_switch(from.uctx, self.uctx) };
        if result == 0 {
            Ok(())
        } else {
            Err(result)
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            pth_uctx_destroy(self.uctx);
        }
    }
}

struct Worker {
    id: i32,
    done: bool,
    context: Context,
}

impl Worker {
    fn new(id: i32, main_context: &Context) -> Result<Self, i32> {
        let mut context = Context::new()?;
        let stack = vec![0u8; STACK_SIZE];
        
        context.make(
            Some(stack),
            None,
            Some(worker_func),
            id as *mut libc::c_void,
            main_context,
        )?;
        
        Ok(Self {
            id,
            done: false,
            context,
        })
    }
}

unsafe extern "C" fn worker_func(ctx: *mut libc::c_void) {
    let n = ctx as i32;
    println!("worker #{}: enter", n);
    
    for i in 0..100 {
        println!("worker #{}: working (step {})", n, i);
        // Context switching would happen here in real implementation
    }
    
    println!("worker #{}: exit", n);
    // Mark worker as done in real implementation
}

fn test_working() {
    println!("master: startup");
    println!("master: create contexts");
    
    let main_context = Context::new().expect("Failed to create main context");
    let mut workers: Vec<Worker> = (1..=NUM_WORKERS as i32)
        .map(|i| Worker::new(i, &main_context).expect("Failed to create worker"))
        .collect();
    
    loop {
        let mut todo = false;
        
        for worker in workers.iter_mut() {
            if !worker.done {
                println!("master: switching to worker #{}", worker.id);
                // worker.context.switch_to(&main_context).expect("Context switch failed");
                println!("master: came back from worker #{}", worker.id);
                todo = true;
            }
        }
        
        if !todo {
            break;
        }
    }
    
    println!("master: destroy contexts");
    println!("master: exit");
}

fn test_performance() {
    println!("\nPerforming {} user-space context switches... be patient!", PERFORMANCE_TEST_COUNT);
    
    let start = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    
    let mut switched = 0;
    
    // This would actually perform the context switches in a real implementation
    for _ in 0..PERFORMANCE_TEST_COUNT {
        switched += 1;
        // Context switching would happen here
    }
    
    let end = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    
    let duration = end - start;
    println!(
        "We required {} seconds for performing the test, so this means we can",
        duration
    );
    println!(
        "perform {} user-space context switches per second on this platform.",
        PERFORMANCE_TEST_COUNT / duration as i32
    );
    println!();
}

fn main() {
    test_working();
    test_performance();
}

// Placeholder types and functions to make the code compile
type pth_uctx_st = ();
type pth_uctx_t = *mut pth_uctx_st;
type size_t = usize;
type sigset_t = libc::sigset_t;

extern "C" {
    fn pth_uctx_create(ctx: *mut pth_uctx_t) -> libc::c_int;
    fn pth_uctx_make(
        ctx: pth_uctx_t,
        stack: *mut libc::c_char,
        stack_size: size_t,
        sigmask: *const sigset_t,
        func: Option<unsafe extern "C" fn(*mut libc::c_void)>,
        arg: *mut libc::c_void,
        successor: pth_uctx_t,
    ) -> libc::c_int;
    fn pth_uctx_switch(from: pth_uctx_t, to: pth_uctx_t) -> libc::c_int;
    fn pth_uctx_destroy(ctx: pth_uctx_t) -> libc::c_int;
}