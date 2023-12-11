use imap_bodystructure::extract_bodystructure;

fn main() {
    let target = br#"BODYSTRUCTURE (("text" "html" ("charset" "utf-8") NIL NIL "base64" 1188 16 NIL NIL NIL NIL) "mixed" ("boundary" "===============1522363357941492443==") NIL NIL NIL)"#;
    let text = br#"* 154 FETCH (UID 649 FLAGS () RFC822.SIZE 2394 INTERNALDATE "05-Dec-2023 06:16:58 +0000" BODYSTRUCTURE (("text" "html" ("charset" "utf-8") NIL NIL "base64" 1188 16 NIL NIL NIL NIL) "mixed" ("boundary" "===============1522363357941492443==") NIL NIL NIL) BODY[HEADER.FIELDS (DATE SUBJECT FROM SENDER REPLY-TO TO CC BCC MESSAGE-ID REFERENCES IN-REPLY-TO X-MAILMASTER-SHOWONERCPT X-CUSTOM-MAIL-MASTER-SENT-ID DISPOSITION-NOTIFICATION-TO X-CM-CTRLMSGS)] {181}
    Subject: =?utf-8?b?5L2g5aW9IDBiMGZiYjZkYmFmM2FmYmIgenFhLWVtYWls5rWL6K+V?=
    From: liutianyu@nextcloud.games
    To: shenzongxu@nextcloud.games
    Date: Tue, 05 Dec 2023 06:16:58 -0000"#;
    let bodystructure = extract_bodystructure(&text.to_vec());
    assert_eq!(bodystructure, target);
}
