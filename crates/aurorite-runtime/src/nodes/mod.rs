mod dice;

macro_rules! try_extract {
    ($source:expr, $t:path, $name:literal) => {
        if let Some($t(value)) = $source.get($name) {
            value
        } else {
            return Err(vismut_core::ScriptError::MissingInput($name.to_string()));
        }
    };
}

pub(crate) use try_extract;