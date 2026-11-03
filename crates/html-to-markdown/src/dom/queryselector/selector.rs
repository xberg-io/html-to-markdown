use std::borrow::Cow;

use crate::dom::{Bytes, Node};

/// A single query selector node
#[derive(Debug, Clone)]
pub enum Selector<'a> {
    /// Tag selector: foo
    Tag(&'a [u8]),
    /// ID selector: #foo
    Id(&'a [u8]),
    /// Class selector: .foo
    Class(&'a [u8]),
    /// All selector: *
    All,
    /// And combinator: .foo.bar
    And(Box<Self>, Box<Self>),
    /// Or combinator: .foo, .bar
    Or(Box<Self>, Box<Self>),
    /// Descendant combinator: .foo .bar
    Descendant(Box<Self>, Box<Self>),
    /// Parent combinator: .foo > .bar
    Parent(Box<Self>, Box<Self>),
    /// Attribute: \[foo\]
    Attribute(&'a [u8]),
    /// Attribute with value: [foo=bar]
    AttributeValue(&'a [u8], &'a [u8]),
    /// Attribute with whitespace-separated list of values that contains a value: [foo~=bar]
    AttributeValueWhitespacedContains(&'a [u8], &'a [u8]),
    /// Attribute with value that starts with: [foo^=bar]
    AttributeValueStartsWith(&'a [u8], &'a [u8]),
    /// Attribute with value that ends with: [foo$=bar]
    AttributeValueEndsWith(&'a [u8], &'a [u8]),
    /// Attribute with value that contains: [foo*=bar]
    AttributeValueSubstring(&'a [u8], &'a [u8]),
}

impl Selector<'_> {
    /// Checks if the given node matches this selector
    #[must_use]
    pub fn matches(&self, node: &Node<'_>) -> bool {
        match self {
            Self::Tag(tag) => node.as_tag().is_some_and(|t| t.name().as_bytes().eq(*tag)),
            Self::Id(id) => node.as_tag().is_some_and(|t| {
                t.attributes()
                    .id()
                    .is_some_and(|value| decoded(value).as_bytes() == *id)
            }),
            Self::Class(class) => node.as_tag().is_some_and(|t| {
                t.attributes().class().is_some_and(|value| {
                    decoded(value)
                        .split_ascii_whitespace()
                        .any(|name| name.as_bytes() == *class)
                })
            }),
            Self::And(a, b) => a.matches(node) && b.matches(node),
            Self::Or(a, b) => a.matches(node) || b.matches(node),
            Self::All => true,
            Self::Attribute(attribute) => node.as_tag().is_some_and(|t| t.attributes().get(*attribute).is_some()),
            Self::AttributeValue(attribute, value) => {
                check_attribute(node, attribute, value, |attr, value| attr == value)
            }
            Self::AttributeValueEndsWith(attribute, value) => {
                check_attribute(node, attribute, value, |attr, value| attr.ends_with(value))
            }
            Self::AttributeValueStartsWith(attribute, value) => {
                check_attribute(node, attribute, value, |attr, value| attr.starts_with(value))
            }
            Self::AttributeValueSubstring(attribute, value) => {
                check_attribute(node, attribute, value, |attr, value| attr.contains(value))
            }
            Self::AttributeValueWhitespacedContains(attribute, value) => {
                check_attribute(node, attribute, value, |attr, value| {
                    attr.split_whitespace().any(|x| x == value)
                })
            }
            _ => false,
        }
    }
}

fn check_attribute<F>(node: &Node, attribute: &[u8], value: &[u8], callback: F) -> bool
where
    F: Fn(&str, &str) -> bool,
{
    node.as_tag().is_some_and(|t| {
        t.attributes()
            .get(attribute)
            .flatten()
            .is_some_and(|attr| callback(&decoded(attr), &String::from_utf8_lossy(value)))
    })
}

/// The value of an attribute as HTML reads it, with its character references decoded.
fn decoded<'v>(value: &'v Bytes<'_>) -> Cow<'v, str> {
    match value.as_utf8_str() {
        Cow::Borrowed(text) => crate::text::decode_attribute_value_cow(text),
        Cow::Owned(text) => Cow::Owned(crate::text::decode_attribute_value_cow(&text).into_owned()),
    }
}
