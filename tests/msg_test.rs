use rk::message::Message;

const KEY: &[u8] = "this is very secret key 12345678".as_bytes();

// #[test]
// fn message_encode_and_decode_test() {
//     let msg = Message::new("msg_type", "content", "data".as_bytes().to_vec(), 1);
//     let msg_encode = msg.encode(KEY).unwrap();
//     let msg_decode = Message::decode(KEY, &msg_encode).unwrap();
//     assert_eq!(msg, msg_decode);
// }
