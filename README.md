# umbra

[![Crate Status](https://img.shields.io/crates/v/umbra.svg)](https://crates.io/crates/umbra)

A macro to generate optional structs

## About

Umbra provides a macro that generates the struct where each field in the struct is of type Option.

### Why?

For example, if you use serde to load a config file, you may find it a bit tedious to deal with optional settings. Umbra will generate the struct with optional fields and conversions back to the original struct, which may make writing such programs a little easier.

Here is an example of usage:

```rust
// The target struct is marked as `umbra::optional`.
// The generated struct is marked as `serde::Deserialize` as the derive.
#[umbra::optional(derives = [serde::Deserialize])]
#[derive(Debug)]
struct Config {
  log_level: String,
  cache: bool,
  timeout_seconds: usize,
}

impl Default for Config {
  fn default() -> Self {
    Config {
      log_level: "warn".into(),
      cache: true,
      timeout_seconds: 30,
    }
  }
}

fn load_config(config_str: &str) -> Config {
  // A struct definition with optional fields and `Into` the original struct are provided.
  let config: OptionalConfig = toml::from_str(config_str).unwrap();
  config.into()
}

fn main() {
  // Any fields you specify will be set to their values,
  // others will be set to the default values defined in the `Default` trait.
  let config_str = r#"
    timeout_seconds = 10
  "#;
  let config = load_config(config_str);

  println!("{:?}", config);
  // => Config { log_level: "warn", cache: true, timeout_seconds: 10 }
}

```

## Usage

### Basic

Add the `#[optional]` and `#[nested]` attributes as follows:

```rs
use umbra::optional;

#[optional]
#[derive(Default)]
struct Foo {
  id: u32,
  name: String,
  #[nested]
  bar: Bar,
}

#[optional]
#[derive(Default)]
struct Bar {
  name: String,
  value: Option<i32>,
}
```

The macro generates following structs:

```rs
#[derive(Default)]
struct Foo {
  id: u32,
  name: String,
  bar: Bar,
}

#[derive(Default)]
struct Bar {
  name: String,
  value: Option<i32>,
}

struct OptionalFoo {
  id: Option<u32>,
  name: Option<String>,
  bar: Option<OptionalBar>,
}

impl From<OptionalFoo> for Foo {
  fn from(optional: OptionalFoo) -> Self {
      let mut base = Self::default(); // create base values by default
      if let Some(value) = optional.id {
          base.id = value; // simple field
      }
      if let Some(value) = optional.bar {
          base.bar = value.into(); // nested field
      }
      // ...
      base
  }
}

struct OptionalBar {
  name: Option<String>,
  value: Option<i32>,
}

impl From<OptionalBar> for Bar {
  fn from(optional: OptionalBar) -> Self {
      let mut base = Self::default();
      // ...
      base
  }
}
```

### Derives

By using the `derives` attribute, the derive attribute can be added to the generated struct:

```rs
use umbra::optional;

#[optional(derives = [Debug, std::clone::Clone])]
#[derive(Default)]
struct Foo {
  id: u32,
  name: String,
}
```

The macro generates following structs:

```rs
#[derive(Default)]
struct Foo {
  id: u32,
  name: String,
}

#[derive(Debug, std::clone::Clone)] // The derive attribute is added
struct OptionalFoo {
  id: Option<u32>,
  name: Option<String>,
}

impl From<OptionalFoo> for Foo {
  fn from(optional: OptionalFoo) -> Self {
      let mut base = Self::default();
      // ...
      base
  }
}
```

### Prefix / Suffix

By using the `prefix` and `suffix` attributes, the prefix and suffix of the generated struct are changed:

```rs
use umbra::optional;

#[optional(prefix = "Pre", suffix = "Suf")]
#[derive(Default)]
struct Foo {
  id: u32,
  name: String,
}
```

The macro generates following structs:

```rs
#[derive(Default)]
struct Foo {
  id: u32,
  name: String,
}

struct PreFooSuf { // <Prefix><Base name><Suffix> format
  id: Option<u32>,
  name: Option<String>,
}

impl From<PreFooSuf> for Foo {
  fn from(optional: PreFooSuf) -> Self {
      let mut base = Self::default();
      // ...
      base
  }
}
```

### Visibility

By using the `visibility` attribute, the visibility can be added to the generated struct:

```rs
use umbra::optional;

#[optional(visibility = pub)]
#[derive(Default)]
struct Foo {
  id: u32,
  name: String,
}
```

The macro generates following structs:

```rs
#[derive(Default)]
struct Foo {
  id: u32,
  name: String,
}

pub struct OptionalFoo { // public
  id: Option<u32>,
  name: Option<String>,
}

impl From<OptionalFoo> for Foo {
  fn from(optional: OptionalFoo) -> Self {
      let mut base = Self::default();
      // ...
      base
  }
}
```

## License

MIT
