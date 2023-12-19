### Analyzing the BODYSTRUCTURE data stream within the IMAP protocol.

###### Example
Extract BODYSTRUCTURE
```rust
use imap_bodystructure::{extractor, parser::*};
fn main() {
let text = br#"* 50000 FETCH (BODYSTRUCTURE ("TEXT" "PLAIN" ("CHARSET" "utf-8") NIL NIL "8BIT" 393 9 NIL NIL NIL))"#.to_vec();
let bodystructure_text = extractor::extract_bodystructure(&text);
assert_eq!(bodystructure_text, br#"BODYSTRUCTURE ("TEXT" "PLAIN" ("CHARSET" "utf-8") NIL NIL "8BIT" 393 9 NIL NIL NIL)"#.to_vec());
let body_text_within_parentheses = head_bodystructure(&bodystructure_text).unwrap().0;
assert_eq!(body_text_within_parentheses, br#"("TEXT" "PLAIN" ("CHARSET" "utf-8") NIL NIL "8BIT" 393 9 NIL NIL NIL)"#.as_ref());
let body_tmp = Body::Single(SingleBody {
    content_type: ContentTypeHeaderField {
        ttype: ContentTypeTypeAndSubType {
            ttype: b"TEXT".to_vec(),
            subtype: b"PLAIN".to_vec()
        },
        parameters: Parameters {
            list: vec![Parameter {
                attribute: b"CHARSET".to_vec(),
                value: b"utf-8".to_vec()
            }]
        }
    },
    content_id: ContentIDHeaderField {
        value: None
    },
    content_description: ContentDescriptionHeaderField { value: None },
    content_transfer_encoding: ContentTransferEncodingHeaderField {
        value: b"8BIT".to_vec()
    },
    content_size: ContentSize(Some(393), Some(9)),
    content_md5: ContentMD5HeaderField {
        value: None
    },
    content_disposition: ContentDispositionHeaderField {
        value: None,
        parameters: Parameters { list: vec![
        ] }
    },
    content_language: ContentLanguageHeaderField { value: None },
    content_location: ContentLocationHeaderField { value: None },
    data: vec![],
    raw_header: vec![],
});
assert_eq!(body_parser(body_text_within_parentheses).unwrap().1, body_tmp);
# }
```

If you want to extract BODYSTRUCT with uid, you can use find_all_bodystructure_with_uid
```rust
# use imap_bodystructure::response::find_all_bodystructure_with_uid;
# use imap_bodystructure::parser::*;
# use std::collections::HashMap;

let mut text = b"* 154 FETCH (UID 649 FLAGS () RFC822.SIZE 2394 INTERNALDATE \"05-Dec-2023 06:16:58 +0000\" BODYSTRUCTURE ((\"text\" \"html\" (\"charset\" \"utf-8\") NIL NIL \"base64\" 1188 16 NIL NIL NIL NIL) \"mixed\" (\"boundary\" \"===============1522363357941492443==\") NIL NIL NIL) BODY[HEADER.FIELDS (DATE SUBJECT FROM SENDER REPLY-TO TO CC BCC MESSAGE-ID REFERENCES IN-REPLY-TO X-MAILMASTER-SHOWONERCPT X-CUSTOM-MAIL-MASTER-SENT-ID DISPOSITION-NOTIFICATION-TO X-CM-CTRLMSGS)] {181}\r\nSubject: =?utf-8?b?5L2g5aW9IDBiMGZiYjZkYmFmM2FmYmIgenFhLWVtYWls5rWL6K+V?=\r\nFrom: liutianyu@nextcloud.games\r\nTo: shenzongxu@nextcloud.games\r\nDate: Tue, 05 Dec 2023 06:16:58 -0000\r\n\r\n)\r\n".to_vec();
let r = find_all_bodystructure_with_uid(&mut text, true);
let mut h: HashMap<Vec<u8>, Body> = HashMap::new();
h.insert(
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
        raw_header: b"Subject: =?utf-8?b?5L2g5aW9IDBiMGZiYjZkYmFmM2FmYmIgenFhLWVtYWls5rWL6K+V?=\r\nFrom: liutianyu@nextcloud.games\r\nTo: shenzongxu@nextcloud.games\r\nDate: Tue, 05 Dec 2023 06:16:58 -0000\r\nMIME-Version: 1.0\r\n".to_vec(),
    }),
);
assert_eq!(r, (b"".as_ref(), h));
```