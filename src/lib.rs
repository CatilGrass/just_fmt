/// Format naming styles
///
/// Provides multiple naming style conversion functions, supporting conversion from input strings
/// in different formats to standardized word lists, then outputting to various common naming styles.
///
/// # Main Features
///
/// - Create `CaseFormatter` from a string
/// - Intelligently split input strings into word lists, handling multiple separators and case boundaries
/// - Convert to multiple naming formats: `camelCase`, `PascalCase`, `snake_case`, `kebab-case`, etc.
///
/// # Examples
///
/// ```
/// # use just_fmt::fmt_case::CaseFormatter;
/// // Using CaseFormatter
/// let formatter = CaseFormatter::from("brew_coffee");
/// assert_eq!(formatter.to_camel_case(), "brewCoffee");
/// assert_eq!(formatter.to_pascal_case(), "BrewCoffee");
/// assert_eq!(formatter.to_snake_case(), "brew_coffee");
/// assert_eq!(formatter.to_kebab_case(), "brew-coffee");
///
/// // Using macros
/// # use just_fmt::fmt_case::{camel_case, pascal_case, snake_case, kebab_case}
/// assert_eq!(camel_case!("brew coffee"), "brewCoffee");
/// assert_eq!(pascal_case!("brewCoffee"), "BrewCoffee");
/// assert_eq!(snake_case!("brew_coffee"), "brew_coffee");
/// assert_eq!(kebab_case!("brew.Coffee"), "brew-coffee");
/// ```
///
/// # Supported Input Separators
///
/// The module can recognize the following characters as word separators:
/// - Underscore `_`
/// - Comma `,`
/// - Dot `.`
/// - Hyphen `-`
/// - Space ` `
///
/// It can also automatically detect case boundaries (e.g., "camel" and "Case" in "camelCase")
pub mod fmt_case_style;

/// Normalize an input path string into a canonical, platform‑agnostic form.
///
/// This function removes ANSI escape sequences, unifies separators to `/`,
/// collapses duplicate slashes, strips unfriendly characters (`*`, `?`, `"`, `<`, `>`, `|`),
/// resolves simple `..` components, and preserves a trailing slash when present.
pub mod fmt_path;
