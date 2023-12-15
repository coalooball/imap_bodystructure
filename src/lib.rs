//! # IMAP protocol library only related to BODYSTRUCTURE
//!
//! ### Examples
//! Extract BODYSTRUCTURE
//! ```rust
//! # use imap_bodystructure::preparser;
//! # use imap_bodystructure::parser::*;
//! # fn main() {
//! let text = br#"* 50000 FETCH (BODYSTRUCTURE ("TEXT" "PLAIN" ("CHARSET" "utf-8") NIL NIL "8BIT" 393 9 NIL NIL NIL))"#.to_vec();
//! let bodystructure_text = preparser::extract_bodystructure(&text);
//! assert_eq!(bodystructure_text, br#"BODYSTRUCTURE ("TEXT" "PLAIN" ("CHARSET" "utf-8") NIL NIL "8BIT" 393 9 NIL NIL NIL)"#.to_vec());
//! let body_text_within_parentheses = head_bodystructure(&bodystructure_text).unwrap().0;
//! assert_eq!(body_text_within_parentheses, br#"("TEXT" "PLAIN" ("CHARSET" "utf-8") NIL NIL "8BIT" 393 9 NIL NIL NIL)"#.as_ref());
//! let body_tmp = Body::Single(SingleBody {
//!     content_type: ContentTypeHeaderField {
//!         ttype: ContentTypeTypeAndSubType {
//!             ttype: b"TEXT".to_vec(),
//!             subtype: b"PLAIN".to_vec()
//!         },
//!         parameters: Parameters {
//!             list: vec![Parameter {
//!                 attribute: b"CHARSET".to_vec(),
//!                 value: b"utf-8".to_vec()
//!             }]
//!         }
//!     },
//!     content_id: ContentIDHeaderField {
//!         value: None
//!     },
//!     content_description: ContentDescriptionHeaderField { value: None },
//!     content_transfer_encoding: ContentTransferEncodingHeaderField {
//!         value: b"8BIT".to_vec()
//!     },
//!     content_size: ContentSize(Some(393), Some(9)),
//!     content_md5: ContentMD5HeaderField {
//!         value: None
//!     },
//!     content_disposition: ContentDispositionHeaderField {
//!         value: None,
//!         parameters: Parameters { list: vec![
//!         ] }
//!     },
//!     content_language: ContentLanguageHeaderField { value: None },
//!     content_location: ContentLocationHeaderField { value: None },
//!     data: vec![],
//!     raw_header: vec![],
//! });
//! assert_eq!(body_parser(body_text_within_parentheses).unwrap().1, body_tmp);
//! # }
//! ```

pub mod parser;
// Get new SequenceNumbers
pub mod sequence;
pub mod preparser;
pub mod fetch;