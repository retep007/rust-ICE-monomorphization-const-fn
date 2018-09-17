#![feature(const_fn)]

pub struct A(B);

struct B {
    a: Option<Function>
}

pub type Function = unsafe extern "C" fn(x: bool) -> bool;


impl A {
    pub const fn throw() -> Self {
        A (B {
            a: Some(unsafefn)
        })
    }
}

unsafe extern "C" fn unsafefn(x: bool) -> bool {
    return false;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
