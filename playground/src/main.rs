enum RSEnum {
    Foo2(Option<i32>),
    Foo(i32),
    Bar(String),
    Baz(Vec<String>),
    Fo(fn() -> i32)
}

fn bar() -> i32 {
    return 5;
}




fn main() {
    let mut a = vec![];
    a.push(1);
    let mut b = a.clone();
    b.push(1);
    println!("{:?}", b);

    let x = 5;
    let y = &x;
    println!("{:?}", x+y);

    let mut x= 5;
    let z = &mut x;

    *z = 7;
 
    println!("{:?}", z);


    let fa = Some(5);

    if let Some(value) = fa {

    }

    match fa {
        Some(value) => {

        },
        None => {

        }
    }

    fa.map(|x| {

    });

fa.filter(|x| x < &10);

    let fo = RSEnum::Fo(bar)

    let foo = RSEnum::Foo(5);

    if let RSEnum::Foo(value) = foo {

    }

    match foo {
        RSEnum::Foo2(Some(value)) => { },
        RSEnum::Foo2(None) => { },
        RSEnum::Foo(value) => { },
        _ => {}
    }
}
