// concept_14_06_dry — DRY: generate whole functions (and tests) with macros.
// Run:   cargo run  --bin concept_14_06_dry     (shows it in action)
// Test:  cargo test --bin concept_14_06_dry     (runs the generated #[test]s)


/*
 DRY ( Don't Repeat Yourself )
   - one small macro replaces many near-identical functions.


 in this example we are using the `tt` (token tree) designator
*/

use std::ops::{Add, Mul, Sub};



/*
 the generated code for below macro when called - assert_equal_len!(); // take example args..when this is called from op!(add_assign, Add, +=, add);

 assert_equal_len!(xs, ys, add_assign, +=); expands to:

    assert!(
        xs.len() == ys.len(),
        "{:?}: dimension mismatch: {:?} {:?} {:?}",
        "add_assign",      // stringify!(add_assign)
        (xs.len(),),
        "+=",              // stringify!(+=)  <- $op captured as `tt`
        (ys.len(),)
    );

*/

macro_rules! assert_equal_len {
    // `tt` (token tree) captures operators/tokens like `+=` that are neither
    // an `expr` nor an `ident`. Here $op is used ONLY to print in the message.
    ($a:expr, $b:expr, $func:ident, $op:tt) => {
        assert!(
            $a.len() == $b.len(),
            "{:?}: dimension mismatch: {:?} {:?} {:?}",
            stringify!($func),   // e.g. "add_assign"
            ($a.len(),),
            stringify!($op),     // e.g. "+="  <- why $op must be `tt`
            ($b.len(),)
        );
    };
}


/*
 the generated code for below macro when called - op!(add_assign, Add, +=, add);  expands to:

    fn add_assign<T: Add<T, Output = T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
        // assert_equal_len!(xs, ys, add_assign, +=) expands further to the
                // assert!(...) shown in the block above.
                    assert!(
                        xs.len() == ys.len(),
                        "{:?}: dimension mismatch: {:?} {:?} {:?}",
                        "add_assign",
                        (xs.len(),),
                        "+=",
                        (ys.len(),)
                    );

        for (x, y) in xs.iter_mut().zip(ys.iter()) {
            *x = Add::add(*x, *y);   // $bound::$method -> Add::add
        }
    }
*/

macro_rules! op {
    ($func:ident, $bound:ident, $op:tt, $method:ident) => {
        fn $func<T: $bound<T, Output = T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_equal_len!(xs, ys, $func, $op); // expands again -> assert!(...)

            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                // $bound::$method  ->  Add::add, Mul::mul, Sub::sub
                *x = $bound::$method(*x, *y);
            }
        }
    };
}

// Generate three near-identical functions from one 6-line macro.
op!(add_assign, Add, +=, add);
op!(mul_assign, Mul, *=, mul);
op!(sub_assign, Sub, -=, sub);

fn main() {
    let mut a = vec![1u32, 2, 3];
    add_assign(&mut a, &vec![10, 20, 30]);
    println!("add_assign: {a:?}"); // [11, 22, 33]

    let mut b = vec![2u32, 3, 4];
    mul_assign(&mut b, &vec![2, 2, 2]);
    println!("mul_assign: {b:?}"); // [4, 6, 8]

    let mut c = vec![9u32, 8, 7];
    sub_assign(&mut c, &vec![1, 2, 3]);
    println!("sub_assign: {c:?}"); // [8, 6, 4]
}

#[cfg(test)]
mod test {
    use std::iter;

    // Another DRY macro: generate a full `#[test]` fn per invocation.
    macro_rules! test {
        ($func:ident, $x:expr, $y:expr, $z:expr) => {
            #[test]
            fn $func() {
                for size in 0usize..10 {
                    let mut x: Vec<_> = iter::repeat($x).take(size).collect();
                    let y: Vec<_> = iter::repeat($y).take(size).collect();
                    let z: Vec<_> = iter::repeat($z).take(size).collect();

                    super::$func(&mut x, &y); // call the fn made by op!
                    assert_eq!(x, z);
                }
            }
        };
    }

    // [x,x,..] OP [y,y,..] should equal [z,z,..] for sizes 0..10.
    test!(add_assign, 1u32, 2u32, 3u32);
    test!(mul_assign, 2u32, 3u32, 6u32);
    test!(sub_assign, 3u32, 2u32, 1u32);
}
