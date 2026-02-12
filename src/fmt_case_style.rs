pub struct CaseFormatter {
    content: Vec<String>,
}

impl From<String> for CaseFormatter {
    fn from(value: String) -> Self {
        Self {
            content: str_split(value),
        }
    }
}

impl From<&String> for CaseFormatter {
    fn from(value: &String) -> Self {
        Self {
            content: str_split(value.clone()),
        }
    }
}

impl From<&str> for CaseFormatter {
    fn from(value: &str) -> Self {
        Self {
            content: str_split(value.to_string()),
        }
    }
}

/// Split the string into segments for conversion
fn str_split(input: String) -> Vec<String> {
    let mut result = String::new();
    let mut prev_space = false;

    for c in input.chars() {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => {
                result.push(c);
                prev_space = false;
            }
            '_' | ',' | '.' | '-' | ' ' => {
                if !prev_space {
                    result.push(' ');
                    prev_space = true;
                }
            }
            _ => {}
        }
    }

    let mut processed = String::new();
    let mut chars = result.chars().peekable();

    while let Some(c) = chars.next() {
        processed.push(c);

        // Detect case boundaries:
        // when the current character is lowercase and the next is uppercase (e.g., "bre[wC]offee")
        // Treat as a word boundary in PascalCase or camelCase, insert a space
        if let Some(&next) = chars.peek()
            && c.is_lowercase()
            && next.is_uppercase()
        {
            processed.push(' ');
        }
    }

    processed
        .to_lowercase()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

impl CaseFormatter {
    /// Convert to camelCase format (brewCoffee)
    ///
    /// # Examples
    ///
    /// ```
    /// # use just_fmt::fmt_case_style::CaseFormatter;
    /// let processor = CaseFormatter::from("brew_coffee");
    /// assert_eq!(processor.to_camel_case(), "brewCoffee");
    /// ```
    pub fn to_camel_case(&self) -> String {
        let mut result = String::new();
        for (i, word) in self.content.iter().enumerate() {
            if i == 0 {
                result.push_str(&word.to_lowercase());
            } else {
                let mut chars = word.chars();
                if let Some(first) = chars.next() {
                    result.push_str(&first.to_uppercase().collect::<String>());
                    result.push_str(&chars.collect::<String>().to_lowercase());
                }
            }
        }
        result
    }

    /// Convert to PascalCase format (BrewCoffee)
    ///
    /// # Examples
    ///
    /// ```
    /// # use just_fmt::fmt_case_style::CaseFormatter;
    /// let processor = CaseFormatter::from("brew_coffee");
    /// assert_eq!(processor.to_pascal_case(), "BrewCoffee");
    /// ```
    pub fn to_pascal_case(&self) -> String {
        let mut result = String::new();
        for word in &self.content {
            let mut chars = word.chars();
            if let Some(first) = chars.next() {
                result.push_str(&first.to_uppercase().collect::<String>());
                result.push_str(&chars.collect::<String>().to_lowercase());
            }
        }
        result
    }

    /// Convert to kebab-case format (brew-coffee)
    ///
    /// # Examples
    ///
    /// ```
    /// # use just_fmt::fmt_case_style::CaseFormatter;
    /// let processor = CaseFormatter::from("brew_coffee");
    /// assert_eq!(processor.to_kebab_case(), "brew-coffee");
    /// ```
    pub fn to_kebab_case(&self) -> String {
        self.content.join("-").to_lowercase()
    }

    /// Convert to snake_case format (brew_coffee)
    ///
    /// # Examples
    ///
    /// ```
    /// # use just_fmt::fmt_case_style::CaseFormatter;
    /// let processor = CaseFormatter::from("brewCoffee");
    /// assert_eq!(processor.to_snake_case(), "brew_coffee");
    /// ```
    pub fn to_snake_case(&self) -> String {
        self.content.join("_").to_lowercase()
    }

    /// Convert to dot.case format (brew.coffee)
    ///
    /// # Examples
    ///
    /// ```
    /// # use just_fmt::fmt_case_style::CaseFormatter;
    /// let processor = CaseFormatter::from("brew_coffee");
    /// assert_eq!(processor.to_dot_case(), "brew.coffee");
    /// ```
    pub fn to_dot_case(&self) -> String {
        self.content.join(".").to_lowercase()
    }

    /// Convert to Title Case format (Brew Coffee)
    ///
    /// # Examples
    ///
    /// ```
    /// # use just_fmt::fmt_case_style::CaseFormatter;
    /// let processor = CaseFormatter::from("brew_coffee");
    /// assert_eq!(processor.to_title_case(), "Brew Coffee");
    /// ```
    pub fn to_title_case(&self) -> String {
        let mut result = String::new();
        for word in &self.content {
            let mut chars = word.chars();
            if let Some(first) = chars.next() {
                result.push_str(&first.to_uppercase().collect::<String>());
                result.push_str(&chars.collect::<String>().to_lowercase());
            }
            result.push(' ');
        }
        result.pop();
        result
    }

    /// Convert to lower case format (brew coffee)
    ///
    /// # Examples
    ///
    /// ```
    /// # use just_fmt::fmt_case_style::CaseFormatter;
    /// let processor = CaseFormatter::from("BREW COFFEE");
    /// assert_eq!(processor.to_lower_case(), "brew coffee");
    /// ```
    pub fn to_lower_case(&self) -> String {
        self.content.join(" ").to_lowercase()
    }

    /// Convert to UPPER CASE format (BREW COFFEE)
    ///
    /// # Examples
    ///
    /// ```
    /// # use just_fmt::fmt_case_style::CaseFormatter;
    /// let processor = CaseFormatter::from("brew coffee");
    /// assert_eq!(processor.to_upper_case(), "BREW COFFEE");
    /// ```
    pub fn to_upper_case(&self) -> String {
        self.content.join(" ").to_uppercase()
    }
}

#[cfg(test)]
mod tests {
    use crate::fmt_case_style::CaseFormatter;

    #[test]
    fn test_processer() {
        let test_cases = vec![
            ("brew_coffee", "brewCoffee"),
            ("brew, coffee", "brewCoffee"),
            ("brew-coffee", "brewCoffee"),
            ("Brew.Coffee", "brewCoffee"),
            ("bRewCofFee", "bRewCofFee"),
            ("brewCoffee", "brewCoffee"),
            ("b&rewCoffee", "brewCoffee"),
            ("BrewCoffee", "brewCoffee"),
            ("brew.coffee", "brewCoffee"),
            ("Brew_Coffee", "brewCoffee"),
            ("BREW COFFEE", "brewCoffee"),
        ];

        for (input, expected) in test_cases {
            let processor = CaseFormatter::from(input);
            assert_eq!(
                processor.to_camel_case(),
                expected,
                "Failed for input: '{}'",
                input
            );
        }
    }

    #[test]
    fn test_conversions() {
        let processor = CaseFormatter::from("brewCoffee");

        assert_eq!(processor.to_upper_case(), "BREW COFFEE");
        assert_eq!(processor.to_lower_case(), "brew coffee");
        assert_eq!(processor.to_title_case(), "Brew Coffee");
        assert_eq!(processor.to_dot_case(), "brew.coffee");
        assert_eq!(processor.to_snake_case(), "brew_coffee");
        assert_eq!(processor.to_kebab_case(), "brew-coffee");
        assert_eq!(processor.to_pascal_case(), "BrewCoffee");
        assert_eq!(processor.to_camel_case(), "brewCoffee");
    }
}

/// Convert to camelCase format (brewCoffee)
///
/// # Examples
///
/// ```
/// # use just_fmt::camel_case;
/// assert_eq!(camel_case!("brew_coffee"), "brewCoffee");
/// ```
#[macro_export]
macro_rules! camel_case {
    ($input:expr) => {{
        use just_fmt::fmt_case_style::CaseFormatter;
        CaseFormatter::from($input).to_camel_case()
    }};
}

/// Convert to UPPER CASE format (BREW COFFEE)
///
/// # Examples
///
/// ```
/// # use just_fmt::upper_case;
/// assert_eq!(upper_case!("brew coffee"), "BREW COFFEE");
/// ```
#[macro_export]
macro_rules! upper_case {
    ($input:expr) => {{
        use just_fmt::fmt_case_style::CaseFormatter;
        CaseFormatter::from($input).to_upper_case()
    }};
}

/// Convert to lower case format (brew coffee)
///
/// # Examples
///
/// ```
/// # use just_fmt::lower_case;
/// assert_eq!(lower_case!("BREW COFFEE"), "brew coffee");
/// ```
#[macro_export]
macro_rules! lower_case {
    ($input:expr) => {{
        use just_fmt::fmt_case_style::CaseFormatter;
        CaseFormatter::from($input).to_lower_case()
    }};
}

/// Convert to Title Case format (Brew Coffee)
///
/// # Examples
///
/// ```
/// # use just_fmt::title_case;
/// assert_eq!(title_case!("brew_coffee"), "Brew Coffee");
/// ```
#[macro_export]
macro_rules! title_case {
    ($input:expr) => {{
        use just_fmt::fmt_case_style::CaseFormatter;
        CaseFormatter::from($input).to_title_case()
    }};
}

/// Convert to dot.case format (brew.coffee)
///
/// # Examples
///
/// ```
/// # use just_fmt::dot_case;
/// assert_eq!(dot_case!("brew_coffee"), "brew.coffee");
/// ```
#[macro_export]
macro_rules! dot_case {
    ($input:expr) => {{
        use just_fmt::fmt_case_style::CaseFormatter;
        CaseFormatter::from($input).to_dot_case()
    }};
}

/// Convert to snake_case format (brew_coffee)
///
/// # Examples
///
/// ```
/// # use just_fmt::snake_case;
/// assert_eq!(snake_case!("brewCoffee"), "brew_coffee");
/// ```
#[macro_export]
macro_rules! snake_case {
    ($input:expr) => {{
        use just_fmt::fmt_case_style::CaseFormatter;
        CaseFormatter::from($input).to_snake_case()
    }};
}

/// Convert to kebab-case format (brew-coffee)
///
/// # Examples
///
/// ```
/// # use just_fmt::kebab_case;
/// assert_eq!(kebab_case!("brew_coffee"), "brew-coffee");
/// ```
#[macro_export]
macro_rules! kebab_case {
    ($input:expr) => {{
        use just_fmt::fmt_case_style::CaseFormatter;
        CaseFormatter::from($input).to_kebab_case()
    }};
}

/// Convert to PascalCase format (BrewCoffee)
///
/// # Examples
///
/// ```
/// # use just_fmt::pascal_case;
/// assert_eq!(pascal_case!("brew_coffee"), "BrewCoffee");
/// ```
#[macro_export]
macro_rules! pascal_case {
    ($input:expr) => {{
        use just_fmt::fmt_case_style::CaseFormatter;
        CaseFormatter::from($input).to_pascal_case()
    }};
}
