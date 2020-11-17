// This file is auto generated by `cg` <https://github.com/teloxide/cg> (e634f65).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

use crate::types::{PassportElementError, True};

impl_payload! {
    /// Informs a user that some of the Telegram Passport elements they provided contains errors. The user will not be able to re-submit their Passport to you until the errors are fixed (the contents of the field for which you returned the error must change). Returns _True_ on success.
    ///
    /// Use this if the data submitted by the user doesn't satisfy the standards your service requires for any reason. For example, if a birthday date seems invalid, a submitted document is blurry, a scan shows evidence of tampering, etc. Supply some details in the error message to make sure the user knows how to correct the issues.
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub SetPassportDataErrors (SetPassportDataErrorsSetters) => True {
        required {
            /// User identifier
            pub user_id: i32,
            /// A JSON-serialized array describing the errors
            pub errors: Vec<PassportElementError> [collect],
        }
    }
}