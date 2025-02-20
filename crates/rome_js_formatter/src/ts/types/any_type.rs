use crate::{Format, FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::TsAnyType;

impl FormatNode for TsAnyType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.any_token().format(formatter)
    }
}
