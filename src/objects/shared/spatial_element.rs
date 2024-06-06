use std::{fmt::Display, ops::Deref};

use crate::parser::{comma::Comma, label::Label, optional::OptionalParameter, IFCParse, IFCParser};

use super::product::Product;

/// A spatial element is the generalization of all spatial elements that
/// might be used to define a spatial structure or to define spatial zones.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifcproductextension/lexical/ifcspatialelement.htm
pub struct SpatialElement {
    product: Product,

    /// Long name for a spatial structure element, used for informal purposes.
    /// It should be used, if available, in conjunction with the inherited
    /// Name attribute.
    pub long_name: OptionalParameter<Label>,
}

impl Deref for SpatialElement {
    type Target = Product;

    fn deref(&self) -> &Self::Target {
        &self.product
    }
}

impl IFCParse for SpatialElement {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                product: Product::parse(),
                _: Comma::parse(),
                long_name: OptionalParameter::parse(),
            }
        }
    }
}

impl Display for SpatialElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.product, self.long_name,)
    }
}