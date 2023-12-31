// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_accept_ranges_header(header_map: &::aws_smithy_runtime_api::http::Headers) -> ::std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Accept-Ranges");
    ::aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_body_payload(body: &[u8]) -> std::result::Result<::std::option::Option<::aws_smithy_types::Blob>, crate::operation::get_object::GetObjectError> {
    (!body.is_empty()).then(||{
        Ok(::aws_smithy_types::Blob::new(body))
    }).transpose()
}

pub(crate) fn de_content_range_header(header_map: &::aws_smithy_runtime_api::http::Headers) -> ::std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Content-Range");
    ::aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_content_type_header(header_map: &::aws_smithy_runtime_api::http::Headers) -> ::std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Content-Type");
    ::aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_e_tag_header(header_map: &::aws_smithy_runtime_api::http::Headers) -> ::std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("ETag");
    ::aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_last_modified_header(header_map: &::aws_smithy_runtime_api::http::Headers) -> ::std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Last-Modified");
    ::aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_x_amz_expiration_header(header_map: &::aws_smithy_runtime_api::http::Headers) -> ::std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-expiration");
    ::aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_x_amz_mp_parts_count_header(header_map: &::aws_smithy_runtime_api::http::Headers) -> ::std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-mp-parts-count");
    ::aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_x_amz_server_side_encryption_header(header_map: &::aws_smithy_runtime_api::http::Headers) -> ::std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-server-side-encryption");
    ::aws_smithy_http::header::one_or_none(headers)
}

