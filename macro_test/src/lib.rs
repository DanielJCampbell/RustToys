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