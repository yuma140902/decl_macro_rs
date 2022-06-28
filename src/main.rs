macro_rules! add {
    ($e1: expr, $e2: expr) => {
        $e1 + $e2
    };
}

macro_rules! twice {
    ($f: ident, $arg: expr) => {
        $f($f($arg))
    };
}

fn main() {
    dbg!(add!(1, 2));
    dbg!(add!(1 + 1, -2));

    fn plus1(n: i32) -> i32 {
        n + 1
    }

    fn double(n: i32) -> i32 {
        n * 2
    }

    #[allow(dead_code)]
    fn f2(n: i32, m: i32) -> i32 {
        n + m
    }

    dbg!(twice!(plus1, 1));
    dbg!(twice!(double, 2));
    // dbg!(twice!(f2, 2));  // error[E0061]: this function takes 2 arguments but 1 argument was supplied
}
