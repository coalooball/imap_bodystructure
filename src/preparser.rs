fn ascii_lowercase_equal(vec1: &[u8], vec2: &[u8]) -> bool {
    vec1.iter()
        .map(|&b| b.to_ascii_lowercase())
        .eq(vec2.iter().map(|&b| b.to_ascii_lowercase()))
}

pub fn extract_bodystructure(origin_vec: &Vec<u8>) -> Vec<u8> {
    let mut token: Vec<u8> = Vec::new();
    let mut bodystructure: Vec<u8> = Vec::new();
    let mut recording = false;
    let mut brackets_count = 0;

    for &i in origin_vec {
        if recording {
            bodystructure.push(i);
            if i == b'(' {
                brackets_count += 1;
            } else if i == b')' {
                brackets_count -= 1;
                if brackets_count == 0 {
                    break;
                }
            }
        } else {
            if i.is_ascii_alphabetic() {
                token.push(i);
            } else {
                if ascii_lowercase_equal(&token, b"BODYSTRUCTURE") {
                    recording = true;
                    bodystructure.extend_from_slice(b"BODYSTRUCTURE");
                    bodystructure.push(i);
                    if i == b'(' {
                        brackets_count += 1;
                    }
                }
                token.clear();
            }
        }
    }

    bodystructure
}

pub fn extract_bodystructures(origin_vec: &Vec<u8>) -> Vec<Vec<u8>> {
    let mut bodystructures: Vec<Vec<u8>> = Vec::new();
    let mut token: Vec<u8> = Vec::new();
    let mut recording = false;
    let mut brackets_count = 0;
    let mut current_bodystructure: Vec<u8> = Vec::new();

    for &i in origin_vec {
        if recording {
            current_bodystructure.push(i);
            if i == b'(' {
                brackets_count += 1;
            } else if i == b')' {
                brackets_count -= 1;
                if brackets_count == 0 {
                    bodystructures.push(current_bodystructure.clone());
                    current_bodystructure.clear();
                    recording = false;
                }
            }
        } else {
            if i.is_ascii_alphabetic() {
                token.push(i);
            } else {
                if ascii_lowercase_equal(&token, b"BODYSTRUCTURE") {
                    recording = true;
                    current_bodystructure.extend_from_slice(b"BODYSTRUCTURE");
                    current_bodystructure.push(i);
                    if i == b'(' {
                        brackets_count += 1;
                    }
                }
                token.clear();
            }
        }
    }

    bodystructures
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_bodystructure_test_1() {
        let target = br#"BODYSTRUCTURE ((("TEXT" "HTML" ("charset" "gbk") NIL NIL "BASE64" 140 2 NIL NIL NIL) "RELATED" ("BOUNDARY" "----=_Part_28035_897908132.1699414214660") NIL NIL) "MIXED" ("BOUNDARY" "----=_Part_28034_578039922.1699414214660") NIL NIL)"#;
        let text = br#"* 10 FETCH (BODYSTRUCTURE ((("TEXT" "HTML" ("charset" "gbk") NIL NIL "BASE64" 140 2 NIL NIL NIL) "RELATED" ("BOUNDARY" "----=_Part_28035_897908132.1699414214660") NIL NIL) "MIXED" ("BOUNDARY" "----=_Part_28034_578039922.1699414214660") NIL NIL))"#;
        let bodystructure = extract_bodystructure(&text.to_vec());
        assert_eq!(bodystructure, target);
    }
    #[test]
    fn extract_bodystructure_test_2() {
        let target = br#"BODYSTRUCTURE (("text" "html" ("charset" "utf-8") NIL NIL "base64" 1188 16 NIL NIL NIL NIL) "mixed" ("boundary" "===============1522363357941492443==") NIL NIL NIL)"#;
        let text = br#"* 154 FETCH (UID 649 FLAGS () RFC822.SIZE 2394 INTERNALDATE "05-Dec-2023 06:16:58 +0000" BODYSTRUCTURE (("text" "html" ("charset" "utf-8") NIL NIL "base64" 1188 16 NIL NIL NIL NIL) "mixed" ("boundary" "===============1522363357941492443==") NIL NIL NIL) BODY[HEADER.FIELDS (DATE SUBJECT FROM SENDER REPLY-TO TO CC BCC MESSAGE-ID REFERENCES IN-REPLY-TO X-MAILMASTER-SHOWONERCPT X-CUSTOM-MAIL-MASTER-SENT-ID DISPOSITION-NOTIFICATION-TO X-CM-CTRLMSGS)] {181}
        Subject: =?utf-8?b?5L2g5aW9IDBiMGZiYjZkYmFmM2FmYmIgenFhLWVtYWls5rWL6K+V?=
        From: liutianyu@nextcloud.games
        To: shenzongxu@nextcloud.games
        Date: Tue, 05 Dec 2023 06:16:58 -0000"#;
        let bodystructure = extract_bodystructure(&text.to_vec());
        assert_eq!(bodystructure, target);
    }
    #[test]
    fn ascii_lowercase_equal_test_1() {
        let str1 = b"Hello";
        let str2 = b"hello";
        let str3 = b"HELLO";
        assert_ne!(str1, str2);
        assert_eq!(str1.to_ascii_lowercase(), str2.to_ascii_lowercase());
        assert!(ascii_lowercase_equal(str2, str3));
    }
    #[test]
    fn test_extract_bodystructures() {
        let text = br#"* 154 FETCH (UID 649 FLAGS () RFC822.SIZE 2394 INTERNALDATE "05-Dec-2023 06:16:58 +0000" BODYSTRUCTURE (("text" "html" ("charset" "utf-8") NIL NIL "base64" 1188 16 NIL NIL NIL NIL) "mixed" ("boundary" "===============1522363357941492443==") NIL NIL NIL) BODY[HEADER.FIELDS (DATE SUBJECT FROM SENDER REPLY-TO TO CC BCC MESSAGE-ID REFERENCES IN-REPLY-TO X-MAILMASTER-SHOWONERCPT X-CUSTOM-MAIL-MASTER-SENT-ID DISPOSITION-NOTIFICATION-TO X-CM-CTRLMSGS)] {181}
        Subject: =?utf-8?b?5L2g5aW9IDBiMGZiYjZkYmFmM2FmYmIgenFhLWVtYWls5rWL6K+V?=
        From: liutianyu@nextcloud.games
        To: shenzongxu@nextcloud.games
        Date: Tue, 05 Dec 2023 06:16:58 -0000
        
        )
        * 155 FETCH (UID 650 FLAGS () RFC822.SIZE 2869 INTERNALDATE "05-Dec-2023 06:16:58 +0000" BODYSTRUCTURE (("text" "html" ("charset" "utf-8") NIL NIL "base64" 54 1 NIL NIL NIL NIL)("application" "octet-stream" NIL NIL NIL "base64" 1336 NIL ("attachment" ("filename*" "utf-8''%E5%85%AC%E6%B0%91%E6%95%B0%E6%8D%AE.txt.zip")) NIL NIL) "mixed" ("boundary" "===============6973775584883558730==") NIL NIL NIL) BODY[HEADER.FIELDS (DATE SUBJECT FROM SENDER REPLY-TO TO CC BCC MESSAGE-ID REFERENCES IN-REPLY-TO X-MAILMASTER-SHOWONERCPT X-CUSTOM-MAIL-MASTER-SENT-ID DISPOSITION-NOTIFICATION-TO X-CM-CTRLMSGS)] {225}
        Subject: =?utf-8?b?6ZmE5Lu25pC65bimemlw5Y6L57ypdHh055qE5YWs5rCR5pWw5o2uIDBlZjBmZTU5OTNiYTdkNmEgenFhLWVtYWls5rWL6K+V?=
        From: liutianyu@nextcloud.games
        To: shenzongxu@nextcloud.games
        Date: Tue, 05 Dec 2023 06:16:58 -0000
        
        )"#.to_vec();
        assert_eq!(extract_bodystructures(&text), vec![
            br#"BODYSTRUCTURE (("text" "html" ("charset" "utf-8") NIL NIL "base64" 1188 16 NIL NIL NIL NIL) "mixed" ("boundary" "===============1522363357941492443==") NIL NIL NIL)"#.to_vec(),
            br#"BODYSTRUCTURE (("text" "html" ("charset" "utf-8") NIL NIL "base64" 54 1 NIL NIL NIL NIL)("application" "octet-stream" NIL NIL NIL "base64" 1336 NIL ("attachment" ("filename*" "utf-8''%E5%85%AC%E6%B0%91%E6%95%B0%E6%8D%AE.txt.zip")) NIL NIL) "mixed" ("boundary" "===============6973775584883558730==") NIL NIL NIL)"#.to_vec()
            ])
    }
}
