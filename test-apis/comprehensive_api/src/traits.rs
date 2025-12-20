pub trait Empty {}

pub trait Simple {
    fn act();
}

pub trait AssociatedConst {
    const CONST: bool;
}

pub trait AssociatedConstDefault {
    const CONST_WITH_DEFAULT: bool = true;
}

pub trait AssociatedType {
    type Type;
}

pub trait TraitReferencingOwnAssociatedType {
    type OwnAssociatedType;

    fn own_associated_type_output(&self) -> Self::OwnAssociatedType;
    fn own_associated_type_output_explicit_as(
        &self,
    ) -> <Self as TraitReferencingOwnAssociatedType>::OwnAssociatedType;
}

pub trait TraitWithGenerics<T, U> {
    type Foo;

    fn bar() -> <Self as TraitWithGenerics<T, U>>::Foo;
}

// error[E0658]: associated type defaults are unstable
// skip for now
// pub trait AssociatedTypeDefault {
//     type Type = usize;
// }

pub unsafe trait UnsafeTrait {}

pub trait TraitWithBounds: private_mod::PubTraitInPrivateMod + Simple + Send {}

pub trait TraitWithBoundsAndGenerics<U>: Simple {}

mod private_mod {
    pub trait PubTraitInPrivateMod {}
}

pub trait GenericAssociatedTypes {
    type WhereSelfSized
    where
        Self: Sized;
    type WhereSimple<T>
    where
        T: Simple;
    type SimpleBound: Simple;
    type WithLifetime<'a>;
}

/// Trait for testing underscore prefix normalization in trait impl parameters (issue #766)
pub trait TraitForUnderscoreNormalization {
    fn method_with_params(a: i32, b: String);
    fn method_with_self(&self, param: i32);
    fn method_with_underscore_pattern(_: i32);
    fn method_with_double_underscore(__param: i32);
}
