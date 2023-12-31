// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_request_expiration(input: &crate::types::RequestExpiration, writer: ::aws_smithy_xml::encode::ElWriter) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.date {
        let mut inner_writer = scope.start_el("Date").finish();
        inner_writer.data(
            var_1.fmt(::aws_smithy_types::date_time::Format::DateTimeWithOffset)?.as_ref()
        );
    }
    if let Some(var_2) = &input.days {
        let mut inner_writer = scope.start_el("Days").finish();
        inner_writer.data(
            ::aws_smithy_types::primitive::Encoder::from(*var_2).encode()
        );
    }
    scope.finish();
    Ok(())
}

