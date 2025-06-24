## TRL - Type Reflection Lib

Adds some useful type reflection macros

## Supported macros

`#[derive(trl)]` - The main macro that is required for any other

### Struct-level macros

`#[getters(...)]` - Adds getter methods to a struct

`#[setters(...)]` - Adds setter methods to a struct

`#[constructor(...)]` - Adds default constructor

### Field-level macros

`#[get(...)]` - Adds getter method to a struct field

`#[set(...)]` - Adds setter method to a struct field

## Parameters
#### Struct-level

- No parameters - Generates getters/setters for any private field, pub fields are ignored

    ```rust
    #[derive(trl)]
    #[getters]
    #[setters]
    struct User {
        id: u32,
        name: String,
        // pub fields are ignored by default
        pub phone_number: u64,
    }
    ```

    Would generate:

    ```rust
    impl User {
        pub fn id(&self) -> &u32 {
            &self.id
        }

        pub fn name(&self) -> &String {
            &self.name
        }

        pub fn set_id(&mut self, value: u32) {
            self.id = value;
        }

        pub fn set_name(&mut self, value: String) {
            self.name = value;
        }
    }
    ```

- `includes=[...]` - Generates getters/setters only for the listed fields:

    ```rust
    #[derive(trl)]
    #[getters(includes = [id])]
    #[setters(includes = [name])]
    struct User {
        id: u32,
        name: String,
        // pub fields are ignored by default
        pub phone_number: u64,
    }
    ```

    Would generate:

    ```rust
    impl User {
        pub fn id(&self) -> &u32 {
            &self.id
        }

        pub fn set_name(&mut self, value: String) {
            self.name = value;
        }
    }
    ```

- `excludes=[...]` - Generates getters/setters for all the fields, excluding the listed:
    ```rust
    #[derive(trl)]
    #[getters(excludes = [id])]
    #[setters(excludes = [name])]
    struct User {
        id: u32,
        name: String,
        // pub fields are ignored by default
        pub phone_number: u64,
    }
    ```

    Would generate:

    ```rust
    impl User {
        pub fn name(&self) -> &String {
            &self.name
        }

        pub fn set_id(&mut self, value: u32) {
            self.id = value;
        }
    }
    ```
- `prefix=...` - Generates getters/setters with the specified prefix. By default, the prefix for getters is empty, while setters have the prefix `set_`:
    ```rust
    #[derive(trl)]
    #[getters(prefix=get_)]
    #[setters(prefix=example_set_)]
    struct User {
        id: u32,
        name: String,
        // pub fields are ignored by default
        pub phone_number: u64,
    }
    ```

    Would generate:

    ```rust
    impl User {
        pub fn get_id(&self) -> &u32 {
            &self.id
        }

        pub fn get_name(&self) -> &String {
            &self.name
        }

        pub fn example_set_id(&mut self, value: u32) {
            self.id = value;
        }

        pub fn example_set_name(&mut self, value: String) {
            self.name = value;
        }
    }
    ```
- `pub` - Generates getters/setter for `pub` fields too
    ```rust
    #[derive(trl)]
    #[getters(pub)]
    #[setters(pub)]
    struct User {
        id: u32,
        name: String,
        pub phone_number: u64,
    }
    ```

    Would generate:

    ```rust
    impl User {
        pub fn id(&self) -> &u32 {
            &self.id
        }

        pub fn name(&self) -> &String {
            &self.name
        }

        pub fn phone_number(&self) -> &String {
            &self.phone_number
        }

        pub fn set_id(&mut self, value: u32) {
            self.id = value;
        }

        pub fn set_name(&mut self, value: String) {
            self.name = value;
        }

        pub fn set_phone_number(&mut self, value: u64) {
            self.phone_number = value;
        }
    }
    ```
- `move` - Generates getters that moves `self`. For setters, this parameter is ignored
    ```rust
    #[derive(trl)]
    #[getters(move)]
    struct User {
        id: u32,
        name: String,
        // pub fields are ignored by default
        pub phone_number: u64,
    }
    ```

    Would generate:

    ```rust
    impl User {
        pub fn get_id(self) -> u32 {
            self.id
        }

        pub fn get_name(self) -> String {
            self.name
        }
    }
    ```
- `mut ref` - Generates getters that provide `self` as `&mut self` instead of `&self`. For setters, this parameter is ignored
    ```rust
    #[derive(trl)]
    #[getters(mut ref)]
    struct User {
        id: u32,
        name: String,
        // pub fields are ignored by default
        pub phone_number: u64,
    }
    ```

    Would generate:

    ```rust
    impl User {
        pub fn get_id(&mut self) -> &mut u32 {
            &mut self.id
        }

        pub fn get_name(&mut self) -> &mut String {
            &mut self.name
        }
    }
    ```

#### Field-level
- `No parameters` - Generates getter/setter for a field:
    ```rust
    #[derive(trl)]
    struct User {
        #[get]
        #[set]
        id: u32,

        #[set]
        name: String,

        pub phone_number: u64,
    }
    ```

    Would generate:

    ```rust
    impl User {
        pub fn id(&self) -> &u32 {
            &self.id
        }

        pub fn set_id(&mut self, value: u32) {
            self.id = value;
        }

        pub fn set_name(&mut self, value: String) {
            self.name = value;
        }
    }
    ```

- `prefix` - Adds a specified prefix to a getter or setter. By default, the prefix for getters is empty, while setters have the prefix `set_`:
    ```rust
    #[derive(trl)]
    struct User {
        #[get(prefix = get_)]
        #[set(prefix = set_ex_)]
        id: u32,

        name: String,

        pub phone_number: u64,
    }
    ```

    Would generate:

    ```rust
    impl User {
        pub fn get_id(&self) -> &u32 {
            &self.id
        }

        pub fn set_ex_id(&mut self, value: u32) {
            self.id = value;
        }
    }
    ```

- `name` - Sets the getter/setter name:
    ```rust
    #[derive(trl)]
    struct User {
        #[get(name = get_identifier)]
        #[set(name = set_identifier)]
        id: u32,

        name: String,

        pub phone_number: u64,
    }
    ```

    Would generate:

    ```rust
    impl User {
        pub fn get_identifier(&self) -> &u32 {
            &self.id
        }

        pub fn set_identifier(&mut self, value: u32) {
            self.id = value;
        }
    }
    ```

    Note: if you specify both `name` and `prefix` the prefix will be added to the specified name

- `move` - Generates getter that moves `self`. For setters, this parameter is ignored:
    ```rust
    #[derive(trl)]
    struct User {
        id: u32,

        #[get(move)]
        name: String,

        pub phone_number: u64,
    }
    ```

    Would generate:

    ```rust
    impl User {
        pub fn name(self) -> u32 {
            self.id
        }
    }
    ```

- `mut ref` - Generates getter that provide `self` as `&mut self` instead of `&self`. For setters, this parameter is ignored:
    ```rust
    #[derive(trl)]
    struct User {
        id: u32,

        #[get(mut ref)]
        name: String,

        pub phone_number: u64,
    }
    ```

    Would generate:

    ```rust
    impl User {
        pub fn name(&mut self) -> &mut u32 {
            &mut self.id
        }
    }
    ```
#### Constructor parameters
- `name` - specify a custom name for the constructor
- `visibility` - specify a custom visibility modifier for the constructor

Possible visibilities (must be specified as string literals): 
- `"pub"` - public visibility
- `"pub(path)"` - restricted public visibility (e.g. "pub(crate)", "pub(super)", "pub(in some::module)")
- `"private"` - private visibility (not actually a Rust keyword, but used here for convenience)

    For example:
    ```rust
    #[derive(trl)]
    #[constructor(name = new_user, visibility="pub(crate)")]
    struct User {
        id: u32,
        name: String,
    }
    ```
    Would generate:
    ```rust
    impl User {
        pub(crate) fn new_user(id: u32, name: String) -> Self {
            Self { id, name }
        }
    }
```

## TODO
- Visibility parameters:
    ```rust
    #[getters]
    struct User {
        #[get(visibility = crate)]
        id: u32,
        ...
    }

    impl User {
        pub fn id(&self) -> &u32 {
            &self.id
        }
    }
    ```
