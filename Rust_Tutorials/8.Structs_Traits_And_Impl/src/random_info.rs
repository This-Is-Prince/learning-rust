pub trait SomeTrait {
    fn is_valid(&self) -> bool;
}

#[derive(Debug, Copy, Clone)]
pub struct RandomInfo {
    pub call_count: i64,
    pub some_bool: bool,
    pub some_int: i64,
}

impl SomeTrait for RandomInfo {
    fn is_valid(&self) -> bool {
        self.some_bool
    }
}

impl RandomInfo {
    pub fn new(param_a: bool) -> Self {
        Self {
            call_count: 0,
            some_bool: !param_a,
            some_int: 0,
        }
    }

    pub fn is_smaller(&mut self, compare_to: i64) -> bool {
        self.call_count += 1;
        self.some_int < compare_to
    }
}

#[allow(dead_code)]
struct LookMaNoFields {}

// Generics Struct
#[allow(dead_code)]
struct Pair<T, U> {
    x: T,
    y: T,
    z: U,
}

// Tuple struct
#[allow(dead_code)]
struct Color(u8, u8, u8);