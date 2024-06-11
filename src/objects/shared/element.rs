use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use crate::parser::{comma::Comma, label::Label, optional::OptionalParameter, IFCParse, IFCParser};

use super::product::Product;

/// An element is a generalization of all components that make up an AEC product.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifcproductextension/lexical/ifcelement.htm
pub struct Element {
    product: Product,

    /// The tag (or label) identifier at the particular instance of a product,
    /// e.g. the serial number, or the position number. It is the identifier
    /// at the occurrence level.
    pub tag: OptionalParameter<Label>,
}

impl Element {
    pub fn new(product: Product) -> Self {
        Self {
            product,
            tag: OptionalParameter::omitted(),
        }
    }
}

pub trait ElementBuilder: Sized {
    fn element_mut(&mut self) -> &mut Element;

    fn tag(mut self, tag: impl Into<Label>) -> Self {
        self.element_mut().tag = tag.into().into();
        self
    }
}

impl Deref for Element {
    type Target = Product;

    fn deref(&self) -> &Self::Target {
        &self.product
    }
}

impl DerefMut for Element {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.product
    }
}

impl IFCParse for Element {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                product: Product::parse(),
                _: Comma::parse(),
                tag: OptionalParameter::parse(),
            }
        }
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.product, self.tag)
    }
}
