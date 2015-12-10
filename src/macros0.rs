
#[macro_export]
macro_rules! inherit {
    ( pub struct $name:ident: $sup:ty { $(pub $fname:ident : $ftype:ty),* } $($o:tt)* ) => {

        pub struct $name {
            pub __super__: $sup,
            $(pub $fname : $ftype),*
        }

        impl $crate::base::Castable for $name {
            fn ident() -> $crate::base::TypeIdent { std::any::TypeId::of::<$name>() }
            fn get_ident(&self) -> $crate::base::TypeIdent { Self::ident() }
            fn get_super(&self) -> &$crate::base::Castable { &self.__super__ }
        }

        inherit!{ $($o)* }
    };
    ( pub struct $name:ident { $(pub $fname:ident : $ftype:ty)* } $($o:tt)* ) => {
        inherit!( pub struct $name: $crate::base::Base { $(pub $fname : $ftype)* } $($o)* );
    };
    () => {};
}
