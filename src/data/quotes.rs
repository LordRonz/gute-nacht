use base64::prelude::*;

pub fn get_quotes() -> [String; 10] {
    let quotes_raw = [
        "R29vZCBuaWdodCwgaGF2ZSBhIG5pY2UgZHJlYW0=",
        "R29vZG5pZ2h0IGFuZCBzd2VldCBkcmVhbXM=",
        "U2xlZXAgdGlnaHQh",
        "VW50aWwgdG9tb3Jyb3c=",
        "Q2xvc2UgeW91ciBleWVzLCBhbmQgZ29vZG5pZ2h0",
        "U2xlZXAgd2VsbCwgYW5kIHN3ZWV0IGRyZWFtcyE=",
        "TmlnaHQgbmlnaHQuIFNsZWVwIHRpZ2h0Lg==",
        "TWF5IHRoZSBtb29uIGFuZCBzdGFycyBzbWlsZSB1cG9uIHlvdSBhcyB5b3Ugc2xlZXA=",
        "SGF2ZSBhIGdyZWF0IG5pZ2h0cyByZXN0Lg==",
        "Tml0ZSBuaXRlICYgc3dlZXQgZHJlYW1z",
    ];

    let quotes = quotes_raw
        .map(|x| BASE64_STANDARD.decode(x).unwrap())
        .map(|x| String::from_utf8(x).unwrap());

    return quotes;
}
