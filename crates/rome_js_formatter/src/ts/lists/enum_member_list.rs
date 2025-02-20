use crate::formatter::TrailingSeparator;
use crate::{
    join_elements, soft_line_break_or_space, token, Format, FormatElement, FormatResult, Formatter,
};
use rome_js_syntax::TsEnumMemberList;

impl Format for TsEnumMemberList {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(join_elements(
            soft_line_break_or_space(),
            formatter.format_separated(self, || token(","), TrailingSeparator::default())?,
        ))
    }
}
