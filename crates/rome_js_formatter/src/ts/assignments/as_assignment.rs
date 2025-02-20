use crate::{
    format_elements, space_token, Format, FormatElement, FormatNode, FormatResult, Formatter,
};
use rome_js_syntax::TsAsAssignment;
use rome_js_syntax::TsAsAssignmentFields;

impl FormatNode for TsAsAssignment {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsAsAssignmentFields {
            assignment,
            as_token,
            ty,
        } = self.as_fields();

        Ok(format_elements![
            assignment.format(formatter)?,
            space_token(),
            as_token.format(formatter)?,
            space_token(),
            ty.format(formatter)?,
        ])
    }
}
