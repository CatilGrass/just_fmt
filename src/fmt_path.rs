use std::path::{Path, PathBuf};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct PathFormatConfig {
    /// Whether to strip ANSI escape sequences (e.g., `\x1b[31m`, `\x1b[0m`).
    /// When the path string may contain terminal color codes, enabling this option will clean them up.
    #[cfg(feature = "strip-ansi")]
    pub strip_ansi: bool,

    /// Whether to strip characters disallowed in Windows filenames (`*`, `?`, `"`, `<`, `>`, `|`).
    /// These characters typically have special meaning or are not allowed in filesystems.
    pub strip_unfriendly_chars: bool,

    /// Whether to resolve parent directory references (`..`).
    /// When enabled, attempts to navigate upward in the path, e.g., `/a/b/../c` becomes `/a/c`.
    /// Note: This operation is based solely on the path string itself, without accessing the actual filesystem.
    pub resolve_parent_dirs: bool,

    /// Whether to collapse consecutive forward slashes (`/`).
    /// For example, `/home//user` becomes `/home/user`.
    pub collapse_consecutive_slashes: bool,

    /// Whether to escape backslashes (`\`) to forward slashes (`/`).
    /// This helps unify Windows‑style paths to Unix style, facilitating cross‑platform handling.
    pub escape_backslashes: bool,
}

impl Default for PathFormatConfig {
    fn default() -> Self {
        Self {
            #[cfg(feature = "strip-ansi")]
            strip_ansi: true,
            strip_unfriendly_chars: true,
            resolve_parent_dirs: true,
            collapse_consecutive_slashes: true,
            escape_backslashes: true,
        }
    }
}

/// Normalize an input path string into a canonical, platform‑agnostic form.
///
/// This function removes ANSI escape sequences, unifies separators to `/`,
/// collapses duplicate slashes, strips unfriendly characters (`*`, `?`, `"`, `<`, `>`, `|`),
/// resolves simple `..` components, and preserves a trailing slash when present.
///
/// See examples below for the exact normalization behavior.
///
/// # Examples
///
/// ```
/// # use just_fmt::fmt_path::fmt_path_str;
/// # use just_fmt::fmt_path::FormatPathError;
///
/// # fn main() -> Result<(), FormatPathError> {
/// assert_eq!(fmt_path_str("C:\\Users\\\\test")?, "C:/Users/test");
/// assert_eq!(
///     fmt_path_str("/path/with/*unfriendly?chars")?,
///     "/path/with/unfriendlychars"
/// );
/// assert_eq!(fmt_path_str("\x1b[31m/path\x1b[0m")?, "/path");
/// assert_eq!(fmt_path_str("/home/user/dir/")?, "/home/user/dir/");
/// assert_eq!(
///     fmt_path_str("/home/user/file.txt")?,
///     "/home/user/file.txt"
/// );
/// assert_eq!(
///     fmt_path_str("/home/my_user/DOCS/JVCS_TEST/Workspace/../Vault/")?,
///     "/home/my_user/DOCS/JVCS_TEST/Vault/"
/// );
/// assert_eq!(fmt_path_str("./home/file.txt")?, "home/file.txt");
/// assert_eq!(fmt_path_str("./home/path/")?, "home/path/");
/// assert_eq!(fmt_path_str("./")?, "");
/// # Ok(())
/// # }
/// ```
pub fn fmt_path_str(path: impl Into<String>) -> Result<String, PathFormatError> {
    fmt_path_str_custom(path, &PathFormatConfig::default())
}

/// Normalize an input path string into a canonical, platform‑agnostic form.
///
/// This function removes ANSI escape sequences, unifies separators to `/`,
/// collapses duplicate slashes, strips unfriendly characters (`*`, `?`, `"`, `<`, `>`, `|`),
/// resolves simple `..` components, and preserves a trailing slash when present.
///
/// Unlike `fmt_path_str`,
/// this method uses `PathFormatConfig` to precisely control
/// what should be processed
pub fn fmt_path_str_custom(
    path: impl Into<String>,
    config: &PathFormatConfig,
) -> Result<String, PathFormatError> {
    let path_result = path.into();
    let ends_with_slash = path_result.ends_with('/');

    // ANSI Strip
    #[cfg(feature = "strip-ansi")]
    let path_result = if config.strip_ansi {
        let cleaned = strip_ansi_escapes::strip(&path_result);
        String::from_utf8(cleaned).map_err(PathFormatError::InvalidUtf8)?
    } else {
        path_result
    };

    let path_result = if config.escape_backslashes {
        path_result.replace('\\', "/")
    } else {
        path_result
    };
    let mut result = String::new();
    let mut prev_char = '\0';

    for c in path_result.chars() {
        if config.collapse_consecutive_slashes && c == '/' && prev_char == '/' {
            continue;
        }
        result.push(c);
        prev_char = c;
    }

    if config.strip_unfriendly_chars {
        let unfriendly_chars = ['*', '?', '"', '<', '>', '|'];
        result = result
            .chars()
            .filter(|c| !unfriendly_chars.contains(c))
            .collect();
    }

    // Handle ".." path components
    let path_buf = PathBuf::from(&result);
    let normalized_path = if config.resolve_parent_dirs {
        normalize_path(&path_buf)
    } else {
        path_buf
    };
    result = normalized_path.to_string_lossy().replace('\\', "/");

    // Restore trailing slash if original path had one
    if ends_with_slash && !result.ends_with('/') {
        result.push('/');
    }

    // Special case: when result is only "./", return ""
    if result == "./" {
        return Ok(String::new());
    }

    Ok(result)
}

/// Normalize path by resolving ".." components without requiring file system access
fn normalize_path(path: &Path) -> PathBuf {
    let mut components = Vec::new();

    for component in path.components() {
        match component {
            std::path::Component::ParentDir => {
                if !components.is_empty() {
                    components.pop();
                }
            }
            std::path::Component::CurDir => {
                // Skip current directory components
            }
            _ => {
                components.push(component);
            }
        }
    }

    if components.is_empty() {
        PathBuf::from(".")
    } else {
        components.iter().collect()
    }
}

/// Format a [`PathBuf`] into its canonical string form and convert it back.
///
/// This is a convenience wrapper around [`fmt_path_str`], preserving
/// the semantics of [`PathBuf`] while applying the same normalization rules:
/// - normalize separators to `/`
/// - remove duplicated separators
/// - strip ANSI escape sequences
/// - remove unfriendly characters (`*`, `?`, etc.)
/// - resolve simple `..` segments
pub fn fmt_path(path: impl Into<PathBuf>) -> Result<PathBuf, PathFormatError> {
    let path_str = fmt_path_str(path.into().display().to_string())?;
    Ok(PathBuf::from(path_str))
}

/// Format a [`PathBuf`] into its canonical string form and convert it back.
///
/// Unlike `fmt_path`,
/// this method uses `PathFormatConfig` to precisely control
/// what should be processed
pub fn fmt_path_custom(
    path: impl Into<PathBuf>,
    config: &PathFormatConfig,
) -> Result<PathBuf, PathFormatError> {
    let path_str = fmt_path_str_custom(path.into().display().to_string(), config)?;
    Ok(PathBuf::from(path_str))
}

/// Error type for path formatting operations.
#[derive(Debug)]
pub enum PathFormatError {
    /// The input string contained invalid UTF-8 after stripping ANSI escape sequences.
    InvalidUtf8(std::string::FromUtf8Error),
}

impl std::fmt::Display for PathFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PathFormatError::InvalidUtf8(e) => {
                write!(f, "Invalid UTF-8 after ANSI stripping: {}", e)
            }
        }
    }
}

impl std::error::Error for PathFormatError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            PathFormatError::InvalidUtf8(e) => Some(e),
        }
    }
}

impl From<std::string::FromUtf8Error> for PathFormatError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        PathFormatError::InvalidUtf8(e)
    }
}
