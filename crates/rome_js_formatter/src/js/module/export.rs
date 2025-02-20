use crate::{
    format_elements, space_token, Format, FormatElement, FormatNode, FormatResult, Formatter,
};

use rome_js_syntax::JsExport;
use rome_js_syntax::JsExportFields;

impl FormatNode for JsExport {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsExportFields {
            export_token,
            export_clause,
        } = self.as_fields();

        let export_token = export_token.format(formatter)?;
        let export_clause = export_clause.format(formatter)?;
        Ok(format_elements![export_token, space_token(), export_clause])
    }
}
