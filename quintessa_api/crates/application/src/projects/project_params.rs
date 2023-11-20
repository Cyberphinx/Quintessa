use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ProjectParams {
    pub category: Option<String>,
}

// This implementation provides default values when the trait is used as ListingParams::default();
impl Default for ProjectParams {
    fn default() -> Self {
        Self {
            category: Some("Architecture".to_owned()),
        }
    }
}
