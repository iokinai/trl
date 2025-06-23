pub use trl_codegen::*;

pub mod prelude {
    //! Module `prelude` reexports all commonly used attributes.
    //!
    //! It's recommended to always import it to simplify usage:
    //! ```rust
    //! use trl::prelude::*;
    //! ```
    //!
    //! ⚠️ **Important:** All attributes must be *directly imported* and
    //! used by their plain names. For example, use:
    //! ```rust
    //! #[constructor]
    //! ```
    //! and **not**
    //! ```rust
    //! #[trl::constructor] // ← This will NOT work
    //! ```
    //!
    //! This limitation exists due to the current implementation.
    pub use crate::constructor;
    pub use crate::getters;
    pub use crate::setters;
    pub use crate::trl;
}

#[cfg(test)]
mod tests {
    use trl_codegen::{constructor, getters, setters, trl};

    #[derive(Default, trl)]
    #[getters]
    struct OnlyGettersUser {
        id: u64,
        name: String,
        email: String,
        pub phone_number: u64,
    }

    #[test]
    fn getters_test() {
        let user = OnlyGettersUser::default();

        assert!(*user.id() == 0);
        assert!(*user.name() == String::from(""));
        assert!(*user.email() == String::from(""));
    }

    #[derive(Default, trl)]
    #[getters(pub)]
    struct GettersIncludePubUser {
        id: u64,
        name: String,
        email: String,
        pub phone_number: u64,
    }

    #[test]
    fn include_pub_test() {
        let user = GettersIncludePubUser::default();

        assert!(*user.phone_number() == 0)
    }

    #[derive(Default, trl)]
    struct GetSetFieldUser {
        #[set]
        id: u64,
        #[get(name = get_name)]
        name: String,
        #[get]
        email: String,
        pub phone_number: u64,
    }

    #[test]
    fn get_one_test() {
        let mut user = GetSetFieldUser::default();

        assert!(user.id == 0);
        user.set_id(2);
        assert!(*user.get_name() == String::from(""));
        assert!(*user.email() == String::from(""));
        assert!(user.id == 2);
    }

    #[derive(Default, trl)]
    #[setters]
    struct SettersUser {
        id: u64,
        name: String,
        email: String,
        pub phone_number: u64,
    }

    #[test]
    fn setters_test() {
        let mut user = SettersUser::default();

        assert!(user.name == "");
        user.set_name(String::from("John"));
        assert!(user.name == "John");
    }

    #[derive(Default, trl)]
    #[constructor]
    struct ConstructorUser {
        id: u64,
        name: String,
        email: String,
        pub phone_number: u64,
    }

    #[test]
    fn constructor_test() {
        let user = ConstructorUser::new(0, String::new(), String::new(), 3);
    }
}
