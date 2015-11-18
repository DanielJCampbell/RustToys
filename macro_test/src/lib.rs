//Designed to emulate fizz-buzz, or any variant thereof (Returns a string)
//First expr must be some kind of integer iterator (eg. a range)
//Then a sequence of tuples of the form (number, string).
//For every number in the range, will append a line: either all strings
//for the numbers it is a multiple of, or just the number itself
macro_rules! fizz_buzz {
    ($e:expr, $(($n:expr, $s:expr)),*) => {{
        let mut output = "".to_owned();
        for i in $e {
            let mut tmp = "".to_owned();
            $(
                if i % $n == 0 {
                    tmp.push_str($s);
                } 
            )*
            if tmp.len() == 0 {
                tmp = i.to_string();
            }
            tmp.push_str("\n");
            output.push_str(&tmp);
        }
        output
    }};
}

//Because I really like the C style ternary operator
macro_rules! tern {
    ($id: ident = ($cond: expr) ? $e1: expr ; $e2: expr) => {{

        if $cond {
            $id = $e1;
        }
        else {
            $id = $e2;
        }
    }};
}

//This macro has to define a macro by the name $fn! (has to be macro because arg order not important, and types not known)
//That macro calls $fn using either default args or args passed in (passed in for required args)
//So $fn! has to know arg order, it has to know which are default, it has to know the default values.

//To minimise space, we can define n*2 variables, where n is the number of arguments
//The first n are named according to the formula tmp_ + $ident + [param index].
//(Each one stores an Option holding its default value, or None if required)
//The other n are named tmp_ + [param_index], and they are a mut reference to their pair variable

//Syntax of $fn!: $fn!($($expr | $ident = $expr),*)

//Functionality of $fn! - stores a count value, incremented every time around $(...)*
//If it matches just an expr, it finds the appropriate index variable and stores  Some(expr) in in it.
//If it matches an ident = expr, it stores Some(expr) in the appropriate ident variable

//After that, we go through all the index variables - if any are none, we panic, else we call $fn

//It is necessary that def_fn takes an expr - the current value of count (initialise)


//Define a function with optional parameters (assumes base form of function exists)
macro_rules! def_fn {

    //def_fn called with a required param first
    ($func: ident($param: ident, $($rest: tt)*)) => {{
        let mut tmp_$param = None;
        let &mut tmp_0 = &tmp_$param;
        def_fn_internal!(0; $func($($rest)*));
    }};

    //def_fn called with an optional param first
    ($func: ident($param: ident = $val: expr, $($rest: tt)*)) => {{
        let mut tmp_$param = $val;
        let &mut tmp_0 = &tmp_$param;
        def_fn_internal!(0; $func($($rest)*));
    }};
}

//This could be created inside the other macro, but too much duplication
macro_rules! def_fn_internal {

    //Required param, empty
    ($n: expr; $func:ident($param: ident)) => {{

    }};

    //Required param, non-empty
    ($n: expr; $func:ident($param: ident, $($rest: tt)*)) => {{

    }};



    //Optional param, empty
    ($n: expr; $func: ident($param: ident = $val: expr)) => {{

    }};

    //Optional param, non-empty
    ($n: expr; $func: ident($param: ident = $val: expr, $($rest: tt)*)) => {{

    }};

    //Empty
    ($n: expr; $func:ident()) => {{

    }};
}

#[test]
fn macro_fizzbuzz() {
    let fizzbuzz = fizz_buzz!(1..16, (3, "Fizz"), (5, "Buzz"));
    assert_eq!(fizzbuzz, "1\n2\nFizz\n4\nBuzz\nFizz\n7\n8\nFizz\nBuzz\n11\nFizz\n13\n14\nFizzBuzz\n");
}

#[test]
fn macro_tern() {
    let mut x = 0;
    tern!(x = (true) ? 1 ; 2);
    assert_eq!(x, 1);
    tern!(x = (false) ? 1 ; 2);
    assert_eq!(x, 2);
}