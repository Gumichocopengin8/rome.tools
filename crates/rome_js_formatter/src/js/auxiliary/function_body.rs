use crate::{format_elements, Format, FormatElement, FormatNode, FormatResult, Formatter};

use rome_js_syntax::JsFunctionBody;
use rome_js_syntax::JsFunctionBodyFields;

impl FormatNode for JsFunctionBody {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsFunctionBodyFields {
            l_curly_token,
            directives,
            statements,
            r_curly_token,
        } = self.as_fields();

        formatter.format_delimited_block_indent(
            &l_curly_token?,
            format_elements![
                directives.format(formatter)?,
                formatter.format_list(statements),
            ],
            &r_curly_token?,
        )
    }
}
