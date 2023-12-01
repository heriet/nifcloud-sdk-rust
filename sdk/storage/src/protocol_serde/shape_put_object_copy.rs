// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_put_object_copy_http_error(_response_status: u16, _response_headers: &::aws_smithy_runtime_api::http::Headers, _response_body: &[u8]) -> std::result::Result<crate::operation::put_object_copy::PutObjectCopyOutput, crate::operation::put_object_copy::PutObjectCopyError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body).map_err(crate::operation::put_object_copy::PutObjectCopyError::unhandled)?;
    let generic = generic_builder.build();
    Err(crate::operation::put_object_copy::PutObjectCopyError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_object_copy_http_response(_response_status: u16, _response_headers: &::aws_smithy_runtime_api::http::Headers, _response_body: &[u8]) -> std::result::Result<crate::operation::put_object_copy::PutObjectCopyOutput, crate::operation::put_object_copy::PutObjectCopyError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::put_object_copy::builders::PutObjectCopyOutputBuilder::default();
        output = crate::protocol_serde::shape_put_object_copy::de_put_object_copy(_response_body, output).map_err(crate::operation::put_object_copy::PutObjectCopyError::unhandled)?;
        output = output.set_content_type(
            crate::protocol_serde::shape_put_object_copy_output::de_content_type_header(_response_headers)
                                    .map_err(|_|crate::operation::put_object_copy::PutObjectCopyError::unhandled("Failed to parse ContentType from header `Content-Type"))?
        );
        output.build()
    })
}

pub fn ser_put_object_copy_headers(
                    input: &crate::operation::put_object_copy::PutObjectCopyInput,
                    mut builder: ::http::request::Builder
                ) -> std::result::Result<::http::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
    if let ::std::option::Option::Some(inner_1) = &input.x_amz_copy_source {
        let formatted_2 = inner_1.as_str();
                        if !formatted_2.is_empty() {
                            let header_value = formatted_2;
                            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                                ::aws_smithy_types::error::operation::BuildError::invalid_field("x_amz_copy_source", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-copy-source", header_value);
                        }
    }
    if let ::std::option::Option::Some(inner_3) = &input.x_amz_copy_source_if_match {
        let formatted_4 = inner_3.as_str();
                        if !formatted_4.is_empty() {
                            let header_value = formatted_4;
                            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                                ::aws_smithy_types::error::operation::BuildError::invalid_field("x_amz_copy_source_if_match", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-copy-source-if-match", header_value);
                        }
    }
    if let ::std::option::Option::Some(inner_5) = &input.x_amz_copy_source_if_modified_since {
        let formatted_6 = inner_5.as_str();
                        if !formatted_6.is_empty() {
                            let header_value = formatted_6;
                            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                                ::aws_smithy_types::error::operation::BuildError::invalid_field("x_amz_copy_source_if_modified_since", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-copy-source-if-modified-since", header_value);
                        }
    }
    if let ::std::option::Option::Some(inner_7) = &input.x_amz_copy_source_if_none_match {
        let formatted_8 = inner_7.as_str();
                        if !formatted_8.is_empty() {
                            let header_value = formatted_8;
                            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                                ::aws_smithy_types::error::operation::BuildError::invalid_field("x_amz_copy_source_if_none_match", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-copy-source-if-none-match", header_value);
                        }
    }
    if let ::std::option::Option::Some(inner_9) = &input.x_amz_copy_source_if_unmodified_since {
        let formatted_10 = inner_9.as_str();
                        if !formatted_10.is_empty() {
                            let header_value = formatted_10;
                            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                                ::aws_smithy_types::error::operation::BuildError::invalid_field("x_amz_copy_source_if_unmodified_since", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-copy-source-if-unmodified-since", header_value);
                        }
    }
    if let ::std::option::Option::Some(inner_11) = &input.x_amz_copy_source_server_side_encryption_customer_algorithm {
        let formatted_12 = inner_11.as_str();
                        if !formatted_12.is_empty() {
                            let header_value = formatted_12;
                            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                                ::aws_smithy_types::error::operation::BuildError::invalid_field("x_amz_copy_source_server_side_encryption_customer_algorithm", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-copy-source-server-side-encryption-customer-algorithm", header_value);
                        }
    }
    if let ::std::option::Option::Some(inner_13) = &input.x_amz_copy_source_server_side_encryption_customer_key {
        let formatted_14 = inner_13.as_str();
                        if !formatted_14.is_empty() {
                            let header_value = formatted_14;
                            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                                ::aws_smithy_types::error::operation::BuildError::invalid_field("x_amz_copy_source_server_side_encryption_customer_key", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-copy-source-server-side-encryption-customer-key", header_value);
                        }
    }
    if let ::std::option::Option::Some(inner_15) = &input.x_amz_copy_source_server_side_encryption_customer_key_md5 {
        let formatted_16 = inner_15.as_str();
                        if !formatted_16.is_empty() {
                            let header_value = formatted_16;
                            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                                ::aws_smithy_types::error::operation::BuildError::invalid_field("x_amz_copy_source_server_side_encryption_customer_key_md5", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-copy-source-server-side-encryption-customer-key-MD5", header_value);
                        }
    }
    if let ::std::option::Option::Some(inner_17) = &input.x_amz_metadata_directive {
        let formatted_18 = inner_17.as_str();
                        if !formatted_18.is_empty() {
                            let header_value = formatted_18;
                            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                                ::aws_smithy_types::error::operation::BuildError::invalid_field("x_amz_metadata_directive", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-metadata-directive", header_value);
                        }
    }
    if let ::std::option::Option::Some(inner_19) = &input.x_amz_server_side_encryption {
        let formatted_20 = inner_19.as_str();
                        if !formatted_20.is_empty() {
                            let header_value = formatted_20;
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
    if let ::std::option::Option::Some(inner_21) = &input.x_amz_server_side_encryption_customer_algorithm {
        let formatted_22 = inner_21.as_str();
                        if !formatted_22.is_empty() {
                            let header_value = formatted_22;
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
    if let ::std::option::Option::Some(inner_23) = &input.x_amz_server_side_encryption_customer_key {
        let formatted_24 = inner_23.as_str();
                        if !formatted_24.is_empty() {
                            let header_value = formatted_24;
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
    if let ::std::option::Option::Some(inner_25) = &input.x_amz_server_side_encryption_customer_key_md5 {
        let formatted_26 = inner_25.as_str();
                        if !formatted_26.is_empty() {
                            let header_value = formatted_26;
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
    if let ::std::option::Option::Some(inner_27) = &input.x_amz_storage_class {
        let formatted_28 = inner_27.as_str();
                        if !formatted_28.is_empty() {
                            let header_value = formatted_28;
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
    if let ::std::option::Option::Some(inner_29) = &input.x_amz_tagging {
        let formatted_30 = inner_29.as_str();
                        if !formatted_30.is_empty() {
                            let header_value = formatted_30;
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
    if let ::std::option::Option::Some(inner_31) = &input.x_amz_tagging_directive {
        let formatted_32 = inner_31.as_str();
                        if !formatted_32.is_empty() {
                            let header_value = formatted_32;
                            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                                ::aws_smithy_types::error::operation::BuildError::invalid_field("x_amz_tagging_directive", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-tagging-directive", header_value);
                        }
    }
    Ok(builder)
}

#[allow(unused_mut)]
pub fn de_put_object_copy(inp: &[u8], mut builder: crate::operation::put_object_copy::builders::PutObjectCopyOutputBuilder) -> Result<crate::operation::put_object_copy::builders::PutObjectCopyOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !start_el.matches("PutObjectCopyResult") {
                            return Err(
                                ::aws_smithy_xml::decode::XmlDecodeError::custom(
                                    format!("encountered invalid XML root: expected PutObjectCopyResult but got {:?}. This is likely a bug in the SDK.", start_el)
                                )
                            )
                        }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("LastModified") /* LastModified com.nifcloud.api.storage.synthetic#PutObjectCopyOutput$LastModified */ =>  {
                let var_33 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `smithy.api#Timestamp`)"))
                        ?
                    )
                ;
                builder = builder.set_last_modified(var_33);
            }
            ,
            s if s.matches("ETag") /* ETag com.nifcloud.api.storage.synthetic#PutObjectCopyOutput$ETag */ =>  {
                let var_34 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_e_tag(var_34);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}

