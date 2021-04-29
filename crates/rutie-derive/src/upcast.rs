pub trait ClassInfo {
    type Class;
    type RubyClass;
}

pub trait UpcastRubyClass<T> {
    fn upcast(&self) -> &T;

    fn upcast_mut(&mut self) -> &mut T;
}

macro_rules! id_impl {
    ($ty:ty) => {
        impl ClassInfo for $ty {
            type Class = Self;
            type RubyClass = Self;
        }

        impl UpcastRubyClass<$ty> for $ty {
            fn upcast(&self) -> &$ty {
                self
            }

            fn upcast_mut(&mut self) -> &mut $ty {
                self
            }
        }
    };

    ( $( $ty:ty ),+ $(,)* ) => {
        $( id_impl!( $ty ); )*
    }
}

id_impl!(
    rutie::AnyObject,
    rutie::RString,
    rutie::Array,
    rutie::Boolean,
    rutie::Fixnum,
    rutie::Float,
    rutie::Hash,
    rutie::Integer,
    rutie::NilClass,
    rutie::Symbol,
);
