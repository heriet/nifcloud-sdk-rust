// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_comtent_type_header(header_map: &::aws_smithy_runtime_api::http::Headers) -> ::std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Comtent-Type");
    ::aws_smithy_http::header::one_or_none(headers)
}

