use crate::utils::format_with_semicolon;
use crate::{
    format_elements, format_traits::FormatOptional, space_token, Format, FormatElement, FormatNode,
    FormatResult, Formatter,
};
use rome_js_syntax::{TsPropertySignatureClassMember, TsPropertySignatureClassMemberFields};

impl FormatNode for TsPropertySignatureClassMember {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsPropertySignatureClassMemberFields {
            modifiers,
            name,
            property_annotation,
            semicolon_token,
        } = self.as_fields();

        let property_annotation = property_annotation.format_or_empty(formatter)?;

        format_with_semicolon(
            formatter,
            format_elements![
                modifiers.format(formatter)?,
                space_token(),
                name.format(formatter)?,
                property_annotation,
            ],
            semicolon_token,
        )
    }
}
