use crate::intermediate_public_item::IntermediatePublicItem;
use std::rc::Rc;

use rustdoc_types::{
    Abi, Constant, FnDecl, GenericArg, GenericArgs, GenericBound, GenericParamDef,
    GenericParamDefKind, Generics, Header, ItemEnum, MacroKind, Term, Type, TypeBinding,
    TypeBindingKind, Variant, WherePredicate,
};

/// A simple macro to write `Token::Whitespace` in less characters.
macro_rules! ws {
    () => {
        Token::Whitespace
    };
}

use crate::tokens::Token;

#[allow(clippy::too_many_lines)]
pub fn token_stream(item: &IntermediatePublicItem) -> Vec<Token> {
    let mut tokens = vec![];

    for attr in &item.item.attrs {
        if attr_relevant_for_public_apis(attr) {
            tokens.push(Token::Annotation(attr.clone()));
            tokens.push(ws!());
        }
    }

    let inner_tokens = match &item.item.inner {
        ItemEnum::Module(_) => render_simple(&["mod"], &item.path()),
        ItemEnum::ExternCrate { .. } => render_simple(&["extern", "crate"], &item.path()),
        ItemEnum::Import(_) => render_simple(&["use"], &item.path()),
        ItemEnum::Union(_) => render_simple(&["union"], &item.path()),
        ItemEnum::Struct(s) => {
            let mut output = render_simple(&["struct"], &item.path());
            output.extend(render_generics(&s.generics));
            output
        }
        ItemEnum::StructField(inner) => {
            let mut output = render_simple(&["struct", "field"], &item.path());
            output.extend(colon());
            output.extend(render_type(inner));
            output
        }
        ItemEnum::Enum(e) => {
            let mut output = render_simple(&["enum"], &item.path());
            output.extend(render_generics(&e.generics));
            output
        }
        ItemEnum::Variant(inner) => {
            let mut output = render_simple(&["enum", "variant"], &item.path());
            match inner {
                Variant::Struct(_) | // Each struct field is printed individually
                Variant::Plain => {}
                Variant::Tuple(types) => output.extend(render_tuple( types)),

            }
            output
        }
        ItemEnum::Function(inner) => render_function(
            render_path(&item.path()),
            &inner.decl,
            &inner.generics,
            &inner.header,
        ),
        ItemEnum::Method(inner) => render_function(
            render_path(&item.path()),
            &inner.decl,
            &inner.generics,
            &inner.header,
        ),
        ItemEnum::Trait(inner) => {
            let tags = if inner.is_unsafe {
                vec!["unsafe", "trait"]
            } else {
                vec!["trait"]
            };
            let mut output = render_simple(&tags, &item.path());
            output.extend(render_generics(&inner.generics));
            output
        }
        ItemEnum::TraitAlias(_) => render_simple(&["trait", "alias"], &item.path()),
        ItemEnum::Impl(_) => render_simple(&["impl"], &item.path()),
        ItemEnum::Typedef(inner) => {
            let mut output = render_simple(&["type"], &item.path());
            output.extend(render_generics(&inner.generics));
            output.extend(equals());
            output.extend(render_type(&inner.type_));
            output
        }
        ItemEnum::AssocType {
            generics,
            bounds,
            default,
        } => {
            let mut output = render_simple(&["type"], &item.path());
            output.extend(render_generics(generics));
            output.extend(render_generic_bounds(bounds));
            if let Some(ty) = default {
                output.extend(equals());
                output.extend(render_type(ty));
            }
            output
        }
        ItemEnum::OpaqueTy(_) => render_simple(&["opaque", "type"], &item.path()),
        ItemEnum::Constant(con) => {
            let mut output = render_simple(&["const"], &item.path());
            output.extend(colon());
            output.extend(render_constant(con));
            output
        }
        ItemEnum::AssocConst { type_, .. } => {
            let mut output = render_simple(&["const"], &item.path());
            output.extend(colon());
            output.extend(render_type(type_));
            output
        }
        ItemEnum::Static(inner) => {
            let tags = if inner.mutable {
                vec!["mut", "static"]
            } else {
                vec!["static"]
            };
            let mut output = render_simple(&tags, &item.path());
            output.extend(colon());
            output.extend(render_type(&inner.type_));
            output
        }
        ItemEnum::ForeignType => render_simple(&["type"], &item.path()),
        ItemEnum::Macro(_definition) => {
            // TODO: _definition contains the whole definition, it would be really neat to get out all possible ways to invoke it
            let mut output = render_simple(&["macro"], &item.path());
            output.push(Token::symbol("!"));
            output
        }
        ItemEnum::ProcMacro(inner) => {
            let mut output = render_simple(&["proc", "macro"], &item.path());
            output.pop(); // Remove name of macro to possibly wrap it in `#[]`
            let name = Token::identifier(item.item.name.as_ref().unwrap_or(&"".to_string()));
            match inner.kind {
                MacroKind::Bang => output.extend(vec![name, Token::symbol("!()")]),
                MacroKind::Attr => {
                    output.extend(vec![Token::symbol("#["), name, Token::symbol("]")]);
                }
                MacroKind::Derive => {
                    output.extend(vec![Token::symbol("#[derive("), name, Token::symbol(")]")]);
                }
            }
            output
        }
        ItemEnum::PrimitiveType(_) => render_simple(&["primitive", "type"], &item.path()),
    };

    tokens.extend(inner_tokens);

    tokens
}

/// Our list of allowed attributes comes from
/// <https://github.com/rust-lang/rust/blob/68d0b29098/src/librustdoc/html/render/mod.rs#L941-L942>
fn attr_relevant_for_public_apis<S: AsRef<str>>(attr: S) -> bool {
    let prefixes = [
        "#[export_name",
        "#[link_section",
        "#[no_mangle",
        "#[non_exhaustive",
        "#[repr",
    ];

    for prefix in prefixes {
        if attr.as_ref().starts_with(prefix) {
            return true;
        }
    }

    false
}

fn render_simple(tags: &[&str], path: &[Rc<IntermediatePublicItem<'_>>]) -> Vec<Token> {
    let mut output = vec![Token::qualifier("pub"), ws!()];
    output.extend(
        tags.iter()
            .flat_map(|t| [Token::kind(*t), ws!()])
            .collect::<Vec<Token>>(),
    );
    output.extend(render_path(path));
    output
}

fn render_path(path: &[Rc<IntermediatePublicItem<'_>>]) -> Vec<Token> {
    let mut output = vec![];
    for item in path {
        let token_fn = if matches!(item.item.inner, ItemEnum::Function(_) | ItemEnum::Method(_)) {
            Token::function
        } else if matches!(
            item.item.inner,
            ItemEnum::Trait(_)
                | ItemEnum::Struct(_)
                | ItemEnum::Union(_)
                | ItemEnum::Enum(_)
                | ItemEnum::Typedef(_)
        ) {
            Token::type_
        } else {
            Token::identifier
        };
        output.push(token_fn(item.name.to_owned()));
        output.push(Token::symbol("::"));
    }
    if !path.is_empty() {
        output.pop();
    }
    output
}

#[allow(clippy::needless_pass_by_value)]
fn render_sequence<T>(
    start: Vec<Token>,
    end: Vec<Token>,
    between: Vec<Token>,
    sequence: &[T],
    render: impl Fn(&T) -> Vec<Token>,
) -> Vec<Token> {
    render_sequence_impl(start, end, between, false, sequence, render)
}

#[allow(clippy::needless_pass_by_value)]
fn render_sequence_if_not_empty<T>(
    start: Vec<Token>,
    end: Vec<Token>,
    between: Vec<Token>,
    sequence: &[T],
    render: impl Fn(&T) -> Vec<Token>,
) -> Vec<Token> {
    render_sequence_impl(start, end, between, true, sequence, render)
}

#[allow(clippy::needless_pass_by_value)]
fn render_sequence_impl<T>(
    start: Vec<Token>,
    end: Vec<Token>,
    between: Vec<Token>,
    return_nothing_if_empty: bool,
    sequence: &[T],
    render: impl Fn(&T) -> Vec<Token>,
) -> Vec<Token> {
    if return_nothing_if_empty && sequence.is_empty() {
        return vec![];
    }
    let mut output = start;
    for (index, seq) in sequence.iter().enumerate() {
        output.extend(render(seq));
        if index < sequence.len() - 1 {
            output.extend(between.clone());
        }
    }
    output.extend(end);
    output
}

#[allow(clippy::too_many_lines)]
fn render_type(ty: &Type) -> Vec<Token> {
    match ty {
        Type::ResolvedPath {
            name,
            args,
            param_names,
            ..
        } => {
            let mut output = vec![];
            if !name.is_empty() {
                let split: Vec<_> = name.split("::").collect();
                let len = split.len();
                for (index, part) in split.into_iter().enumerate() {
                    if index == 0 && part == "$crate" {
                        output.push(Token::identifier("$crate"));
                    } else if index == len - 1 {
                        output.push(Token::type_(part));
                    } else {
                        output.push(Token::identifier(part));
                    }
                    output.push(Token::symbol("::"));
                }
                if len > 0 {
                    output.pop();
                }
                if let Some(args) = args {
                    output.extend(render_generic_args(args));
                }
            }
            if !param_names.is_empty() {
                output.extend(plus());
                output.extend(render_generic_bounds(param_names));
            }
            output
        } //  _serde::__private::Result | standard type
        Type::Generic(name) => vec![Token::generic(name)],
        Type::Primitive(name) => vec![Token::primitive(name)],
        Type::FunctionPointer(ptr) => {
            let mut output = render_higher_rank_trait_bounds(&ptr.generic_params);
            output.push(Token::kind("fn"));
            output.extend(render_fn_decl(&ptr.decl));
            output
        }
        Type::Tuple(types) => render_tuple(types),
        Type::Slice(ty) => {
            let mut output = vec![Token::symbol("[")];
            output.extend(render_type(ty));
            output.push(Token::symbol("]"));
            output
        }
        Type::Array { type_, len } => {
            let mut output = vec![Token::symbol("[")];
            output.extend(render_type(type_));
            output.extend(vec![
                Token::symbol(";"),
                ws!(),
                Token::primitive(len),
                Token::symbol("]"),
            ]);
            output
        }
        Type::ImplTrait(bounds) => {
            let mut output = vec![Token::keyword("impl")];
            output.push(ws!());
            output.extend(render_generic_bounds(bounds));
            output
        }
        Type::Infer => vec![Token::symbol("_")],
        Type::RawPointer { mutable, type_ } => {
            let mut output = vec![Token::symbol("*")];
            output.push(Token::keyword(if *mutable { "mut" } else { "const" }));
            output.push(ws!());
            output.extend(render_type(type_));
            output
        }
        Type::BorrowedRef {
            lifetime,
            mutable,
            type_,
        } => {
            let mut output = vec![Token::symbol("&")];
            if let Some(lt) = lifetime {
                output.extend(vec![Token::lifetime(lt), ws!()]);
            }
            if *mutable {
                output.extend(vec![Token::keyword("mut"), ws!()]);
            }
            output.extend(render_type(type_));
            output
        }
        Type::QualifiedPath {
            name,
            args: _,
            self_type,
            trait_,
        } => {
            let mut output = vec![Token::symbol("<")];
            output.extend(render_type(self_type));
            output.extend(vec![ws!(), Token::keyword("as"), ws!()]);
            output.extend(render_type(trait_));
            output.push(Token::symbol(">::"));
            output.push(Token::identifier(name));
            output
        }
    }
}

fn render_function(
    name: Vec<Token>,
    decl: &FnDecl,
    generics: &Generics,
    header: &Header,
) -> Vec<Token> {
    let mut output = vec![Token::qualifier("pub"), ws!()];
    if header.unsafe_ {
        output.extend(vec![Token::qualifier("unsafe"), ws!()]);
    };
    if header.const_ {
        output.extend(vec![Token::qualifier("const"), ws!()]);
    };
    if header.async_ {
        output.extend(vec![Token::qualifier("async"), ws!()]);
    };
    if header.abi != Abi::Rust {
        output.push(match &header.abi {
            Abi::C { .. } => Token::qualifier("c"),
            Abi::Cdecl { .. } => Token::qualifier("cdecl"),
            Abi::Stdcall { .. } => Token::qualifier("stdcall"),
            Abi::Fastcall { .. } => Token::qualifier("fastcall"),
            Abi::Aapcs { .. } => Token::qualifier("aapcs"),
            Abi::Win64 { .. } => Token::qualifier("win64"),
            Abi::SysV64 { .. } => Token::qualifier("sysV64"),
            Abi::System { .. } => Token::qualifier("system"),
            Abi::Other(text) => Token::qualifier(text),
            Abi::Rust => unreachable!(),
        });
        output.push(ws!());
    }

    output.extend(vec![Token::kind("fn"), ws!()]);
    output.extend(name);

    // Generic parameters
    output.extend(render_generic_param_defs(&generics.params));

    // Regular parameters and return type
    output.extend(render_fn_decl(decl));

    // Where predicates
    output.extend(render_where_predicates(&generics.where_predicates));

    output
}

fn render_fn_decl(decl: &FnDecl) -> Vec<Token> {
    let mut output = vec![];
    // Main arguments
    output.extend(render_sequence(
        vec![Token::symbol("(")],
        vec![Token::symbol(")")],
        comma(),
        &decl.inputs,
        |(name, ty)| {
            simplified_self(name, ty).unwrap_or_else(|| {
                let mut output = vec![];
                if name != "_" {
                    output.extend(vec![Token::identifier(name), Token::symbol(":"), ws!()]);
                }
                output.extend(render_type(ty));
                output
            })
        },
    ));
    // Return type
    if let Some(ty) = &decl.output {
        output.extend(arrow());
        output.extend(render_type(ty));
    }
    output
}

fn simplified_self(name: &str, ty: &Type) -> Option<Vec<Token>> {
    if name == "self" {
        match ty {
            Type::Generic(name) if name == "Self" => Some(vec![Token::self_("self")]),
            Type::BorrowedRef {
                lifetime,
                mutable,
                type_,
            } => match type_.as_ref() {
                Type::Generic(name) if name == "Self" => {
                    let mut output = vec![Token::symbol("&")];
                    if let Some(lt) = lifetime {
                        output.extend(vec![Token::lifetime(lt), ws!()]);
                    }
                    if *mutable {
                        output.extend(vec![Token::keyword("mut"), ws!()]);
                    }
                    output.push(Token::self_("self"));
                    Some(output)
                }
                _ => None,
            },
            _ => None,
        }
    } else {
        None
    }
}

fn render_tuple(types: &[Type]) -> Vec<Token> {
    render_sequence(
        vec![Token::symbol("(")],
        vec![Token::symbol(")")],
        comma(),
        types,
        render_type,
    )
}

enum Binding<'a> {
    GenericArg(&'a GenericArg),
    TypeBinding(&'a TypeBinding),
}

fn render_generic_args(args: &GenericArgs) -> Vec<Token> {
    match args {
        GenericArgs::AngleBracketed { args, bindings } => render_sequence_if_not_empty(
            vec![Token::symbol("<")],
            vec![Token::symbol(">")],
            comma(),
            &args
                .iter()
                .map(Binding::GenericArg)
                .chain(bindings.iter().map(Binding::TypeBinding))
                .collect::<Vec<_>>(),
            |arg| match arg {
                Binding::GenericArg(arg) => render_generic_arg(arg),
                Binding::TypeBinding(binding) => render_type_binding(binding),
            },
        ),
        GenericArgs::Parenthesized {
            inputs,
            output: return_ty,
        } => {
            let mut output = render_sequence(
                vec![Token::symbol("(")],
                vec![Token::symbol(")")],
                comma(),
                inputs,
                render_type,
            );
            if let Some(return_ty) = return_ty {
                output.extend(arrow());
                output.extend(render_type(return_ty));
            }
            output
        }
    }
}

fn render_term(term: &Term) -> Vec<Token> {
    match term {
        Term::Type(ty) => render_type(ty),
        Term::Constant(c) => render_constant(c),
    }
}

fn render_generic_arg(arg: &GenericArg) -> Vec<Token> {
    match arg {
        GenericArg::Lifetime(name) => vec![Token::lifetime(name)],
        GenericArg::Type(ty) => render_type(ty),
        GenericArg::Const(c) => render_constant(c),
        GenericArg::Infer => vec![Token::symbol("_")],
    }
}

fn render_type_binding(binding: &TypeBinding) -> Vec<Token> {
    let mut output = vec![Token::identifier(&binding.name)];
    output.extend(render_generic_args(&binding.args));
    match &binding.binding {
        TypeBindingKind::Equality(term) => {
            output.extend(equals());
            output.extend(render_term(term));
        }
        TypeBindingKind::Constraint(bounds) => {
            output.extend(render_generic_bounds(bounds));
        }
    }
    output
}

fn render_constant(constant: &Constant) -> Vec<Token> {
    let mut output = render_type(&constant.type_);
    if let Some(value) = &constant.value {
        output.extend(equals());
        if constant.is_literal {
            output.push(Token::primitive(value));
        } else {
            output.push(Token::identifier(value));
        }
    }
    output
}

fn render_generics(generics: &Generics) -> Vec<Token> {
    let mut output = vec![];
    output.extend(render_generic_param_defs(&generics.params));
    output.extend(render_where_predicates(&generics.where_predicates));
    output
}

fn render_generic_param_defs(params: &[GenericParamDef]) -> Vec<Token> {
    let params_without_synthetics: Vec<_> = params
        .iter()
        .filter(|p| {
            if let GenericParamDefKind::Type { synthetic, .. } = p.kind {
                !synthetic
            } else {
                true
            }
        })
        .collect();

    render_sequence_if_not_empty(
        vec![Token::symbol("<")],
        vec![Token::symbol(">")],
        comma(),
        &params_without_synthetics,
        |param| render_generic_param_def(param),
    )
}

fn render_generic_param_def(generic_param_def: &GenericParamDef) -> Vec<Token> {
    let mut output = vec![];
    match &generic_param_def.kind {
        GenericParamDefKind::Lifetime { outlives } => {
            output.push(Token::lifetime(&generic_param_def.name));
            if !outlives.is_empty() {
                output.extend(colon());
                output.extend(render_sequence(vec![], vec![], plus(), outlives, |s| {
                    vec![Token::lifetime(s)]
                }));
            }
        }
        GenericParamDefKind::Type { bounds, .. } => {
            output.push(Token::generic(&generic_param_def.name));
            if !bounds.is_empty() {
                output.extend(colon());
                output.extend(render_generic_bounds(bounds));
            }
        }
        GenericParamDefKind::Const { type_, .. } => {
            output.push(Token::qualifier("const"));
            output.push(ws!());
            output.push(Token::identifier(&generic_param_def.name));
            output.extend(colon());
            output.extend(render_type(type_));
        }
    }
    output
}

fn render_where_predicates(where_predicates: &[WherePredicate]) -> Vec<Token> {
    let mut output = vec![];
    if !where_predicates.is_empty() {
        output.push(ws!());
        output.push(Token::Keyword("where".to_owned()));
        output.push(ws!());
        output.extend(render_sequence(
            vec![],
            vec![],
            comma(),
            where_predicates,
            render_where_predicate,
        ));
    }
    output
}

fn render_where_predicate(where_predicate: &WherePredicate) -> Vec<Token> {
    let mut output = vec![];
    match where_predicate {
        WherePredicate::BoundPredicate {
            type_,
            bounds,
            generic_params,
        } => {
            output.extend(render_higher_rank_trait_bounds(generic_params));
            output.extend(render_type(type_));
            output.extend(colon());
            output.extend(render_generic_bounds(bounds));
        }
        WherePredicate::RegionPredicate {
            lifetime,
            bounds: _,
        } => output.push(Token::Lifetime(lifetime.clone())),
        WherePredicate::EqPredicate { lhs, rhs } => {
            output.extend(render_type(lhs));
            output.extend(equals());
            output.extend(render_term(rhs));
        }
    }
    output
}

fn render_generic_bounds(bounds: &[GenericBound]) -> Vec<Token> {
    render_sequence_if_not_empty(vec![], vec![], plus(), bounds, |bound| match bound {
        GenericBound::TraitBound {
            trait_,
            generic_params,
            ..
        } => {
            let mut output = vec![];
            output.extend(render_higher_rank_trait_bounds(generic_params));
            output.extend(render_type(trait_));
            output
        }
        GenericBound::Outlives(id) => vec![Token::lifetime(id)],
    })
}

fn render_higher_rank_trait_bounds(generic_params: &[GenericParamDef]) -> Vec<Token> {
    let mut output = vec![];
    if !generic_params.is_empty() {
        output.push(Token::keyword("for"));
        output.extend(render_generic_param_defs(generic_params));
        output.push(ws!());
    }
    output
}

fn plus() -> Vec<Token> {
    vec![ws!(), Token::symbol("+"), ws!()]
}

fn colon() -> Vec<Token> {
    vec![Token::symbol(":"), ws!()]
}

fn comma() -> Vec<Token> {
    vec![Token::symbol(","), ws!()]
}

fn equals() -> Vec<Token> {
    vec![ws!(), Token::symbol("="), ws!()]
}

fn arrow() -> Vec<Token> {
    vec![ws!(), Token::symbol("->"), ws!()]
}

#[cfg(test)]
mod test {
    macro_rules! parameterized_test {
        [$($name:ident: $function:ident($($values:expr,)*) => $expected:expr => $str:literal);*;] => {
            $(
            #[test]
            fn $name() {
                let value = $function($($values),*);
                assert_eq!(value, $expected);
                assert_eq!(crate::item_iterator::tokens_to_string(&value), $str);
            }
        )*
        };
    }

    macro_rules! s {
        ($value:literal) => {
            $value.to_string()
        };
    }

    use super::*;
    use rustdoc_types::Id;

    // Tests for the `render_type` function.
    // Missing:
    //  * ImplTrait
    //  * FunctionPointer
    parameterized_test![
        test_type_infer:
        render_type(&Type::Infer,)
        => vec![Token::symbol("_")]
        => "_";
        test_type_generic:
        render_type(&Type::Generic("name".to_string()),)
        => vec![Token::generic("name")]
        => "name";
        test_type_primitive:
        render_type(&Type::Primitive("name".to_string()),)
        => vec![Token::primitive("name")]
        => "name";
        test_type_resolved_simple:
        render_type(&Type::ResolvedPath{name: "name".to_string(), args: None, id: Id("id".to_string()), param_names: vec![]},)
        => vec![Token::type_("name")]
        => "name";
        test_type_resolved_long_name:
        render_type(&Type::ResolvedPath{name: "name::with::parts".to_string(), args: None, id: Id("id".to_string()), param_names: vec![]},)
        => vec![Token::identifier("name"), Token::symbol("::"), Token::identifier("with"), Token::symbol("::"), Token::type_("parts")]
        => "name::with::parts";
        test_type_resolved_crate_name:
        render_type(&Type::ResolvedPath{name: "$crate::name".to_string(), args: None, id: Id("id".to_string()), param_names: vec![]},)
        => vec![Token::identifier("$crate"), Token::symbol("::"), Token::type_("name")]
        => "$crate::name";
        test_type_resolved_name_crate:
        render_type(&Type::ResolvedPath{name: "name::$crate".to_string(), args: None, id: Id("id".to_string()), param_names: vec![]},)
        => vec![Token::identifier("name"), Token::symbol("::"), Token::type_("$crate")]
        => "name::$crate";
        test_type_tuple_empty:
        render_type(&Type::Tuple(vec![]),)
        => vec![Token::symbol("("), Token::symbol(")")]
        => "()";
        test_type_tuple:
        render_type(&Type::Tuple(vec![Type::Infer, Type::Generic(s!("gen"))]),)
        => vec![Token::symbol("("), Token::symbol("_"), Token::symbol(","), ws!(), Token::generic("gen"), Token::symbol(")")]
        => "(_, gen)";
        test_type_slice:
        render_type(&Type::Slice(Box::new(Type::Infer)),)
        => vec![Token::symbol("["), Token::symbol("_"), Token::symbol("]")]
        => "[_]";
        test_type_array:
        render_type(&Type::Array{type_:Box::new(Type::Infer), len: s!("20")},)
        => vec![Token::symbol("["), Token::symbol("_"), Token::symbol(";"), ws!(), Token::primitive("20"), Token::symbol("]")]
        => "[_; 20]";
        test_type_pointer:
        render_type(&Type::RawPointer { mutable: false, type_: Box::new(Type::Infer)},)
        => vec![Token::symbol("*"), Token::keyword("const"), ws!(), Token::symbol("_")]
        => "*const _";
        test_type_pointer_mut:
        render_type(&Type::RawPointer { mutable: true, type_: Box::new(Type::Infer)},)
        => vec![Token::symbol("*"), Token::keyword("mut"), ws!(), Token::symbol("_")]
        => "*mut _";
        test_type_ref:
        render_type(&Type::BorrowedRef { lifetime: None, mutable: false, type_: Box::new(Type::Infer)},)
        => vec![Token::symbol("&"), Token::symbol("_")]
        => "&_";
        test_type_ref_mut:
        render_type(&Type::BorrowedRef { lifetime: None, mutable: true, type_: Box::new(Type::Infer)},)
        => vec![Token::symbol("&"), Token::keyword("mut"), ws!(), Token::symbol("_")]
        => "&mut _";
        test_type_ref_lt:
        render_type(&Type::BorrowedRef { lifetime: Some(s!("'a")), mutable: false, type_: Box::new(Type::Infer)},)
        => vec![Token::symbol("&"), Token::lifetime("'a"), ws!(), Token::symbol("_")]
        => "&'a _";
        test_type_ref_lt_mut:
        render_type(&Type::BorrowedRef { lifetime: Some(s!("'a")), mutable: true, type_: Box::new(Type::Infer)},)
        => vec![Token::symbol("&"), Token::lifetime("'a"), ws!(), Token::keyword("mut"), ws!(), Token::symbol("_")]
        => "&'a mut _";
        test_type_path:
        render_type(&Type::QualifiedPath { name: s!("name"), args: Box::new(GenericArgs::AngleBracketed { args: vec![], bindings: vec![] }), self_type: Box::new(Type::Generic(s!("type"))), trait_: Box::new(Type::Generic(s!("trait"))) },)
        => vec![Token::symbol("<"), Token::generic("type"), ws!(), Token::keyword("as"), ws!(), Token::generic("trait"), Token::symbol(">::"), Token::identifier("name")]
        => "<type as trait>::name";
        //test_type_fn_pointer:
        //render_type(&Type::FunctionPointer(Box::new(FunctionPointer{
        //    decl: FnDecl{inputs: vec![(s!("a"), Type::Infer)], output: None, c_variadic: false},
        //    generic_params: vec![],
        //    header: Header{const_:false, unsafe_:false, async_:false, abi: Abi::Rust}})),)
        //=> vec![Token::symbol("<"), Token::generic("type"), ws!(), Token::keyword("as"), ws!(), Token::generic("trait"), Token::symbol(">::"), Token::identifier("name")].into()
        //=> "Fn(_)";
    ];
}
