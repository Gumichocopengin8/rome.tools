use crate::formatter::TrailingSeparator;
use crate::{
    join_elements, soft_line_break_or_space, token, Format, FormatElement, FormatResult, Formatter,
};
use rome_js_syntax::{JsAnyObjectBindingPatternMember, JsObjectBindingPatternPropertyList};

impl Format for JsObjectBindingPatternPropertyList {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        // The trailing separator is disallowed after a rest element
        let has_trailing_rest = match self.into_iter().last() {
            Some(elem) => matches!(
                elem?,
                JsAnyObjectBindingPatternMember::JsObjectBindingPatternRest(_)
            ),
            None => false,
        };

        let trailing_separator = if has_trailing_rest {
            TrailingSeparator::Disallowed
        } else {
            TrailingSeparator::Allowed
        };

        Ok(join_elements(
            soft_line_break_or_space(),
            formatter.format_separated(self, || token(","), trailing_separator)?,
        ))
    }
}
