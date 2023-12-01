// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_tag_set_of_get_bucket_tagging(decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::types::TagSetOfGetBucketTagging, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::TagSetOfGetBucketTagging::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Key") /* Key com.nifcloud.api.storage#TagSetOfGetBucketTagging$Key */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_key(var_1);
            }
            ,
            s if s.matches("Value") /* Value com.nifcloud.api.storage#TagSetOfGetBucketTagging$Value */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_value(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
