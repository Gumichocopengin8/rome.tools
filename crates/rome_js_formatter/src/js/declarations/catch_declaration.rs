use crate::format_traits::FormatOptional;

use crate::{format_elements, Format, FormatElement, FormatNode, FormatResult, Formatter};

use rome_js_syntax::JsCatchDeclaration;
use rome_js_syntax::JsCatchDeclarationFields;

impl FormatNode for JsCatchDeclaration {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsCatchDeclarationFields {
            l_paren_token,
            binding,
            r_paren_token,
            type_annotation,
        } = self.as_fields();

        formatter.format_delimited_soft_block_indent(
            &l_paren_token?,
            format_elements![
                binding.format(formatter)?,
                type_annotation.format_or_empty(formatter)?
            ],
            &r_paren_token?,
        )
    }
}
