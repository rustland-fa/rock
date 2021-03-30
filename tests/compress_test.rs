use rk::compress::*;

#[test]
pub fn compress_test() {
    let data_input = "Hello World !";
    let data_compress = compress(data_input.as_bytes()).unwrap();
    let data_decompress = decompress(&data_compress).unwrap();
    let data_output = std::str::from_utf8(&data_decompress).unwrap();
    assert_eq!(data_output, data_input);
}
