use crate::{format_elements, Format, FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::TsMappedTypeOptionalModifierClause;
use rome_js_syntax::TsMappedTypeOptionalModifierClauseFields;

impl FormatNode for TsMappedTypeOptionalModifierClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsMappedTypeOptionalModifierClauseFields {
            operator_token,
            question_mark_token,
        } = self.as_fields();

        Ok(format_elements![
            operator_token.format(formatter)?,
            question_mark_token.format(formatter)?
        ])
    }
}
