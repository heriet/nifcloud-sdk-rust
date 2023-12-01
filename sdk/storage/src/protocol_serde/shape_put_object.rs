// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_put_object_http_error(_response_status: u16, _response_headers: &::aws_smithy_runtime_api::http::Headers, _response_body: &[u8]) -> std::result::Result<crate::operation::put_object::PutObjectOutput, crate::operation::put_object::PutObjectError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body).map_err(crate::operation::put_object::PutObjectError::unhandled)?;
    let generic = generic_builder.build();
    Err(crate::operation::put_object::PutObjectError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_object_http_response(_response_status: u16, _response_headers: &::aws_smithy_runtime_api::http::Headers, _response_body: &[u8]) -> std::result::Result<crate::operation::put_object::PutObjectOutput, crate::operation::put_object::PutObjectError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::put_object::builders::PutObjectOutputBuilder::default();
        output = output.set_e_tag(
            crate::protocol_serde::shape_put_object_output::de_e_tag_header(_response_headers)
                                    .map_err(|_|crate::operation::put_object::PutObjectError::unhandled("Failed to parse ETag from header `ETag"))?
        );
        output = output.set_x_amz_version_id(
            crate::protocol_serde::shape_put_object_output::de_x_amz_version_id_header(_response_headers)
                                    .map_err(|_|crate::operation::put_object::PutObjectError::unhandled("Failed to parse XAmzVersionId from header `x-amz-version-id"))?
        );
        output.build()
    })
}

pub fn ser_put_object_headers(
                    input: &crate::operation::put_object::PutObjectInput,
                    mut builder: ::http::request::Builder
                ) -> std::result::Result<::http::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
    if let ::std::option::Option::Some(inner_1) = &input.content_disposition {
        let formatted_2 = inner_1.as_str();
                        if !formatted_2.is_empty() {
                            let header_value = formatted_2;
                            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                                ::aws_smithy_types::error::operation::BuildError::invalid_field("content_disposition", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("Content-Disposition", header_value);
                        }
    }
    if let ::std::option::Option::Some(inner_3) = &input.content_encoding {
        let formatted_4 = inner_3.as_str();
                        if !formatted_4.is_empty() {
                            let header_value = formatted_4;
                            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                                ::aws_smithy_types::error::operation::BuildError::invalid_field("content_encoding", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("Content-Encoding", header_value);
                        }
    }
    if let ::std::option::Option::Some(inner_5) = &input.content_md5 {
        let formatted_6 = inner_5.as_str();
                        if !formatted_6.is_empty() {
                            let header_value = formatted_6;
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
    if let ::std::option::Option::Some(inner_7) = &input.content_type {
        let formatted_8 = inner_7.as_str();
                        if !formatted_8.is_empty() {
                            let header_value = formatted_8;
                            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                                ::aws_smithy_types::error::operation::BuildError::invalid_field("content_type", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("Content-Type", header_value);
                        }
    }
    if let ::std::option::Option::Some(inner_9) = &input.x_amz_meta {
        let formatted_10 = inner_9.as_str();
                        if !formatted_10.is_empty() {
                            let header_value = formatted_10;
                            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                                ::aws_smithy_types::error::operation::BuildError::invalid_field("x_amz_meta", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-meta-", header_value);
                        }
    }
    if let ::std::option::Option::Some(inner_11) = &input.x_amz_server_side_encryption {
        let formatted_12 = inner_11.as_str();
                        if !formatted_12.is_empty() {
                            let header_value = formatted_12;
                            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                                ::aws_smithy_types::error::operation::BuildError::invalid_field("x_amz_server_side_encryption", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-server-side-encryption", header_value);
                        }
    }
    if let ::std::option::Option::Some(inner_13) = &input.x_amz_server_side_encryption_customer_algorithm {
        let formatted_14 = inner_13.as_str();
                        if !formatted_14.is_empty() {
                            let header_value = formatted_14;
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
    if let ::std::option::Option::Some(inner_15) = &input.x_amz_server_side_encryption_customer_key {
        let formatted_16 = inner_15.as_str();
                        if !formatted_16.is_empty() {
                            let header_value = formatted_16;
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
    if let ::std::option::Option::Some(inner_17) = &input.x_amz_server_side_encryption_customer_key_md5 {
        let formatted_18 = inner_17.as_str();
                        if !formatted_18.is_empty() {
                            let header_value = formatted_18;
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
    if let ::std::option::Option::Some(inner_19) = &input.x_amz_storage_class {
        let formatted_20 = inner_19.as_str();
                        if !formatted_20.is_empty() {
                            let header_value = formatted_20;
                            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                                ::aws_smithy_types::error::operation::BuildError::invalid_field("x_amz_storage_class", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-storage-class", header_value);
                        }
    }
    if let ::std::option::Option::Some(inner_21) = &input.x_amz_tagging {
        let formatted_22 = inner_21.as_str();
                        if !formatted_22.is_empty() {
                            let header_value = formatted_22;
                            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                                ::aws_smithy_types::error::operation::BuildError::invalid_field("x_amz_tagging", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-tagging", header_value);
                        }
    }
    Ok(builder)
}
