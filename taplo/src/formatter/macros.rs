macro_rules! create_options {
    (
        $(#[$attr:meta])*
        pub struct Options {
            $(
                $(#[$field_attr:meta])*
                pub $name:ident: $ty:ty,
            )+
        }
    ) => {
        $(#[$attr])*
        pub struct Options {
            $(
                $(#[$field_attr])*
                pub $name: $ty,
            )+
        }

        impl Options {
            #[doc(hidden)]
            pub fn update(&mut self, incomplete: OptionsIncomplete) {
                $(
                    if let Some(v) = incomplete.$name {
                        self.$name = v;
                    }
                )+
            }

            #[doc(hidden)]
            pub fn update_camel(&mut self, incomplete: OptionsIncompleteCamel) {
                $(
                    if let Some(v) = incomplete.$name {
                        self.$name = v;
                    }
                )+
            }
        }

        $(#[$attr])*
        #[doc(hidden)]
        #[derive(Default)]
        pub struct OptionsIncomplete {
            $(
                $(#[$field_attr])*
                pub $name: Option<$ty>,
            )+
        }

        $(#[$attr])*
        #[doc(hidden)]
        #[derive(Default)]
        #[serde(rename_all = "camelCase")]
        pub struct OptionsIncompleteCamel {
            $(
                $(#[$field_attr])*
                pub $name: Option<$ty>,
            )+
        }
    };
}
