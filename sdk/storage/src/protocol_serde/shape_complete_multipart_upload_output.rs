// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_content_type_header(header_map: &::aws_smithy_runtime_api::http::Headers) -> ::std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Content-Type");
    ::aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_x_amz_expiration_header(header_map: &::aws_smithy_runtime_api::http::Headers) -> ::std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-expiration");
    ::aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_x_amz_server_side_encryption_header(header_map: &::aws_smithy_runtime_api::http::Headers) -> ::std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-server-side-encryption");
    ::aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_x_amz_server_side_encryption_aws_kms_key_id_header(header_map: &::aws_smithy_runtime_api::http::Headers) -> ::std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-server-side-encryption-aws-kms-key-id");
    ::aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_x_amz_server_side_encryption_customer_algorithm_header(header_map: &::aws_smithy_runtime_api::http::Headers) -> ::std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-server-side-encryption-customer-algorithm");
    ::aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_x_amz_version_id_header(header_map: &::aws_smithy_runtime_api::http::Headers) -> ::std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-version-id");
    ::aws_smithy_http::header::one_or_none(headers)
}

