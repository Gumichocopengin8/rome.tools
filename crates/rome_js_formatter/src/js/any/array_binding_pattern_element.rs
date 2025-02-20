//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, FormatElement, FormatResult, Formatter};
use rome_js_syntax::JsAnyArrayBindingPatternElement;
impl Format for JsAnyArrayBindingPatternElement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsArrayHole(node) => node.format(formatter),
            Self::JsAnyBindingPattern(node) => node.format(formatter),
            Self::JsBindingPatternWithDefault(node) => node.format(formatter),
            Self::JsArrayBindingPatternRestElement(node) => node.format(formatter),
        }
    }
}
