//! Canonical case mappings for SVG and MathML attributes.
//!
//! `tl` lowercases all attribute names when it re-parses a wrapped HTML
//! fragment (e.g. `<html><body><svg …></body></html>`).  Tier-1's
//! `emit_svg_from_slice` takes that path; Tier-2 parses the full document
//! once and preserves original case.  This table restores the WHATWG
//! canonical spelling so both tiers produce byte-identical output.
//!
//! Only attributes with mixed-case canonical names are listed here; purely
//! lowercase attributes (e.g. `fill`, `stroke`, `d`, `cx`) need no mapping.

/// Pairs of `(lowercase_key, canonical_name)` for SVG and MathML attributes
/// whose WHATWG-specified spelling is not already all-lowercase.
///
/// The array is sorted by the lowercase key so a binary-search lookup is
/// possible, but a linear scan over ~60 entries is also fine at call-site.
pub const SVG_CAMEL_ATTRS: &[(&str, &str)] = &[
    ("attributename", "attributeName"),
    ("attributetype", "attributeType"),
    ("basefrequency", "baseFrequency"),
    ("calcmode", "calcMode"),
    ("clippath", "clipPath"),
    ("clippathunits", "clipPathUnits"),
    ("diffuseconstant", "diffuseConstant"),
    ("edgemode", "edgeMode"),
    ("filterunits", "filterUnits"),
    ("gradienttransform", "gradientTransform"),
    ("gradientunits", "gradientUnits"),
    ("kernelmatrix", "kernelMatrix"),
    ("kernelunitlength", "kernelUnitLength"),
    ("keypoints", "keyPoints"),
    ("keysplines", "keySplines"),
    ("keytimes", "keyTimes"),
    ("lengthadjust", "lengthAdjust"),
    ("limitingconeangle", "limitingConeAngle"),
    ("markerheight", "markerHeight"),
    ("markerunits", "markerUnits"),
    ("markerwidth", "markerWidth"),
    ("maskunits", "maskUnits"),
    ("maskcontentunits", "maskContentUnits"),
    ("numoctaves", "numOctaves"),
    ("pathlength", "pathLength"),
    ("patterncontentunits", "patternContentUnits"),
    ("patterntransform", "patternTransform"),
    ("patternunits", "patternUnits"),
    ("pointsatx", "pointsAtX"),
    ("pointsaty", "pointsAtY"),
    ("pointsatz", "pointsAtZ"),
    ("preserveaspectratio", "preserveAspectRatio"),
    ("primitiveunits", "primitiveUnits"),
    ("refx", "refX"),
    ("refy", "refY"),
    ("repeatcount", "repeatCount"),
    ("repeatdur", "repeatDur"),
    ("specularconstant", "specularConstant"),
    ("specularexponent", "specularExponent"),
    ("spreadmethod", "spreadMethod"),
    ("startoffset", "startOffset"),
    ("stddeviation", "stdDeviation"),
    ("stitchtiles", "stitchTiles"),
    ("surfacescale", "surfaceScale"),
    ("systemlanguage", "systemLanguage"),
    ("tablevalues", "tableValues"),
    ("targetx", "targetX"),
    ("targety", "targetY"),
    ("textlength", "textLength"),
    ("viewbox", "viewBox"),
    ("xchannelselector", "xChannelSelector"),
    ("ychannelselector", "yChannelSelector"),
    ("zoomandpan", "zoomAndPan"),
];

/// Return the canonical WHATWG spelling for `lowercase_key`, or `None` if
/// the attribute is already all-lowercase (no substitution needed).
#[inline]
pub fn canonical_svg_attr(lowercase_key: &str) -> Option<&'static str> {
    SVG_CAMEL_ATTRS
        .iter()
        .find(|(lc, _)| *lc == lowercase_key)
        .map(|(_, canonical)| *canonical)
}
