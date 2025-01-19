use crate::utility::flags::Flags;
use crate::blocks::feature::Feature;

#[derive(Clone)]
pub struct ExtendAccountingFeatureList {
    pub features: Flags<Feature>,
    pub feature_list_same_as_extends: bool,
}