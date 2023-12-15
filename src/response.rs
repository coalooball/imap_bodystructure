use crate::extractor;
pub use crate::extractor::uid_fetch_body_parser;
use crate::parser;
use std::collections::HashMap;

pub fn find_all_bodystructure_with_uid(s: &mut Vec<u8>) -> HashMap<Vec<u8>, parser::Body> {
    let mut tmp_hashmap = HashMap::new();
    let responses = extractor::split_multi_fetch_response(s);
    for response in responses.iter() {
        let uid = extractor::find_uid_in_response(response);
        if uid.len() == 0 {
            continue;
        }
        let bodystructure_text = extractor::extract_bodystructure(response);
        let body_text_within_parentheses =
            parser::head_bodystructure(&bodystructure_text).unwrap().0;
        let body_result = parser::body_parser(body_text_within_parentheses);
        match body_result {
            Ok((_, body)) => {
                tmp_hashmap.insert(uid, body);
            }
            Err(_) => {
                continue;
            }
        }
    }
    tmp_hashmap
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::*;
    use crate::parser::{Body, MultiBody, SingleBody};
    #[test]
    fn test_find_all_bodystructure_with_uid() {
        let mut text1 = b"1234".to_vec();
        let r1 = find_all_bodystructure_with_uid(&mut text1);
        assert_eq!(r1, HashMap::new());

        let mut text2 = b"* 154 FETCH (UID 649 FLAGS () RFC822.SIZE 2394 INTERNALDATE \"05-Dec-2023 06:16:58 +0000\" BODYSTRUCTURE ((\"text\" \"html\" (\"charset\" \"utf-8\") NIL NIL \"base64\" 1188 16 NIL NIL NIL NIL) \"mixed\" (\"boundary\" \"===============1522363357941492443==\") NIL NIL NIL) BODY[HEADER.FIELDS (DATE SUBJECT FROM SENDER REPLY-TO TO CC BCC MESSAGE-ID REFERENCES IN-REPLY-TO X-MAILMASTER-SHOWONERCPT X-CUSTOM-MAIL-MASTER-SENT-ID DISPOSITION-NOTIFICATION-TO X-CM-CTRLMSGS)] {181}\r\nSubject: =?utf-8?b?5L2g5aW9IDBiMGZiYjZkYmFmM2FmYmIgenFhLWVtYWls5rWL6K+V?=\r\nFrom: liutianyu@nextcloud.games\r\nTo: shenzongxu@nextcloud.games\r\nDate: Tue, 05 Dec 2023 06:16:58 -0000\r\n\r\n)".to_vec();
        let r2 = find_all_bodystructure_with_uid(&mut text2);
        let mut h2: HashMap<Vec<u8>, parser::Body> = HashMap::new();
        h2.insert(
            b"649".to_vec(),
            Body::Multi(MultiBody {
                parts: vec![Body::Single(SingleBody {
                    content_type: ContentTypeHeaderField {
                        ttype: ContentTypeTypeAndSubType {
                            ttype: b"text".to_vec(),
                            subtype: b"html".to_vec(),
                        },
                        parameters: Parameters {
                            list: vec![Parameter {
                                attribute: b"charset".to_vec(),
                                value: b"utf-8".to_vec(),
                            }],
                        },
                    },
                    content_id: ContentIDHeaderField { value: None },
                    content_description: ContentDescriptionHeaderField { value: None },
                    content_transfer_encoding: ContentTransferEncodingHeaderField {
                        value: b"base64".to_vec(),
                    },
                    content_size: ContentSize(Some(1188), Some(16)),
                    content_md5: ContentMD5HeaderField { value: None },
                    content_disposition: ContentDispositionHeaderField {
                        value: None,
                        parameters: Parameters { list: vec![] },
                    },
                    content_language: ContentLanguageHeaderField { value: None },
                    content_location: ContentLocationHeaderField { value: None },
                    data: vec![],
                    raw_header: vec![],
                })],
                content_type: b"mixed".to_vec(),
                parameters: Parameters {
                    list: vec![Parameter {
                        attribute: b"boundary".to_vec(),
                        value: b"===============1522363357941492443==".to_vec(),
                    }],
                },
                raw_header: vec![],
            }),
        );
        assert_eq!(r2, h2);
    }
}
