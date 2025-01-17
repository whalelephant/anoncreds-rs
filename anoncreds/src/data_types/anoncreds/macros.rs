#[macro_export]
macro_rules! impl_anoncreds_object_identifier {
    ($i:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, Default)]
        pub struct $i(pub String);

        impl $i {
            pub fn new_unchecked(s: impl Into<String>) -> Self {
                Self(s.into())
            }

            pub fn new(s: impl Into<String>) -> Result<Self, crate::data_types::ValidationError> {
                let s = Self(s.into());
                s.validate()?;
                Ok(s)
            }
        }

        impl crate::data_types::Validatable for $i {
            fn validate(&self) -> Result<(), crate::data_types::ValidationError> {
                // TODO: stricten the URI regex.
                // Right now everything after the first colon is allowed, we might want to restrict
                // this
                let uri_regex = regex::Regex::new(r"^[a-zA-Z0-9\+\-\.]+:.+$").unwrap();
                uri_regex
                    .captures(&self.0)
                    .ok_or_else(|| {
                        indy_utils::invalid!(
                            "type: {}, identifier: {} is invalid. It MUST be a URI.",
                            stringify!($i),
                            self.0
                        )
                    })
                    .map(|_| ())
            }
        }

        impl Into<String> for $i {
            fn into(self) -> String {
                self.0
            }
        }

        // TODO: replace these with TryInto
        impl From<String> for $i {
            fn from(value: String) -> Self {
                $i::new_unchecked(value)
            }
        }

        // TODO: replace these with TryInto
        impl From<&str> for $i {
            fn from(value: &str) -> Self {
                $i::new_unchecked(value.to_owned())
            }
        }

        impl std::fmt::Display for $i {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}
