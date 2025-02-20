use crate::{Format, FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::TsNonPrimitiveType;

impl FormatNode for TsNonPrimitiveType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.object_token().format(formatter)
    }
}
