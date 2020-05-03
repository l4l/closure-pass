#![feature(stmt_expr_attributes, proc_macro_hygiene)]

use closure_pass::closure_pass;

use std::rc::Rc;

fn main() {
    let rc = Rc::new(2);
    let mut string = "test".to_string();

    let f = #[closure_pass(rc, s = string.split_off(2))]
    move || {
        println!("{:?}", rc);
        println!("{:?}", s);
    };

    println!("{:?}", rc);
    println!("{:?}", string);

    f();
}
