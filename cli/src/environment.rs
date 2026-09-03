use std::path::PathBuf;

use bon::Builder;

#[derive(Debug, Clone, Eq, PartialEq, Builder)]
pub struct Environment {
    #[builder(into)]
    db_path: Option<PathBuf>,
}
