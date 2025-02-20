use crate::format_traits::FormatOptional;

use crate::utils::format_with_semicolon;
use crate::{
    format_elements, space_token, Format, FormatElement, FormatNode, FormatResult, Formatter,
};

use rome_js_syntax::JsExportNamedClause;
use rome_js_syntax::JsExportNamedClauseFields;

impl FormatNode for JsExportNamedClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsExportNamedClauseFields {
            type_token,
            l_curly_token,
            specifiers,
            r_curly_token,
            semicolon_token,
        } = self.as_fields();

        let type_token = type_token
            .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?;

        let specifiers = specifiers.format(formatter)?;

        let list = formatter.format_delimited_soft_block_spaces(
            &l_curly_token?,
            specifiers,
            &r_curly_token?,
        )?;

        format_with_semicolon(
            formatter,
            format_elements![type_token, list],
            semicolon_token,
        )
    }
}
