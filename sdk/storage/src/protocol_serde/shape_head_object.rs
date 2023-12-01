// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_head_object_http_error(_response_status: u16, _response_headers: &::aws_smithy_runtime_api::http::Headers, _response_body: &[u8]) -> std::result::Result<crate::operation::head_object::HeadObjectOutput, crate::operation::head_object::HeadObjectError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body).map_err(crate::operation::head_object::HeadObjectError::unhandled)?;
    let generic = generic_builder.build();
    Err(crate::operation::head_object::HeadObjectError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_head_object_http_response(_response_status: u16, _response_headers: &::aws_smithy_runtime_api::http::Headers, _response_body: &[u8]) -> std::result::Result<crate::operation::head_object::HeadObjectOutput, crate::operation::head_object::HeadObjectError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::head_object::builders::HeadObjectOutputBuilder::default();
        output = output.set_accept_ranges(
            crate::protocol_serde::shape_head_object_output::de_accept_ranges_header(_response_headers)
                                    .map_err(|_|crate::operation::head_object::HeadObjectError::unhandled("Failed to parse AcceptRanges from header `Accept-Ranges"))?
        );
        output = output.set_content_type(
            crate::protocol_serde::shape_head_object_output::de_content_type_header(_response_headers)
                                    .map_err(|_|crate::operation::head_object::HeadObjectError::unhandled("Failed to parse ContentType from header `Content-Type"))?
        );
        output = output.set_e_tag(
            crate::protocol_serde::shape_head_object_output::de_e_tag_header(_response_headers)
                                    .map_err(|_|crate::operation::head_object::HeadObjectError::unhandled("Failed to parse ETag from header `ETag"))?
        );
        output = output.set_last_modified(
            crate::protocol_serde::shape_head_object_output::de_last_modified_header(_response_headers)
                                    .map_err(|_|crate::operation::head_object::HeadObjectError::unhandled("Failed to parse LastModified from header `Last-Modified"))?
        );
        output = output.set_x_amz_expiration(
            crate::protocol_serde::shape_head_object_output::de_x_amz_expiration_header(_response_headers)
                                    .map_err(|_|crate::operation::head_object::HeadObjectError::unhandled("Failed to parse XAmzExpiration from header `x-amz-expiration"))?
        );
        output = output.set_x_amz_mp_parts_count(
            crate::protocol_serde::shape_head_object_output::de_x_amz_mp_parts_count_header(_response_headers)
                                    .map_err(|_|crate::operation::head_object::HeadObjectError::unhandled("Failed to parse XAmzMpPartsCount from header `x-amz-mp-parts-count"))?
        );
        output = output.set_x_amz_server_side_encryption(
            crate::protocol_serde::shape_head_object_output::de_x_amz_server_side_encryption_header(_response_headers)
                                    .map_err(|_|crate::operation::head_object::HeadObjectError::unhandled("Failed to parse XAmzServerSideEncryption from header `x-amz-server-side-encryption"))?
        );
        output = output.set_x_amz_version_id(
            crate::protocol_serde::shape_head_object_output::de_x_amz_version_id_header(_response_headers)
                                    .map_err(|_|crate::operation::head_object::HeadObjectError::unhandled("Failed to parse XAmzVersionId from header `x-amz-version-id"))?
        );
        output.build()
    })
}

pub fn ser_head_object_headers(
                    input: &crate::operation::head_object::HeadObjectInput,
                    mut builder: ::http::request::Builder
                ) -> std::result::Result<::http::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
    if let ::std::option::Option::Some(inner_1) = &input.consistency_control {
        let formatted_2 = inner_1.as_str();
                        if !formatted_2.is_empty() {
                            let header_value = formatted_2;
                            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                                ::aws_smithy_types::error::operation::BuildError::invalid_field("consistency_control", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("Consistency-Control", header_value);
                        }
    }
    if let ::std::option::Option::Some(inner_3) = &input.x_amz_server_side_encryption_customer_algorithm {
        let formatted_4 = inner_3.as_str();
                        if !formatted_4.is_empty() {
                            let header_value = formatted_4;
                            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                                ::aws_smithy_types::error::operation::BuildError::invalid_field("x_amz_server_side_encryption_customer_algorithm", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-server-side-encryption-customer-algorithm", header_value);
                        }
    }
    if let ::std::option::Option::Some(inner_5) = &input.x_amz_server_side_encryption_customer_key {
        let formatted_6 = inner_5.as_str();
                        if !formatted_6.is_empty() {
                            let header_value = formatted_6;
                            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                                ::aws_smithy_types::error::operation::BuildError::invalid_field("x_amz_server_side_encryption_customer_key", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-server-side-encryption-customer-key", header_value);
                        }
    }
    if let ::std::option::Option::Some(inner_7) = &input.x_amz_server_side_encryption_customer_key_md5 {
        let formatted_8 = inner_7.as_str();
                        if !formatted_8.is_empty() {
                            let header_value = formatted_8;
                            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                                ::aws_smithy_types::error::operation::BuildError::invalid_field("x_amz_server_side_encryption_customer_key_md5", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-server-side-encryption-customer-key-MD5", header_value);
                        }
    }
    Ok(builder)
}

