// big thanks to https://danielkeep.github.io/tlborm/book/blk-enum-parsing.html
// and to Quxxy from #rust
#[macro_export]
macro_rules! inherit {
    // phase 1: public struct parse
    (pub struct $($tail:tt)*) => {
        inherit!{ @pub struct $($tail)* }
    };
    // phase 1: private struct parse
    (struct $($tail:tt)*) => {
        inherit!{ @priv struct $($tail)* }
    };
    // phase 2: non-empty struct parse
    (@$v:ident struct $name:ident: $sup:ty { $($queue:tt)* } $($tail:tt)*) => {
        inherit!{@impl @struct [@$v, $name, $sup]
            @queue [ $($queue)* , ]
            $($tail)*
        }
    };
    // phase 2: empty struct parse
    (@$v:ident struct $name:ident: $sup:ty; $($tail:tt)*) => {
        inherit!{@impl @struct [@$v, $name, $sup]
            @queue []
            $($tail)*
        }
    };
    // phase 2: non-empty struct, default $sup
    (@$v:ident struct $name:ident { $($queue:tt)* } $($tail:tt)*) => {
        inherit!{@impl @struct [@$v, $name, $crate::base::Base]
            @queue [ $($queue)* , ]
            $($tail)*
        }
    };
    // phase 2: empty struct, default $sup
    (@$v:ident struct $name:ident; $($tail:tt)*) => {
        inherit!{@impl @struct [@$v, $name, $crate::base::Base]
            @queue []
            $($tail)*
        }
    };
    // phase 3: impl * for struct, recurse
    (@impl @struct [@$v:ident, $name:ident, $sup:ty]
        @queue $queue:tt
        $($tail:tt)*
    ) => {
        inherit!{@struct [@$v, $name, $sup]
            @queue $queue
            @pub []
            @priv []
        }

        impl $crate::base::Constructor for $name {
            type Super = $sup;

            fn init_base(&mut self, s: *const $crate::base::Castable) {
                self.__super__.init_base(s);
            }

            fn inherit(sup: Self::Super) -> Self {
                $name {
                    __super__: sup,
                    .. Self::init()
                }
            }
        }

        impl $crate::base::Castable for $name {
            fn ident() -> $crate::base::TypeIdent { std::any::TypeId::of::<$name>() }
            fn get_ident(&self) -> $crate::base::TypeIdent { Self::ident() }
            fn get_super(&self) -> &$crate::base::Castable { &self.__super__ }
        }

        impl $crate::base::CastableHelper for $name {}

        inherit!{ $($tail)* }
    };
    // phase 4: public fields
    (@struct $m:tt
        @queue [ pub  $a:ident : $b:ty, $($tail:tt)* ]
        @pub [ $($public:tt)* ]
        @priv $private:tt
    ) => {
        inherit!{@struct $m
            @queue [ $($tail)* ]
            @pub [ $($public)* pub $a : $b, ]
            @priv $private
        }
    };
    // phase 4: private fields
    (@struct $m:tt
        @queue [ $a:ident : $b:ty, $($tail:tt)* ]
        @pub $public:tt
        @priv [ $($private:tt)* ]
    ) => {
        inherit!{@struct $m
            @queue [ $($tail)* ]
            @pub $public
            @priv [ $($private)* $a : $b, ]
        }
    };
    // phase 5: public struct generation
    (@struct [@pub, $name:ident, $sup:ty]
        @queue [ $(,)* ]
        @pub  [ $(pub  $a:ident : $b:ty,)* ]
        @priv [ $( $c:ident : $d:ty,)* ]
    ) => {
        pub struct $name {
            __super__: $sup,
            $(pub  $a : $b,)*
            $(     $c : $d,)*
        }
    };
    // phase 5: private struct generation
    (@struct [@priv, $name:ident, $sup:ty]
        @queue [ $(,)* ]
        @pub  [ $(pub  $a:ident : $b:ty,)* ]
        @priv [ $( $c:ident : $d:ty,)* ]
    ) => {
        struct $name {
            __super__: $sup,
            $(pub  $a : $b,)*
            $(     $c : $d,)*
        }
    };
    // base case of recursion
    () => {};
}
