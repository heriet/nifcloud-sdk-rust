// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_bucket_version2_http_error(_response_status: u16, _response_headers: &::aws_smithy_runtime_api::http::Headers, _response_body: &[u8]) -> std::result::Result<crate::operation::get_bucket_version2::GetBucketVersion2Output, crate::operation::get_bucket_version2::GetBucketVersion2Error> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body).map_err(crate::operation::get_bucket_version2::GetBucketVersion2Error::unhandled)?;
    let generic = generic_builder.build();
    Err(crate::operation::get_bucket_version2::GetBucketVersion2Error::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_bucket_version2_http_response(_response_status: u16, _response_headers: &::aws_smithy_runtime_api::http::Headers, _response_body: &[u8]) -> std::result::Result<crate::operation::get_bucket_version2::GetBucketVersion2Output, crate::operation::get_bucket_version2::GetBucketVersion2Error> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_bucket_version2::builders::GetBucketVersion2OutputBuilder::default();
        output = crate::protocol_serde::shape_get_bucket_version2::de_get_bucket_version2(_response_body, output).map_err(crate::operation::get_bucket_version2::GetBucketVersion2Error::unhandled)?;
        output = output.set_content_type(
            crate::protocol_serde::shape_get_bucket_version2_output::de_content_type_header(_response_headers)
                                    .map_err(|_|crate::operation::get_bucket_version2::GetBucketVersion2Error::unhandled("Failed to parse ContentType from header `Content-Type"))?
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_get_bucket_version2(inp: &[u8], mut builder: crate::operation::get_bucket_version2::builders::GetBucketVersion2OutputBuilder) -> Result<crate::operation::get_bucket_version2::builders::GetBucketVersion2OutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !start_el.matches("GetBucketVersion2Result") {
                            return Err(
                                ::aws_smithy_xml::decode::XmlDecodeError::custom(
                                    format!("encountered invalid XML root: expected GetBucketVersion2Result but got {:?}. This is likely a bug in the SDK.", start_el)
                                )
                            )
                        }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("CommonPrefixes") /* CommonPrefixes com.nifcloud.api.storage.synthetic#GetBucketVersion2Output$CommonPrefixes */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_common_prefixes(var_1);
            }
            ,
            s if s.matches("NextContinuationToken") /* NextContinuationToken com.nifcloud.api.storage.synthetic#GetBucketVersion2Output$NextContinuationToken */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_next_continuation_token(var_2);
            }
            ,
            s if s.matches("ContinuationToken") /* ContinuationToken com.nifcloud.api.storage.synthetic#GetBucketVersion2Output$ContinuationToken */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_continuation_token(var_3);
            }
            ,
            s if s.matches("Delimiter") /* Delimiter com.nifcloud.api.storage.synthetic#GetBucketVersion2Output$Delimiter */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_delimiter(var_4);
            }
            ,
            s if s.matches("Encoding-Type") /* EncodingType com.nifcloud.api.storage.synthetic#GetBucketVersion2Output$EncodingType */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_encoding_type(var_5);
            }
            ,
            s if s.matches("IsTruncated") /* IsTruncated com.nifcloud.api.storage.synthetic#GetBucketVersion2Output$IsTruncated */ =>  {
                let var_6 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `smithy.api#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_is_truncated(var_6);
            }
            ,
            s if s.matches("Prefix") /* Prefix com.nifcloud.api.storage.synthetic#GetBucketVersion2Output$Prefix */ =>  {
                let var_7 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_prefix(var_7);
            }
            ,
            s if s.matches("Name") /* Name com.nifcloud.api.storage.synthetic#GetBucketVersion2Output$Name */ =>  {
                let var_8 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_name(var_8);
            }
            ,
            s if s.matches("Contents") /* Contents com.nifcloud.api.storage.synthetic#GetBucketVersion2Output$Contents */ =>  {
                let var_9 =
                    Some(
                        Result::<::std::vec::Vec::<crate::types::Contents>, ::aws_smithy_xml::decode::XmlDecodeError>::Ok({
                            let mut list_10 = builder.contents.take().unwrap_or_default();
                            list_10.push(
                                crate::protocol_serde::shape_contents::de_contents(&mut tag)
                                ?
                            );
                            list_10
                        })
                        ?
                    )
                ;
                builder = builder.set_contents(var_9);
            }
            ,
            s if s.matches("StartAfter") /* StartAfter com.nifcloud.api.storage.synthetic#GetBucketVersion2Output$StartAfter */ =>  {
                let var_11 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_start_after(var_11);
            }
            ,
            s if s.matches("KeyCount") /* KeyCount com.nifcloud.api.storage.synthetic#GetBucketVersion2Output$KeyCount */ =>  {
                let var_12 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_key_count(var_12);
            }
            ,
            s if s.matches("MaxKeys") /* MaxKeys com.nifcloud.api.storage.synthetic#GetBucketVersion2Output$MaxKeys */ =>  {
                let var_13 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_max_keys(var_13);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
