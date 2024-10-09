#![allow(dead_code)]

#[umbra::optional]
#[derive(Debug, PartialEq, Eq)]
pub struct Foo {
    pub id: i32,
    pub name: String,
    pub is_active: bool,
    #[nested]
    bar: Bar,
}

impl Default for Foo {
    fn default() -> Self {
        Self {
            id: 1000,
            name: "foo".into(),
            is_active: true,
            bar: Bar::default(),
        }
    }
}

#[umbra::optional(derives = ["Debug", "Default"])]
#[derive(Debug, PartialEq, Eq)]
struct Bar {
    name: String,
    ty: Type,
}

impl Default for Bar {
    fn default() -> Self {
        Self {
            name: "bar".into(),
            ty: Type::default(),
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
enum Type {
    A,
    #[default]
    B,
    C,
}

#[test]
fn test_into_1() {
    let optional = OptionalFoo {
        id: Some(2000),
        name: Some("FOO".into()),
        is_active: Some(false),
        bar: Some(OptionalBar {
            name: Some("BAR".into()),
            ty: Some(Type::C),
        }),
    };
    let expected = Foo {
        id: 2000,
        name: "FOO".into(),
        is_active: false,
        bar: Bar {
            name: "BAR".into(),
            ty: Type::C,
        },
    };

    let actual: Foo = optional.into();

    assert_eq!(actual, expected);
}

#[test]
fn test_into_2() {
    let optional = OptionalFoo {
        id: Some(2000),
        name: None,
        is_active: Some(false),
        bar: Some(OptionalBar {
            name: None,
            ty: Some(Type::C),
        }),
    };
    let expected = Foo {
        id: 2000,
        name: "foo".into(),
        is_active: false,
        bar: Bar {
            name: "bar".into(),
            ty: Type::C,
        },
    };

    let actual: Foo = optional.into();

    assert_eq!(actual, expected);
}

#[test]
fn test_into_3() {
    let optional = OptionalFoo {
        id: None,
        name: None,
        is_active: None,
        bar: None,
    };
    let expected = Foo::default();

    let actual: Foo = optional.into();

    assert_eq!(actual, expected);
}
