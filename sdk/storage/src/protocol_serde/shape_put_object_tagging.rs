// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_put_object_tagging_http_error(_response_status: u16, _response_headers: &::aws_smithy_runtime_api::http::Headers, _response_body: &[u8]) -> std::result::Result<crate::operation::put_object_tagging::PutObjectTaggingOutput, crate::operation::put_object_tagging::PutObjectTaggingError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body).map_err(crate::operation::put_object_tagging::PutObjectTaggingError::unhandled)?;
    let generic = generic_builder.build();
    Err(crate::operation::put_object_tagging::PutObjectTaggingError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_object_tagging_http_response(_response_status: u16, _response_headers: &::aws_smithy_runtime_api::http::Headers, _response_body: &[u8]) -> std::result::Result<crate::operation::put_object_tagging::PutObjectTaggingOutput, crate::operation::put_object_tagging::PutObjectTaggingError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::put_object_tagging::builders::PutObjectTaggingOutputBuilder::default();
        output = output.set_x_amz_version_id(
            crate::protocol_serde::shape_put_object_tagging_output::de_x_amz_version_id_header(_response_headers)
                                    .map_err(|_|crate::operation::put_object_tagging::PutObjectTaggingError::unhandled("Failed to parse XAmzVersionId from header `x-amz-version-id"))?
        );
        output.build()
    })
}

pub fn ser_put_object_tagging_headers(
                    input: &crate::operation::put_object_tagging::PutObjectTaggingInput,
                    mut builder: ::http::request::Builder
                ) -> std::result::Result<::http::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
    if let ::std::option::Option::Some(inner_1) = &input.content_md5 {
        let formatted_2 = inner_1.as_str();
                        if !formatted_2.is_empty() {
                            let header_value = formatted_2;
                            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                                ::aws_smithy_types::error::operation::BuildError::invalid_field("content_md5", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("Content-MD5", header_value);
                        }
    }
    Ok(builder)
}

