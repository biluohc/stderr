/** `Internal variability` for [`Lazystatic`](https://crates.io/crates/lazy_static)

## Example:

### On Cargo.toml:
```toml
lazy_static = "^0.2.8"
stderr = "0.8.1" 
```

### On Code:
```rust
#[macro_use]
extern crate lazy_static;
extern crate stderr;
use stderr::StaticMut;

lazy_static!{
    static ref STRING : StaticMut<String> = StaticMut::new(String::new());
    static ref USIZE : StaticMut<Usize> = StaticMut::new(Usize(0));
}

fn main() {
    // Before write, You can read it Concurrent safely.
    println!("{:?}", STRING.as_ref());
    println!("{:?}", USIZE.as_ref());

    let str = {
        let mut str = "StaticMut, 雪之下".to_string();
        // do some work
        str
    };
    // But when write it, operate it Concurrent is unsafe.
    {
        STRING.set(str); // update by setting value
        USIZE.as_mut().0 = 123; // update by modifying field
    }

    // After write, You can read it Concurrent safely.
    println!("{:?}", STRING.as_ref());
    println!("{:?}", USIZE.as_ref());
}

#[derive(Debug)]
struct Usize(usize);

```

## About safe and unsafe 

If you read or write it when you write it, I don't ensure anything.

You can add `AtomicXXX` to ensure safe on above situation also.

If you need full Concurrent read and write, you maybe need `RwLock`
*/
#[derive(Debug)]
pub struct StaticMut<T>(UnsafeCell<T>);

use std::marker::Sync;
use std::cell::UnsafeCell;
unsafe impl<T> Sync for StaticMut<T> {}

impl<T> StaticMut<T> {
    #[inline]    
    pub fn new(value: T) -> Self {
        StaticMut(UnsafeCell::new(value))
    }
    /// read it
    #[inline]
    #[allow(unknown_lints,should_implement_trait)]
    pub fn as_ref(&self) -> &T {
        unsafe { self.0.get().as_ref().unwrap() }
    }
    /// write it
    #[allow(unknown_lints,mut_from_ref)]
    #[inline]
    pub fn as_mut(&self) -> &mut T {
        unsafe { self.0.get().as_mut().unwrap() }
    }
    /// update it
    #[inline]
    pub fn set(&self, value:T) {
        *self.as_mut() = value
    }
    ///Unwraps the value
    #[inline]
    pub fn into_inner(self)->T {
        unsafe {self.0.into_inner()}
    }
}

