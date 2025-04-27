use serde_json::Value;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Json(serde_json::Error),
    MissingDefaultTemplate,
    UnknownTemplate(String),
    UnknownStaticProperty(String),
    InvalidStaticProperty {
        prop: String,
        value: Value,
    },
    ExpectedObjectToSetProperty {
        path: String,
        prop: String,
    },
    ExpectedObjectToRemoveProperty {
        path: String,
        prop: String,
    },
    UnknownPropertyInObjectPath {
        path: String,
        prop: String,
    },
    IndexOutOfBounds {
        index: usize,
        len: usize,
        path: String,
    },
    ExpectedArrayToSetIndex {
        path: String,
        index: usize,
    },
    ExpectedArrayToRemoveIndex {
        path: String,
        index: usize,
    },
    ExpectedWildcardToSetProperty {
        path: String,
        value: Value,
    },
    ExpectedWildcardToRemoveProperty {
        path: String,
        value: Value,
    },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(err) => err.fmt(f),
            Error::Json(err) => err.fmt(f),
            Error::MissingDefaultTemplate => write!(f, "Missing default template"),
            Error::UnknownTemplate(alias) => write!(f, "Unknown template alias: {alias}"),
            Error::UnknownStaticProperty(prop) => write!(f, "Missing static property: {prop}"),
            Error::InvalidStaticProperty { prop, value } =>
                write!(f, "Invalid static property: {prop} with value: {value}"),
            Error::ExpectedObjectToSetProperty { path, prop } =>
                write!(
                    f,
                    "Expected object to set property {prop} at path: {path}. Does the [path) exist?"
                ),
            Error::ExpectedObjectToRemoveProperty { path, prop } =>
                write!(f, "Expected object to remove property at path: {path}. Property: {prop}"),
            Error::UnknownPropertyInObjectPath { path, prop } =>
                write!(f, "Unknown property in object path: {path}. Property: {prop}"),
            Error::IndexOutOfBounds { index, len, path } =>
                write!(
                    f,
                    "Attempted to access index out of bounds. Path: {path}. Index: {index}, Length: {len}"
                ),
            Error::ExpectedArrayToSetIndex { path, index } =>
                write!(f, "Expected array to set property at path: {path}. Index: {index}"),
            Error::ExpectedArrayToRemoveIndex { path, index } =>
                write!(f, "Expected array to remove property at path: {path}. Index: {index}"),
            Error::ExpectedWildcardToSetProperty { path, value } =>
                write!(f, "Expected wildcard to set property at path: {path}. Value: {value}"),
            Error::ExpectedWildcardToRemoveProperty { path, value } =>
                write!(f, "Expected wildcard to remove property at path: {path}. Value: {value}"),
        }
    }
}

impl std::error::Error for Error {}
