For some use cases, when matching enums, match is awkward. For example:



// Make `optional` of type `Option<i32>`
let optional = Some(7);

match optional {
    Some(i) => {
        println!("This is a really long string and `{:?}`", i);
        // ^ Needed 2 indentations just so we could destructure
        // `i` from the option.
    },
    _ => {},
    // ^ Required because `match` is exhaustive. Doesn't it seem
    // like wasted space?
};

if let is cleaner for this use case and in addition allows various failure options to be specified:


fn main() {
    // All have type `Option<i32>`


In the same way, if let can be used to match any enum value:


    }
}


Another benefit is that if let allows us to match non-parameterized enum variants. This is true even in cases where the enum doesn't implement or derive PartialEq. In such cases if Foo::Bar == a would fail to compile, because instances of the enum cannot be equated, however if let will continue to work.

Would you like a challenge? Fix the following example to use if let:


```
// This enum purposely neither implements nor derives PartialEq.
// That is why comparing Foo::Bar == a fails below.
enum Foo {Bar}

fn main() {
    let a = Foo::Bar;

    // Variable a matches Foo::Bar
    if Foo::Bar == a {
    // ^-- this causes a compile-time error. Use `if let` instead.
        println!("a is foobar");
    }
}
```

