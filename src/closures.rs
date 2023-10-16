pub trait MyFnOnce<Arg> {
    type Output;

    fn call_once(self, arg: Arg) -> Self::Output;
}

pub trait MyFnMut<Arg>: MyFnOnce<Arg> {
    fn call_mut(&mut self, arg: Arg) -> Self::Output;
}

pub trait MyFn<Arg>: MyFnMut<Arg> {
    fn call(&self, arg: Arg) -> Self::Output;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_once_closure() {
        let s = String::from("a");
        let closure = |arg| {
            let captured_s = s;
            drop(captured_s);
            arg
        };

        // print(&s);
        closure(1);
        // closure(2);
    }

    #[test]
    fn my_fn_once_closure() {
        struct MyFnOnceClosure {
            captured_s: String,
        }

        impl MyFnOnce<i32> for MyFnOnceClosure {
            type Output = i32;

            fn call_once(self, arg: i32) -> i32 {
                drop(self.captured_s);
                arg
            }
        }

        let s = String::from("a");
        let closure = MyFnOnceClosure { captured_s: s };

        // print(&s);
        closure.call_once(1);
        // closure.call_once(2);
    }

    #[test]
    fn fn_mut_closure() {
        let mut s = String::from("a");
        let mut closure = |arg| {
            let captured_s = &mut s;
            captured_s.push('b');
            arg
        };

        // print(&s);
        closure(1);
        closure(2);
    }

    #[test]
    fn my_fn_mut_closure() {
        struct MyFnMutClosure<'s> {
            captured_s: &'s mut String,
        }

        impl MyFnOnce<i32> for MyFnMutClosure<'_> {
            type Output = i32;

            fn call_once(mut self, arg: i32) -> i32 {
                self.call_mut(arg)
            }
        }

        impl MyFnMut<i32> for MyFnMutClosure<'_> {
            fn call_mut(&mut self, arg: i32) -> i32 {
                self.captured_s.push('b');
                arg
            }
        }

        let mut s = String::from("a");
        let mut closure = MyFnMutClosure { captured_s: &mut s };

        // print(&s);
        closure.call_mut(1);
        closure.call_mut(2);
    }

    #[test]
    fn fn_closure() {
        let s = String::from("a");
        let closure = |arg| {
            let captured_s = &s;
            print(captured_s);
            arg
        };

        print(&s);
        closure(1);
        closure(2);
    }

    #[test]
    fn my_fn_closure() {
        struct MyFnClosure<'s> {
            captured_s: &'s String,
        }

        impl MyFnOnce<i32> for MyFnClosure<'_> {
            type Output = i32;

            fn call_once(self, arg: i32) -> i32 {
                self.call(arg)
            }
        }

        impl MyFnMut<i32> for MyFnClosure<'_> {
            fn call_mut(&mut self, arg: i32) -> i32 {
                self.call(arg)
            }
        }

        impl MyFn<i32> for MyFnClosure<'_> {
            fn call(&self, arg: i32) -> i32 {
                print(self.captured_s);
                arg
            }
        }

        let s = String::from("a");
        let closure = MyFnClosure { captured_s: &s };

        print(&s);
        closure.call(1);
        closure.call(2);
    }

    #[test]
    fn move_fn_closure() {
        let s = String::from("a");
        let closure = move |arg| {
            let captured_s = &s;
            println!("{}", captured_s);
            arg
        };

        // print(&s);
        closure(1);
        closure(2);
    }

    #[test]
    fn my_move_fn_closure() {
        struct MyMoveFnClosure {
            captured_s: String,
        }

        impl MyFnOnce<i32> for MyMoveFnClosure {
            type Output = i32;

            fn call_once(self, arg: i32) -> i32 {
                self.call(arg)
            }
        }

        impl MyFnMut<i32> for MyMoveFnClosure {
            fn call_mut(&mut self, arg: i32) -> i32 {
                self.call(arg)
            }
        }

        impl MyFn<i32> for MyMoveFnClosure {
            fn call(&self, arg: i32) -> i32 {
                print(&self.captured_s);
                arg
            }
        }

        let s = String::from("a");
        let closure = MyMoveFnClosure { captured_s: s };

        // print(&s);
        closure.call(1);
        closure.call(2);
    }

    fn print(s: &String) {
        println!("{}", s);
    }
}
