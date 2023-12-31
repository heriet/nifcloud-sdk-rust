// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_request_part(input: &crate::types::RequestPart, writer: ::aws_smithy_xml::encode::ElWriter) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
     {
        let mut inner_writer = scope.start_el("ETag").finish();
        inner_writer.data(
            input.e_tag.as_str()
        );
    }
     {
        let mut inner_writer = scope.start_el("PartNumber").finish();
        inner_writer.data(
            ::aws_smithy_types::primitive::Encoder::from(input.part_number).encode()
        );
    }
    scope.finish();
    Ok(())
}

