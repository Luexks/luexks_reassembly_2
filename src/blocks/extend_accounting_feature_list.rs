use crate::blocks::feature::Feature;
use crate::utility::flags::Flags;

#[derive(Clone)]
pub struct ExtendAccountingFeatureList {
    pub features: Flags<Feature>,
    pub feature_list_same_as_extends: bool,
}
