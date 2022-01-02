use std::collections::HashSet;

#[test]
fn syntect_v4_6_0() {
    assert_public_items(
        include_str!("./rustdoc_json/syntect-v4.6.0-by-rust-nightly-2021-11-15.json"),
        EXPECTED_PUBLIC_ITEMS_IN_SYNTECT_V4_6_0,
    );
}

fn assert_public_items(rustdoc_json_str: &str, expected_public_items: &[&str]) {
    let actual = public_items::from_rustdoc_json_str(rustdoc_json_str).unwrap();

    let expected = string_hash_set_from_str_array(expected_public_items);

    assert_eq!(actual, expected);
}

fn string_hash_set_from_str_array(str_array: &[&str]) -> HashSet<String> {
    str_array.iter().map(|s| String::from(*s)).collect()
}

/// This list is sourced by browsing around
/// <https://docs.rs/syntect/4.6.0/syntect/all.html>. If an item is in the below
/// list, it can be found by browsing the above URL. Note that I have added an
/// explicit "syntect::" prefix on all items in the interest of clarity. Also
/// note that the JSON contains some extra items from
/// <https://github.com/bitflags/bitflags/commit/a74e4577d5> that are not
/// present in <https://docs.rs/syntect/4.6.0/syntect/all.html> due to different
/// `bitflags` versions used.
static EXPECTED_PUBLIC_ITEMS_IN_SYNTECT_V4_6_0: &[&str] = &[
    "syntect::dumps::dump_binary",
    "syntect::dumps::dump_to_file",
    "syntect::dumps::dump_to_writer",
    "syntect::dumps::from_binary",
    "syntect::dumps::from_dump_file",
    "syntect::dumps::from_reader",
    "syntect::dumps",
    "syntect::easy::HighlightFile::highlight_lines",
    "syntect::easy::HighlightFile::new",
    "syntect::easy::HighlightFile::reader",
    "syntect::easy::HighlightFile",
    "syntect::easy::HighlightLines::highlight",
    "syntect::easy::HighlightLines::new",
    "syntect::easy::HighlightLines",
    "syntect::easy::ScopeRegionIterator::fmt",
    "syntect::easy::ScopeRegionIterator::Item",
    "syntect::easy::ScopeRegionIterator::new",
    "syntect::easy::ScopeRegionIterator::next",
    "syntect::easy::ScopeRegionIterator",
    "syntect::easy",
    "syntect::highlighting::Color::a",
    "syntect::highlighting::Color::b",
    "syntect::highlighting::Color::BLACK",
    "syntect::highlighting::Color::clone",
    "syntect::highlighting::Color::deserialize",
    "syntect::highlighting::Color::eq",
    "syntect::highlighting::Color::Err",
    "syntect::highlighting::Color::fmt",
    "syntect::highlighting::Color::from_str",
    "syntect::highlighting::Color::g",
    "syntect::highlighting::Color::hash",
    "syntect::highlighting::Color::ne",
    "syntect::highlighting::Color::r",
    "syntect::highlighting::Color::serialize",
    "syntect::highlighting::Color::WHITE",
    "syntect::highlighting::Color",
    "syntect::highlighting::FontStyle::all",
    "syntect::highlighting::FontStyle::bitand_assign",
    "syntect::highlighting::FontStyle::bitand",
    "syntect::highlighting::FontStyle::bitor_assign",
    "syntect::highlighting::FontStyle::bitor",
    "syntect::highlighting::FontStyle::bits",
    "syntect::highlighting::FontStyle::bitxor_assign",
    "syntect::highlighting::FontStyle::bitxor",
    "syntect::highlighting::FontStyle::BOLD",
    "syntect::highlighting::FontStyle::clone",
    "syntect::highlighting::FontStyle::cmp",
    "syntect::highlighting::FontStyle::complement",
    "syntect::highlighting::FontStyle::contains",
    "syntect::highlighting::FontStyle::default",
    "syntect::highlighting::FontStyle::deserialize",
    "syntect::highlighting::FontStyle::difference",
    "syntect::highlighting::FontStyle::empty",
    "syntect::highlighting::FontStyle::eq",
    "syntect::highlighting::FontStyle::Err",
    "syntect::highlighting::FontStyle::extend",
    "syntect::highlighting::FontStyle::fmt",
    "syntect::highlighting::FontStyle::from_bits_truncate",
    "syntect::highlighting::FontStyle::from_bits_unchecked",
    "syntect::highlighting::FontStyle::from_bits",
    "syntect::highlighting::FontStyle::from_iter",
    "syntect::highlighting::FontStyle::from_str",
    "syntect::highlighting::FontStyle::hash",
    "syntect::highlighting::FontStyle::insert",
    "syntect::highlighting::FontStyle::intersection",
    "syntect::highlighting::FontStyle::intersects",
    "syntect::highlighting::FontStyle::is_all",
    "syntect::highlighting::FontStyle::is_empty",
    "syntect::highlighting::FontStyle::ITALIC",
    "syntect::highlighting::FontStyle::ne",
    "syntect::highlighting::FontStyle::not",
    "syntect::highlighting::FontStyle::Output",
    "syntect::highlighting::FontStyle::partial_cmp",
    "syntect::highlighting::FontStyle::remove",
    "syntect::highlighting::FontStyle::serialize",
    "syntect::highlighting::FontStyle::set",
    "syntect::highlighting::FontStyle::sub_assign",
    "syntect::highlighting::FontStyle::sub",
    "syntect::highlighting::FontStyle::symmetric_difference",
    "syntect::highlighting::FontStyle::toggle",
    "syntect::highlighting::FontStyle::UNDERLINE",
    "syntect::highlighting::FontStyle::union",
    "syntect::highlighting::FontStyle",
    "syntect::highlighting::Highlighter::fmt",
    "syntect::highlighting::Highlighter::get_default",
    "syntect::highlighting::Highlighter::new",
    "syntect::highlighting::Highlighter::style_for_stack",
    "syntect::highlighting::Highlighter::style_mod_for_stack",
    "syntect::highlighting::Highlighter",
    "syntect::highlighting::HighlightIterator::fmt",
    "syntect::highlighting::HighlightIterator::Item",
    "syntect::highlighting::HighlightIterator::new",
    "syntect::highlighting::HighlightIterator::next",
    "syntect::highlighting::HighlightIterator",
    "syntect::highlighting::HighlightState::clone",
    "syntect::highlighting::HighlightState::eq",
    "syntect::highlighting::HighlightState::fmt",
    "syntect::highlighting::HighlightState::ne",
    "syntect::highlighting::HighlightState::new",
    "syntect::highlighting::HighlightState::path",
    "syntect::highlighting::HighlightState",
    "syntect::highlighting::ParseThemeError::ColorShemeScopeIsNotObject",
    "syntect::highlighting::ParseThemeError::ColorShemeSettingsIsNotObject",
    "syntect::highlighting::ParseThemeError::DuplicateSettings",
    "syntect::highlighting::ParseThemeError::fmt",
    "syntect::highlighting::ParseThemeError::from",
    "syntect::highlighting::ParseThemeError::IncorrectColor",
    "syntect::highlighting::ParseThemeError::IncorrectFontStyle",
    "syntect::highlighting::ParseThemeError::IncorrectSettings",
    "syntect::highlighting::ParseThemeError::IncorrectSyntax",
    "syntect::highlighting::ParseThemeError::IncorrectUnderlineOption",
    "syntect::highlighting::ParseThemeError::ScopeParse",
    "syntect::highlighting::ParseThemeError::ScopeSelectorIsNotString",
    "syntect::highlighting::ParseThemeError::UndefinedScopeSettings",
    "syntect::highlighting::ParseThemeError::UndefinedSettings",
    "syntect::highlighting::ParseThemeError",
    "syntect::highlighting::RangedHighlightIterator::fmt",
    "syntect::highlighting::RangedHighlightIterator::Item",
    "syntect::highlighting::RangedHighlightIterator::new",
    "syntect::highlighting::RangedHighlightIterator::next",
    "syntect::highlighting::RangedHighlightIterator",
    "syntect::highlighting::ScopeSelector::clone",
    "syntect::highlighting::ScopeSelector::default",
    "syntect::highlighting::ScopeSelector::deserialize",
    "syntect::highlighting::ScopeSelector::does_match",
    "syntect::highlighting::ScopeSelector::eq",
    "syntect::highlighting::ScopeSelector::Err",
    "syntect::highlighting::ScopeSelector::excludes",
    "syntect::highlighting::ScopeSelector::extract_scopes",
    "syntect::highlighting::ScopeSelector::extract_single_scope",
    "syntect::highlighting::ScopeSelector::fmt",
    "syntect::highlighting::ScopeSelector::from_str",
    "syntect::highlighting::ScopeSelector::ne",
    "syntect::highlighting::ScopeSelector::path",
    "syntect::highlighting::ScopeSelector::serialize",
    "syntect::highlighting::ScopeSelector",
    "syntect::highlighting::ScopeSelectors::clone",
    "syntect::highlighting::ScopeSelectors::default",
    "syntect::highlighting::ScopeSelectors::deserialize",
    "syntect::highlighting::ScopeSelectors::does_match",
    "syntect::highlighting::ScopeSelectors::eq",
    "syntect::highlighting::ScopeSelectors::Err",
    "syntect::highlighting::ScopeSelectors::fmt",
    "syntect::highlighting::ScopeSelectors::from_str",
    "syntect::highlighting::ScopeSelectors::ne",
    "syntect::highlighting::ScopeSelectors::selectors",
    "syntect::highlighting::ScopeSelectors::serialize",
    "syntect::highlighting::ScopeSelectors",
    "syntect::highlighting::ScoredStyle::background",
    "syntect::highlighting::ScoredStyle::clone",
    "syntect::highlighting::ScoredStyle::eq",
    "syntect::highlighting::ScoredStyle::fmt",
    "syntect::highlighting::ScoredStyle::font_style",
    "syntect::highlighting::ScoredStyle::foreground",
    "syntect::highlighting::ScoredStyle::ne",
    "syntect::highlighting::ScoredStyle",
    "syntect::highlighting::SettingsError::fmt",
    "syntect::highlighting::SettingsError::from",
    "syntect::highlighting::SettingsError::Plist",
    "syntect::highlighting::SettingsError",
    "syntect::highlighting::Style::apply",
    "syntect::highlighting::Style::background",
    "syntect::highlighting::Style::clone",
    "syntect::highlighting::Style::default",
    "syntect::highlighting::Style::deserialize",
    "syntect::highlighting::Style::eq",
    "syntect::highlighting::Style::fmt",
    "syntect::highlighting::Style::font_style",
    "syntect::highlighting::Style::foreground",
    "syntect::highlighting::Style::hash",
    "syntect::highlighting::Style::ne",
    "syntect::highlighting::Style::serialize",
    "syntect::highlighting::Style",
    "syntect::highlighting::StyleModifier::apply",
    "syntect::highlighting::StyleModifier::background",
    "syntect::highlighting::StyleModifier::clone",
    "syntect::highlighting::StyleModifier::default",
    "syntect::highlighting::StyleModifier::deserialize",
    "syntect::highlighting::StyleModifier::eq",
    "syntect::highlighting::StyleModifier::fmt",
    "syntect::highlighting::StyleModifier::font_style",
    "syntect::highlighting::StyleModifier::foreground",
    "syntect::highlighting::StyleModifier::ne",
    "syntect::highlighting::StyleModifier::serialize",
    "syntect::highlighting::StyleModifier",
    "syntect::highlighting::Theme::author",
    "syntect::highlighting::Theme::clone",
    "syntect::highlighting::Theme::default",
    "syntect::highlighting::Theme::deserialize",
    "syntect::highlighting::Theme::fmt",
    "syntect::highlighting::Theme::name",
    "syntect::highlighting::Theme::scopes",
    "syntect::highlighting::Theme::serialize",
    "syntect::highlighting::Theme::settings",
    "syntect::highlighting::Theme",
    "syntect::highlighting::ThemeItem::clone",
    "syntect::highlighting::ThemeItem::default",
    "syntect::highlighting::ThemeItem::deserialize",
    "syntect::highlighting::ThemeItem::fmt",
    "syntect::highlighting::ThemeItem::scope",
    "syntect::highlighting::ThemeItem::serialize",
    "syntect::highlighting::ThemeItem::style",
    "syntect::highlighting::ThemeItem",
    "syntect::highlighting::ThemeSet::add_from_folder",
    "syntect::highlighting::ThemeSet::default",
    "syntect::highlighting::ThemeSet::deserialize",
    "syntect::highlighting::ThemeSet::discover_theme_paths",
    "syntect::highlighting::ThemeSet::fmt",
    "syntect::highlighting::ThemeSet::get_theme",
    "syntect::highlighting::ThemeSet::load_defaults",
    "syntect::highlighting::ThemeSet::load_from_folder",
    "syntect::highlighting::ThemeSet::load_from_reader",
    "syntect::highlighting::ThemeSet::new",
    "syntect::highlighting::ThemeSet::serialize",
    "syntect::highlighting::ThemeSet::themes",
    "syntect::highlighting::ThemeSet",
    "syntect::highlighting::ThemeSettings::accent",
    "syntect::highlighting::ThemeSettings::active_guide",
    "syntect::highlighting::ThemeSettings::background",
    "syntect::highlighting::ThemeSettings::bracket_contents_foreground",
    "syntect::highlighting::ThemeSettings::bracket_contents_options",
    "syntect::highlighting::ThemeSettings::brackets_background",
    "syntect::highlighting::ThemeSettings::brackets_foreground",
    "syntect::highlighting::ThemeSettings::brackets_options",
    "syntect::highlighting::ThemeSettings::caret",
    "syntect::highlighting::ThemeSettings::clone",
    "syntect::highlighting::ThemeSettings::default",
    "syntect::highlighting::ThemeSettings::deserialize",
    "syntect::highlighting::ThemeSettings::find_highlight_foreground",
    "syntect::highlighting::ThemeSettings::find_highlight",
    "syntect::highlighting::ThemeSettings::fmt",
    "syntect::highlighting::ThemeSettings::foreground",
    "syntect::highlighting::ThemeSettings::guide",
    "syntect::highlighting::ThemeSettings::gutter_foreground",
    "syntect::highlighting::ThemeSettings::gutter",
    "syntect::highlighting::ThemeSettings::highlight_foreground",
    "syntect::highlighting::ThemeSettings::highlight",
    "syntect::highlighting::ThemeSettings::inactive_selection_foreground",
    "syntect::highlighting::ThemeSettings::inactive_selection",
    "syntect::highlighting::ThemeSettings::line_highlight",
    "syntect::highlighting::ThemeSettings::minimap_border",
    "syntect::highlighting::ThemeSettings::misspelling",
    "syntect::highlighting::ThemeSettings::phantom_css",
    "syntect::highlighting::ThemeSettings::popup_css",
    "syntect::highlighting::ThemeSettings::selection_background",
    "syntect::highlighting::ThemeSettings::selection_border",
    "syntect::highlighting::ThemeSettings::selection_foreground",
    "syntect::highlighting::ThemeSettings::selection",
    "syntect::highlighting::ThemeSettings::serialize",
    "syntect::highlighting::ThemeSettings::shadow",
    "syntect::highlighting::ThemeSettings::stack_guide",
    "syntect::highlighting::ThemeSettings::tags_foreground",
    "syntect::highlighting::ThemeSettings::tags_options",
    "syntect::highlighting::ThemeSettings",
    "syntect::highlighting::UnderlineOption::clone",
    "syntect::highlighting::UnderlineOption::default",
    "syntect::highlighting::UnderlineOption::deserialize",
    "syntect::highlighting::UnderlineOption::Err",
    "syntect::highlighting::UnderlineOption::fmt",
    "syntect::highlighting::UnderlineOption::from_str",
    "syntect::highlighting::UnderlineOption::None",
    "syntect::highlighting::UnderlineOption::serialize",
    "syntect::highlighting::UnderlineOption::SquigglyUnderline",
    "syntect::highlighting::UnderlineOption::StippledUnderline",
    "syntect::highlighting::UnderlineOption::Underline",
    "syntect::highlighting::UnderlineOption",
    "syntect::highlighting",
    "syntect::html::append_highlighted_html_for_styled_line",
    "syntect::html::ClassedHTMLGenerator::finalize",
    "syntect::html::ClassedHTMLGenerator::new_with_class_style",
    "syntect::html::ClassedHTMLGenerator::new",
    "syntect::html::ClassedHTMLGenerator::parse_html_for_line_which_includes_newline",
    "syntect::html::ClassedHTMLGenerator::parse_html_for_line",
    "syntect::html::ClassedHTMLGenerator",
    "syntect::html::ClassStyle::clone",
    "syntect::html::ClassStyle::eq",
    "syntect::html::ClassStyle::fmt",
    "syntect::html::ClassStyle::ne",
    "syntect::html::ClassStyle::Spaced",
    "syntect::html::ClassStyle::SpacedPrefixed",
    "syntect::html::ClassStyle",
    "syntect::html::css_for_theme_with_class_style",
    "syntect::html::css_for_theme",
    "syntect::html::highlighted_html_for_file",
    "syntect::html::highlighted_html_for_string",
    "syntect::html::IncludeBackground::clone",
    "syntect::html::IncludeBackground::eq",
    "syntect::html::IncludeBackground::fmt",
    "syntect::html::IncludeBackground::IfDifferent",
    "syntect::html::IncludeBackground::ne",
    "syntect::html::IncludeBackground::No",
    "syntect::html::IncludeBackground::Yes",
    "syntect::html::IncludeBackground",
    "syntect::html::line_tokens_to_classed_spans",
    "syntect::html::start_highlighted_html_snippet",
    "syntect::html::styled_line_to_highlighted_html",
    "syntect::html::tokens_to_classed_html",
    "syntect::html::tokens_to_classed_spans",
    "syntect::html",
    "syntect::LoadingError::BadPath",
    "syntect::LoadingError::cause",
    "syntect::LoadingError::fmt",
    "syntect::LoadingError::from",
    "syntect::LoadingError::Io",
    "syntect::LoadingError::ParseSyntax",
    "syntect::LoadingError::ParseTheme",
    "syntect::LoadingError::ReadSettings",
    "syntect::LoadingError::WalkDir",
    "syntect::LoadingError",
    "syntect::parsing::ATOM_LEN_BITS",
    "syntect::parsing::BasicScopeStackOp::clone",
    "syntect::parsing::BasicScopeStackOp::eq",
    "syntect::parsing::BasicScopeStackOp::fmt",
    "syntect::parsing::BasicScopeStackOp::ne",
    "syntect::parsing::BasicScopeStackOp::Pop",
    "syntect::parsing::BasicScopeStackOp::Push",
    "syntect::parsing::BasicScopeStackOp",
    "syntect::parsing::ClearAmount::All",
    "syntect::parsing::ClearAmount::clone",
    "syntect::parsing::ClearAmount::deserialize",
    "syntect::parsing::ClearAmount::eq",
    "syntect::parsing::ClearAmount::fmt",
    "syntect::parsing::ClearAmount::ne",
    "syntect::parsing::ClearAmount::serialize",
    "syntect::parsing::ClearAmount::TopN",
    "syntect::parsing::ClearAmount",
    "syntect::parsing::MatchPower::0",
    "syntect::parsing::MatchPower::clone",
    "syntect::parsing::MatchPower::cmp",
    "syntect::parsing::MatchPower::eq",
    "syntect::parsing::MatchPower::fmt",
    "syntect::parsing::MatchPower::ne",
    "syntect::parsing::MatchPower::partial_cmp",
    "syntect::parsing::MatchPower",
    "syntect::parsing::ParseScopeError::fmt",
    "syntect::parsing::ParseScopeError::TooLong",
    "syntect::parsing::ParseScopeError::TooManyAtoms",
    "syntect::parsing::ParseScopeError",
    "syntect::parsing::ParseState::clone",
    "syntect::parsing::ParseState::eq",
    "syntect::parsing::ParseState::fmt",
    "syntect::parsing::ParseState::ne",
    "syntect::parsing::ParseState::new",
    "syntect::parsing::ParseState::parse_line",
    "syntect::parsing::ParseState",
    "syntect::parsing::ParseSyntaxError::BadFileRef",
    "syntect::parsing::ParseSyntaxError::cause",
    "syntect::parsing::ParseSyntaxError::EmptyFile",
    "syntect::parsing::ParseSyntaxError::fmt",
    "syntect::parsing::ParseSyntaxError::InvalidScope",
    "syntect::parsing::ParseSyntaxError::InvalidYaml",
    "syntect::parsing::ParseSyntaxError::MainMissing",
    "syntect::parsing::ParseSyntaxError::MissingMandatoryKey",
    "syntect::parsing::ParseSyntaxError::RegexCompileError",
    "syntect::parsing::ParseSyntaxError::TypeMismatch",
    "syntect::parsing::ParseSyntaxError",
    "syntect::parsing::Regex::clone",
    "syntect::parsing::Regex::deserialize",
    "syntect::parsing::Regex::eq",
    "syntect::parsing::Regex::fmt",
    "syntect::parsing::Regex::is_match",
    "syntect::parsing::Regex::new",
    "syntect::parsing::Regex::regex_str",
    "syntect::parsing::Regex::search",
    "syntect::parsing::Regex::serialize",
    "syntect::parsing::Regex::try_compile",
    "syntect::parsing::Regex",
    "syntect::parsing::Region::clone",
    "syntect::parsing::Region::eq",
    "syntect::parsing::Region::fmt",
    "syntect::parsing::Region::ne",
    "syntect::parsing::Region::new",
    "syntect::parsing::Region::pos",
    "syntect::parsing::Region",
    "syntect::parsing::SCOPE_REPO::deref",
    "syntect::parsing::SCOPE_REPO::Target",
    "syntect::parsing::SCOPE_REPO",
    "syntect::parsing::Scope::atom_at",
    "syntect::parsing::Scope::build_string",
    "syntect::parsing::Scope::clone",
    "syntect::parsing::Scope::cmp",
    "syntect::parsing::Scope::default",
    "syntect::parsing::Scope::deserialize",
    "syntect::parsing::Scope::eq",
    "syntect::parsing::Scope::Err",
    "syntect::parsing::Scope::fmt",
    "syntect::parsing::Scope::from_str",
    "syntect::parsing::Scope::hash",
    "syntect::parsing::Scope::is_empty",
    "syntect::parsing::Scope::is_prefix_of",
    "syntect::parsing::Scope::len",
    "syntect::parsing::Scope::ne",
    "syntect::parsing::Scope::new",
    "syntect::parsing::Scope::partial_cmp",
    "syntect::parsing::Scope::serialize",
    "syntect::parsing::Scope",
    "syntect::parsing::ScopeRepository::atom_str",
    "syntect::parsing::ScopeRepository::build",
    "syntect::parsing::ScopeRepository::fmt",
    "syntect::parsing::ScopeRepository::to_string",
    "syntect::parsing::ScopeRepository",
    "syntect::parsing::ScopeStack::apply_with_hook",
    "syntect::parsing::ScopeStack::apply",
    "syntect::parsing::ScopeStack::as_slice",
    "syntect::parsing::ScopeStack::bottom_n",
    "syntect::parsing::ScopeStack::clone",
    "syntect::parsing::ScopeStack::debug_print",
    "syntect::parsing::ScopeStack::default",
    "syntect::parsing::ScopeStack::deserialize",
    "syntect::parsing::ScopeStack::does_match",
    "syntect::parsing::ScopeStack::eq",
    "syntect::parsing::ScopeStack::Err",
    "syntect::parsing::ScopeStack::fmt",
    "syntect::parsing::ScopeStack::from_str",
    "syntect::parsing::ScopeStack::from_vec",
    "syntect::parsing::ScopeStack::is_empty",
    "syntect::parsing::ScopeStack::len",
    "syntect::parsing::ScopeStack::ne",
    "syntect::parsing::ScopeStack::new",
    "syntect::parsing::ScopeStack::pop",
    "syntect::parsing::ScopeStack::push",
    "syntect::parsing::ScopeStack::scopes",
    "syntect::parsing::ScopeStack::serialize",
    "syntect::parsing::ScopeStack",
    "syntect::parsing::ScopeStackOp::Clear",
    "syntect::parsing::ScopeStackOp::clone",
    "syntect::parsing::ScopeStackOp::eq",
    "syntect::parsing::ScopeStackOp::fmt",
    "syntect::parsing::ScopeStackOp::ne",
    "syntect::parsing::ScopeStackOp::Noop",
    "syntect::parsing::ScopeStackOp::Pop",
    "syntect::parsing::ScopeStackOp::Push",
    "syntect::parsing::ScopeStackOp::Restore",
    "syntect::parsing::ScopeStackOp",
    "syntect::parsing::syntax_definition::CaptureMapping",
    "syntect::parsing::syntax_definition::context_iter",
    "syntect::parsing::syntax_definition::Context::clear_scopes",
    "syntect::parsing::syntax_definition::Context::clone",
    "syntect::parsing::syntax_definition::Context::deserialize",
    "syntect::parsing::syntax_definition::Context::eq",
    "syntect::parsing::syntax_definition::Context::fmt",
    "syntect::parsing::syntax_definition::Context::match_at",
    "syntect::parsing::syntax_definition::Context::meta_content_scope",
    "syntect::parsing::syntax_definition::Context::meta_include_prototype",
    "syntect::parsing::syntax_definition::Context::meta_scope",
    "syntect::parsing::syntax_definition::Context::ne",
    "syntect::parsing::syntax_definition::Context::new",
    "syntect::parsing::syntax_definition::Context::patterns",
    "syntect::parsing::syntax_definition::Context::prototype",
    "syntect::parsing::syntax_definition::Context::serialize",
    "syntect::parsing::syntax_definition::Context::uses_backrefs",
    "syntect::parsing::syntax_definition::Context",
    "syntect::parsing::syntax_definition::ContextId::clone",
    "syntect::parsing::syntax_definition::ContextId::deserialize",
    "syntect::parsing::syntax_definition::ContextId::eq",
    "syntect::parsing::syntax_definition::ContextId::fmt",
    "syntect::parsing::syntax_definition::ContextId::ne",
    "syntect::parsing::syntax_definition::ContextId::new",
    "syntect::parsing::syntax_definition::ContextId::serialize",
    "syntect::parsing::syntax_definition::ContextId",
    "syntect::parsing::syntax_definition::ContextReference::ByScope",
    "syntect::parsing::syntax_definition::ContextReference::clone",
    "syntect::parsing::syntax_definition::ContextReference::deserialize",
    "syntect::parsing::syntax_definition::ContextReference::Direct",
    "syntect::parsing::syntax_definition::ContextReference::eq",
    "syntect::parsing::syntax_definition::ContextReference::File",
    "syntect::parsing::syntax_definition::ContextReference::fmt",
    "syntect::parsing::syntax_definition::ContextReference::id",
    "syntect::parsing::syntax_definition::ContextReference::Inline",
    "syntect::parsing::syntax_definition::ContextReference::Named",
    "syntect::parsing::syntax_definition::ContextReference::ne",
    "syntect::parsing::syntax_definition::ContextReference::resolve",
    "syntect::parsing::syntax_definition::ContextReference::serialize",
    "syntect::parsing::syntax_definition::ContextReference",
    "syntect::parsing::syntax_definition::MatchIter::fmt",
    "syntect::parsing::syntax_definition::MatchIter::Item",
    "syntect::parsing::syntax_definition::MatchIter::next",
    "syntect::parsing::syntax_definition::MatchIter",
    "syntect::parsing::syntax_definition::MatchOperation::clone",
    "syntect::parsing::syntax_definition::MatchOperation::deserialize",
    "syntect::parsing::syntax_definition::MatchOperation::eq",
    "syntect::parsing::syntax_definition::MatchOperation::fmt",
    "syntect::parsing::syntax_definition::MatchOperation::ne",
    "syntect::parsing::syntax_definition::MatchOperation::None",
    "syntect::parsing::syntax_definition::MatchOperation::Pop",
    "syntect::parsing::syntax_definition::MatchOperation::Push",
    "syntect::parsing::syntax_definition::MatchOperation::serialize",
    "syntect::parsing::syntax_definition::MatchOperation::Set",
    "syntect::parsing::syntax_definition::MatchOperation",
    "syntect::parsing::syntax_definition::MatchPattern::captures",
    "syntect::parsing::syntax_definition::MatchPattern::clone",
    "syntect::parsing::syntax_definition::MatchPattern::deserialize",
    "syntect::parsing::syntax_definition::MatchPattern::eq",
    "syntect::parsing::syntax_definition::MatchPattern::fmt",
    "syntect::parsing::syntax_definition::MatchPattern::has_captures",
    "syntect::parsing::syntax_definition::MatchPattern::ne",
    "syntect::parsing::syntax_definition::MatchPattern::new",
    "syntect::parsing::syntax_definition::MatchPattern::operation",
    "syntect::parsing::syntax_definition::MatchPattern::regex_with_refs",
    "syntect::parsing::syntax_definition::MatchPattern::regex",
    "syntect::parsing::syntax_definition::MatchPattern::scope",
    "syntect::parsing::syntax_definition::MatchPattern::serialize",
    "syntect::parsing::syntax_definition::MatchPattern::with_prototype",
    "syntect::parsing::syntax_definition::MatchPattern",
    "syntect::parsing::syntax_definition::Pattern::clone",
    "syntect::parsing::syntax_definition::Pattern::deserialize",
    "syntect::parsing::syntax_definition::Pattern::eq",
    "syntect::parsing::syntax_definition::Pattern::fmt",
    "syntect::parsing::syntax_definition::Pattern::Include",
    "syntect::parsing::syntax_definition::Pattern::Match",
    "syntect::parsing::syntax_definition::Pattern::ne",
    "syntect::parsing::syntax_definition::Pattern::serialize",
    "syntect::parsing::syntax_definition::Pattern",
    "syntect::parsing::syntax_definition::SyntaxDefinition::clone",
    "syntect::parsing::syntax_definition::SyntaxDefinition::contexts",
    "syntect::parsing::syntax_definition::SyntaxDefinition::deserialize",
    "syntect::parsing::syntax_definition::SyntaxDefinition::eq",
    "syntect::parsing::syntax_definition::SyntaxDefinition::file_extensions",
    "syntect::parsing::syntax_definition::SyntaxDefinition::first_line_match",
    "syntect::parsing::syntax_definition::SyntaxDefinition::fmt",
    "syntect::parsing::syntax_definition::SyntaxDefinition::hidden",
    "syntect::parsing::syntax_definition::SyntaxDefinition::load_from_str",
    "syntect::parsing::syntax_definition::SyntaxDefinition::name",
    "syntect::parsing::syntax_definition::SyntaxDefinition::ne",
    "syntect::parsing::syntax_definition::SyntaxDefinition::scope",
    "syntect::parsing::syntax_definition::SyntaxDefinition::serialize",
    "syntect::parsing::syntax_definition::SyntaxDefinition::variables",
    "syntect::parsing::syntax_definition::SyntaxDefinition",
    "syntect::parsing::syntax_definition",
    "syntect::parsing::SyntaxDefinition", // Note that this is a re-export
    "syntect::parsing::SyntaxReference::clone",
    "syntect::parsing::SyntaxReference::deserialize",
    "syntect::parsing::SyntaxReference::file_extensions",
    "syntect::parsing::SyntaxReference::first_line_match",
    "syntect::parsing::SyntaxReference::fmt",
    "syntect::parsing::SyntaxReference::hidden",
    "syntect::parsing::SyntaxReference::name",
    "syntect::parsing::SyntaxReference::scope",
    "syntect::parsing::SyntaxReference::serialize",
    "syntect::parsing::SyntaxReference::variables",
    "syntect::parsing::SyntaxReference",
    "syntect::parsing::SyntaxSet::clone",
    "syntect::parsing::SyntaxSet::default",
    "syntect::parsing::SyntaxSet::deserialize",
    "syntect::parsing::SyntaxSet::find_syntax_by_extension",
    "syntect::parsing::SyntaxSet::find_syntax_by_first_line",
    "syntect::parsing::SyntaxSet::find_syntax_by_name",
    "syntect::parsing::SyntaxSet::find_syntax_by_path",
    "syntect::parsing::SyntaxSet::find_syntax_by_scope",
    "syntect::parsing::SyntaxSet::find_syntax_by_token",
    "syntect::parsing::SyntaxSet::find_syntax_for_file",
    "syntect::parsing::SyntaxSet::find_syntax_plain_text",
    "syntect::parsing::SyntaxSet::find_unlinked_contexts",
    "syntect::parsing::SyntaxSet::fmt",
    "syntect::parsing::SyntaxSet::into_builder",
    "syntect::parsing::SyntaxSet::load_defaults_newlines",
    "syntect::parsing::SyntaxSet::load_defaults_nonewlines",
    "syntect::parsing::SyntaxSet::load_from_folder",
    "syntect::parsing::SyntaxSet::new",
    "syntect::parsing::SyntaxSet::serialize",
    "syntect::parsing::SyntaxSet::syntaxes",
    "syntect::parsing::SyntaxSet",
    "syntect::parsing::SyntaxSetBuilder::add_from_folder",
    "syntect::parsing::SyntaxSetBuilder::add_plain_text_syntax",
    "syntect::parsing::SyntaxSetBuilder::add",
    "syntect::parsing::SyntaxSetBuilder::build",
    "syntect::parsing::SyntaxSetBuilder::clone",
    "syntect::parsing::SyntaxSetBuilder::default",
    "syntect::parsing::SyntaxSetBuilder::new",
    "syntect::parsing::SyntaxSetBuilder::syntaxes",
    "syntect::parsing::SyntaxSetBuilder",
    "syntect::parsing",
    "syntect::util::as_24_bit_terminal_escaped",
    "syntect::util::as_latex_escaped",
    "syntect::util::debug_print_ops",
    "syntect::util::LinesWithEndings::from",
    "syntect::util::LinesWithEndings::Item",
    "syntect::util::LinesWithEndings::next",
    "syntect::util::LinesWithEndings",
    "syntect::util::modify_range",
    "syntect::util::split_at",
    "syntect::util",
    "syntect",
];
