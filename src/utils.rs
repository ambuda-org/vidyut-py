/// Generates the following boilerplate methods:
/// - `__hash__`
/// - `__str__`
/// - `from_string`, which is the inverse of `__str__`
/// - `From<RustEnum> for PyEnum`
/// - `From<PyEnum> for RustEnum`
///
/// `__iter__` is not possible since it requires accessing the metaclass.
/// See https://github.com/PyO3/pyo3/issues/906 and related issues.
///
/// Requirements:
/// - Enum must derive `Hash`
/// - Rust enum must implement `FromStr`.
///
/// For enum compat, see: https://github.com/PyO3/pyo3/issues/2887
macro_rules! py_enum {
    ($Py:ident, $Rust:ident, [$( $variant:ident ),*]) => {
        impl From<$Rust> for $Py {
            fn from(val: $Rust) -> Self {
                match val {
                    $(
                        $Rust::$variant => $Py::$variant,
                    )*
                }
            }
        }

        impl From<$Py> for $Rust {
            fn from(val: $Py) -> Self {
                match val {
                    $(
                        $Py::$variant => $Rust::$variant,
                    )*
                }
            }
        }

        impl std::fmt::Display for $Py {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", $Rust::from(*self).as_str())
            }
        }

        #[pymethods]
        impl $Py {
            fn __hash__(&self) -> u64 {
                let mut hasher = DefaultHasher::new();
                self.hash(&mut hasher);
                hasher.finish()
            }

            fn __repr__(&self) -> String {
                format!("{}.{}", stringify!{$Rust}, self.name())
            }

            fn __str__(&self) -> String {
                let rust: $Rust = self.clone().into();
                rust.as_str().to_string()
            }

            /// The name used to define the `Enum` member.
            ///
            /// (Defined for compatibility with Python enums.)
            #[getter]
            fn name(&self) -> String {
                let ret = match self {
                    $(
                        $Py::$variant => stringify!($variant),
                    )*
                };
                ret.to_string()
            }

            /// Returns all options for this enum.
            #[staticmethod]
            fn choices() -> Vec<$Py> {
                const CHOICES: &[$Py] = &[
                    $(
                        $Py::$variant,
                    )*
                ];
                CHOICES.to_vec()
            }

            /// Create an enum value from the given string.
            ///
            /// This is the inverse of the `__str__` method.
            #[staticmethod]
            fn from_string(val: &str) -> PyResult<Self> {
                match $Rust::from_str(val) {
                    Ok(x) => Ok(x.into()),
                    Err(_) => Err(PyNotImplementedError::new_err("Could not parse {val}")),
                }
            }
        }
    }
}

pub(crate) use py_enum;
