## TRL - Type Reflection Lib

Adds some useful type reflection macros

## Supported macros

`#[getters(...)]` - Adds getter methods to a struct

`#[setters(...)]` - Adds setter methods to a struct

## Parameters

- No parameters - Generates getters/setters for any private field, pub field are ignored

    ```rust
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
    }

    impl User {
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
    }

    impl User {
        pub fn set_name(&mut self, value: String) {
            self.name = value;
        }
    }
    ```

- `excludes=[...]` - Generates getters/setters for all the fields excludes listed:
    ```rust
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
    }

    impl User {
        pub fn set_id(&mut self, value: u32) {
            self.id = value;
        }
    }
    ```
- `prefix=...` - Generates getters/setters with the specified prefix. By default, getter's prefix is empty, setter's prefix is `set_`:
    ```rust
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
    }

    impl User {
        pub fn example_set_id(&mut self, value: u32) {
            self.id = value;
        }

        pub fn example_set_name(&mut self, value: String) {
            self.name = value;
        }
    }
    ```
- `include_pub` - Generates getters/setter for `pub` fields too
    ```rust
    #[getters(include_pub)]
    #[setters(include_pub)]
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
    }

    impl User {
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
- `borrow` - Generates getters that borrow `self`. For setters, this parameter is ignored
    ```rust
    #[getters(borrow)]
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
- `mut_ref` - Generates getters that provide `self` as `&mut self` instead of `&self`. For setters, this parameter is ignored
    ```rust
    #[getters(mut_ref)]
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

## TODO
- Field-level getters/setters:
    ```rust
    #[getters]
    struct User {
        #[get]
        id: u32,

        #[get(prefix = get_)]
        name: String,

        #[get(name = get_pn)]
        phone_number: u64,
    }
    ```

- Visibility parameters:
    ```rust
    #[getters]
    struct User {
        #[get(visibility = crate)]
        id: u32,
        ...
    }

    impl User {
        pub(crate) fn id(&self) -> &u32 {
            &self.id
        }
    }
    ```