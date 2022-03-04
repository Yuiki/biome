use crate::{concat_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::{ast::TsTemplateElementList, AstNodeList};

impl ToFormatElement for TsTemplateElementList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(concat_elements(formatter.format_nodes(self.iter())?))
    }
}