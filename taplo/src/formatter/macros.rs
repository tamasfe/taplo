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

        impl OptionsIncomplete {
            #[doc(hidden)]
            pub fn from_options(opts: Options) -> Self {
                let mut o = Self::default();

                $(
                    o.$name = Some(opts.$name);
                )+

                o
            }
        }

        $(#[$attr])*
        #[doc(hidden)]
        #[derive(Default)]
        #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
        pub struct OptionsIncompleteCamel {
            $(
                $(#[$field_attr])*
                pub $name: Option<$ty>,
            )+
        }

        impl OptionsIncompleteCamel {
            #[doc(hidden)]
            pub fn from_options(opts: Options) -> Self {
                let mut o = Self::default();

                $(
                    o.$name = Some(opts.$name);
                )+

                o
            }
        }
    };
}
