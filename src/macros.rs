#[macro_export]
macro_rules! impl_inherit {
    ($name:ident from $supf:ident : $sup:ty;) => {
        impl $crate::UnsafeCastable for $name {
            fn init_base(&mut self, s: Option<*mut $crate::UnsafeCastable>) {
                self.$supf.init_base(s);
            }
            fn ident() -> ::std::any::TypeId { ::std::any::TypeId::of::<$name>() }
            fn get_ident(&self) -> ::std::any::TypeId { Self::ident() }
            fn get_super(&self) -> &$crate::UnsafeCastable { &self.$supf }
        }

        impl $crate::Constructable for $name {
            type Super = $sup;

            unsafe fn inherit(sup: Self::Super) -> Self {
                $name {
                    $supf: sup,
                    .. Self::null()
                }
            }
        }

        impl $crate::Castable for $name {}

        impl ::std::ops::Deref for $name {
            type Target = $sup;
            fn deref(&self) -> &$sup {
                $crate::Castable::upcast::<$sup>(self).expect("unable to downcast for Deref")
            }
        }

        impl ::std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut $sup {
                $crate::Castable::upcast_mut::<$sup>(self).expect("unable to downcast for DerefMut")
            }
        }
    };
}

// big thanks to https://danielkeep.github.io/tlborm/book/blk-enum-parsing.html
// and to Quxxy from #rust
#[macro_export]
macro_rules! inherit {
    // phase 1: public struct parse
    (pub struct $($tail:tt)*) => {
        inherit!{ meta [] @pub struct $($tail)* }
    };
    // phase 1: private struct parse
    (struct $($tail:tt)*) => {
        inherit!{ meta [] @priv struct $($tail)* }
    };
    (#[$nm:meta] $($tail:tt)*) => {
        inherit!{ meta [$nm,] $($tail)* }
    };
    // phase 0: handle attributes
    (meta [ $($meta:tt)* ] #[$nm:meta] $($tail:tt)*) => {
        inherit!{ meta [$($meta)* $nm,] $($tail)* }
    };
    // phase 1: public struct parse
    (meta $meta:tt pub struct $($tail:tt)*) => {
        inherit!{ meta $meta @pub struct $($tail)* }
    };
    // phase 1: private struct parse
    (meta $meta:tt struct $($tail:tt)*) => {
        inherit!{ meta $meta @priv struct $($tail)* }
    };
    // phase 2: non-empty struct parse
    (meta $meta:tt @$v:ident struct $name:ident: $sup:ty { $($queue:tt)* } $($tail:tt)*) => {
        inherit!{meta $meta @impl @struct [@$v, $name, $sup]
            @queue [ $($queue)* , ]
            $($tail)*
        }
    };
    // phase 2: empty struct parse
    (meta $meta:tt @$v:ident struct $name:ident: $sup:ty; $($tail:tt)*) => {
        inherit!{meta $meta @impl @struct [@$v, $name, $sup]
            @queue []
            $($tail)*
        }
    };
    // phase 2: non-empty struct, default $sup
    (meta $meta:tt @$v:ident struct $name:ident { $($queue:tt)* } $($tail:tt)*) => {
        inherit!{meta $meta @impl @struct [@$v, $name, $crate::Base]
            @queue [ $($queue)* , ]
            $($tail)*
        }
    };
    // phase 2: empty struct, default $sup
    (meta $meta:tt @$v:ident struct $name:ident; $($tail:tt)*) => {
        inherit!{meta $meta @impl @struct [@$v, $name, $crate::Base]
            @queue []
            $($tail)*
        }
    };
    // phase 3: impl traits for struct
    (meta $meta:tt @impl @struct [@$v:ident, $name:ident, $sup:ty]
        @queue $queue:tt
        $($tail:tt)*
    ) => {
        inherit!{meta $meta @struct [@$v, $name, $sup]
            @queue $queue
            @pub []
            @priv []
        }

        impl_inherit!{$name from __super__: $sup;}

        inherit!{ $($tail)* }
    };
    // phase 4: public fields
    (meta $meta:tt @struct $m:tt
        @queue [ pub  $a:ident : $b:ty, $($tail:tt)* ]
        @pub [ $($public:tt)* ]
        @priv $private:tt
    ) => {
        inherit!{meta $meta @struct $m
            @queue [ $($tail)* ]
            @pub [ $($public)* pub $a : $b, ]
            @priv $private
        }
    };
    // phase 4: private fields
    (meta $meta:tt @struct $m:tt
        @queue [ $a:ident : $b:ty, $($tail:tt)* ]
        @pub $public:tt
        @priv [ $($private:tt)* ]
    ) => {
        inherit!{meta $meta @struct $m
            @queue [ $($tail)* ]
            @pub $public
            @priv [ $($private)* $a : $b, ]
        }
    };
    // phase 5: public struct generation
    (meta [$($meta:meta),* $(,)*] @struct [@pub, $name:ident, $sup:ty]
        @queue [ $(,)* ]
        @pub  [ $(pub  $a:ident : $b:ty,)* ]
        @priv [ $( $c:ident : $d:ty,)* ]
    ) => {
        $(#[$meta])*
        pub struct $name {
            pub __super__: $sup,
            $(pub  $a : $b,)*
            $(     $c : $d,)*
        }
    };
    // phase 5: private struct generation
    (meta [$($meta:meta),* $(,)*] @struct [@priv, $name:ident, $sup:ty]
        @queue [ $(,)* ]
        @pub  [ $(pub  $a:ident : $b:ty,)* ]
        @priv [ $( $c:ident : $d:ty,)* ]
    ) => {
        $(#[$meta])*
        struct $name {
            __super__: $sup,
            $(pub  $a : $b,)*
            $(     $c : $d,)*
        }
    };
    // base case of recursion
    () => {};
}

#[macro_export]
macro_rules! construct {
    // phase 1: struct expr recognition
    (raw $t:ident { $($tail:tt)* }) => {
        construct!( parse [] $t { $($tail)* } )
    };
    // phase 1: struct expr recognition and init call
    ($t:ident { $($tail:tt)* }) => {
        <$t as $crate::Constructable>::init::<$t>(construct!( parse [] $t { $($tail)* } ))
    };
    // phase 1: struct expr recognition and init call
    ($t:ident as $s:ident { $($tail:tt)* }) => {
        <$t as $crate::Constructable>::init::<$s>(construct!( parse [] $t { $($tail)* } ))
    };
    // phase 2: parse normal `field: value` part
    (parse [ $($f:tt)* ] $t:ident { $a:ident: $b:expr, $($tail:tt)* }) => {
        construct!( parse [ $($f)* $a : $b, ] $t { $($tail)* })
    };
    // phase 2: parse final `field: value` part
    (parse [ $($f:tt)* ] $t:ident { $a:ident: $b:expr }) => {
        construct!( parse [ $($f)* $a : $b, ] $t {  })
    };
    // phase 2: prepare super struct, prepare for expression output
    (parse [ $($f:tt)* ] $t:ident { sup.. $($tail:tt)* }) => {
        construct!( expr [ $($f)* __super__ : construct!( raw $($tail)* ), ] $t )
    };
    // phase 2: prepare super struct, prepare for expression output
    (parse [ $($f:tt)* ] $t:ident { $(,)* }) => {
        construct!( expr [ $($f)* __super__ : <<$t as $crate::Constructable>::Super as $crate::Constructable>::default(), ] $t )
    };
    // phase 3: output modified strut expression
    (expr [ $($a:ident : $b:expr,)* ] $t:ident ) => {
        $t {
            $($a : $b,)*
        }
    };
}
