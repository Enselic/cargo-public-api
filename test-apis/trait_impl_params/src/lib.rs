// Test case for issue #766: underscore prefixes in trait impl parameters

pub trait MyTrait {
    fn trait_method(a: i32, b: String);
}

pub struct MyStruct;

impl MyTrait for MyStruct {
    fn trait_method(a: i32, _b: String) {
        // The _b parameter here should be normalized to b in the public API
    }
}

// Control: inherent impl should NOT normalize
impl MyStruct {
    pub fn inherent_method(_x: i32, y: String) {
        // The _x parameter here should NOT be normalized
    }
}

// Control: free function should NOT normalize
pub fn free_function(_p: i32, q: String) {
    // The _p parameter here should NOT be normalized
}
