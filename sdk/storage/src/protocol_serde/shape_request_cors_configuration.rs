// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_request_cors_configuration(input: &crate::types::RequestCorsConfiguration, writer: ::aws_smithy_xml::encode::ElWriter) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
     {
        for list_item_1 in &input.list_of_request_cors_rule {
             {
                let inner_writer = scope.start_el("CORSRule");
                crate::protocol_serde::shape_request_cors_rule::ser_request_cors_rule(list_item_1, inner_writer)?
            }
        }
    }
    scope.finish();
    Ok(())
}
