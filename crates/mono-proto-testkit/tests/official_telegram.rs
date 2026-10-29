//! Vectors published by Telegram at <https://core.telegram.org/mtproto/samples-auth_key>.

use mono_proto_codec::{Decoder, Limits, TlDecode, decode_bytes, decode_vector};
use mono_proto_crypto::{
    aes_ige_decrypt, factor_pq, new_nonce_hash, sha1, temporary_aes_key_iv, validate_dh_parameters,
    validate_dh_public_value,
};
use mono_proto_engine::{decode_plain_message, encode_plain_message};
use num_bigint::BigUint;

fn hex(value: &str) -> Vec<u8> {
    let compact: Vec<_> = value
        .bytes()
        .filter(|byte| !byte.is_ascii_whitespace())
        .collect();
    assert_eq!(compact.len() % 2, 0);
    compact
        .chunks_exact(2)
        .map(|pair| {
            let digit = |byte: u8| match byte {
                b'0'..=b'9' => byte - b'0',
                b'a'..=b'f' => byte - b'a' + 10,
                b'A'..=b'F' => byte - b'A' + 10,
                _ => panic!("non-hex fixture byte"),
            };
            digit(pair[0]) << 4 | digit(pair[1])
        })
        .collect()
}

#[test]
fn dh_completion_nonce_hash_matches_sample() {
    let new_nonce: [u8; 32] =
        hex("BF8CB5BD9C5B4FE7CF24D64D281F89311576D53C0DA65A83267E57315414C9A6")
            .try_into()
            .unwrap();
    let auth_key = hex(concat!(
        "8E1081A1B5CA1B399A9A9D7E08BB9A9182AB634F8C03F2A49F944E2F944A9C71",
        "EDBA61A32A70D3DADEB33752AE515B16B2D8E75039C40EBE18136775C3727372A",
        "8DF486606D671FD63842DF0A44ACC31E68B7B1EC6A731A1DC5C748F0CB46AC00F",
        "DE363F0520B51D9B59EAE519EA511A8E8591FC7010DF0B07CDBAB04013DD85172",
        "CB54555DC5C982EA0A5DCF4411E798D338B823161FD8C93100B7A426186B4C16F",
        "9113521081C8D2075872F4A0CF238034843DC01F2C26828721A2E2FFD93A9B014",
        "2B8DF6355C43D9AEF5B448F1CC0D84E0E72A7FF494D4CC3B1650050DDEC5DC32",
        "1ADA68E420F45098280CEAB58A1CBFAA60FFF3218E56B4741143AC5A6F0"
    ));
    assert_eq!(
        new_nonce_hash(&new_nonce, &auth_key, 1),
        hex("AA404B58DF404D8F363772B14CE5A56F").as_slice()
    );
}

#[test]
fn unencrypted_req_pq_sample_round_trips() {
    let payload =
        hex("000000000000000078F404006170466A14000000F18E7EBE51A1143FC7A3666BE4BE54D6890A02DC");
    let decoded = decode_plain_message(&payload, 1024).unwrap();
    assert_eq!(
        decoded.message_id.to_le_bytes(),
        hex("78F404006170466A").as_slice()
    );
    assert_eq!(
        decoded.body,
        hex("F18E7EBE51A1143FC7A3666BE4BE54D6890A02DC")
    );
    assert_eq!(encode_plain_message(&decoded).unwrap(), payload);
}

#[test]
fn res_pq_tl_payload_decodes_exactly() {
    let payload = hex(concat!(
        "000000000000000001F4CCC26170466A500000006324160551A1143FC7A3666B",
        "E4BE54D6890A02DC63248F6748214EAB8A2F4CC876E11974082E9CDB98C80CDA",
        "4B00000015C4B51C0300000085FD64DE851D9DD0A5B7F709355FC30B216BE86C",
        "022BB4C3"
    ));
    let plain = decode_plain_message(&payload, 1024).unwrap();
    let mut decoder = Decoder::new(&plain.body, Limits::default()).unwrap();
    assert_eq!(decoder.read_u32().unwrap(), 0x0516_2463);
    let nonce = <[u8; 16]>::decode(&mut decoder).unwrap();
    let server_nonce = <[u8; 16]>::decode(&mut decoder).unwrap();
    assert_eq!(nonce, hex("51A1143FC7A3666BE4BE54D6890A02DC").as_slice());
    assert_eq!(
        server_nonce,
        hex("63248F6748214EAB8A2F4CC876E11974").as_slice()
    );
    assert_eq!(decode_bytes(&mut decoder).unwrap(), hex("2E9CDB98C80CDA4B"));
    assert_eq!(
        decode_vector::<i64>(&mut decoder, true).unwrap(),
        [
            i64::from_le_bytes(hex("85FD64DE851D9DD0").try_into().unwrap()),
            i64::from_le_bytes(hex("A5B7F709355FC30B").try_into().unwrap()),
            i64::from_le_bytes(hex("216BE86C022BB4C3").try_into().unwrap()),
        ]
    );
    decoder.finish().unwrap();
}

#[test]
fn proof_of_work_factorization_matches_sample() {
    assert_eq!(
        factor_pq(3_358_800_871_349_344_843).unwrap(),
        (1_786_331_737, 1_880_278_339)
    );
}

#[test]
fn temporary_aes_and_server_dh_answer_match_sample() {
    let new_nonce: [u8; 32] =
        hex("BF8CB5BD9C5B4FE7CF24D64D281F89311576D53C0DA65A83267E57315414C9A6")
            .try_into()
            .unwrap();
    let server_nonce: [u8; 16] = hex("63248F6748214EAB8A2F4CC876E11974").try_into().unwrap();
    let (key, iv) = temporary_aes_key_iv(&new_nonce, &server_nonce);
    assert_eq!(
        key,
        hex("16F548177058E8D39C41CBAD4D419446BEB12EB9B8F5AD28EA824B8015F17D81").as_slice()
    );
    assert_eq!(
        iv,
        hex("C4D14166C1378E35C698460047DBB6075441BE9984611C28837357EBBF8CB5BD").as_slice()
    );

    let encrypted = hex(concat!(
        "C334D313064174F443CE90E13C835FAEA6AE9677089A0781CC8C17ADC8FF5B50",
        "72934C1DFB1F2B9222197DE806186E6612E0CFA2593809B4B91B49F006FFBBD9",
        "EAAADE1EEDCA046F500A77BB538E3C2F02A4A6814DF0BC77993E493B7F2C9834",
        "4F674455A90A541070740F4B811FFF4B80B161737E0E867FF20D03BE6B52BA66",
        "F7319D03B621732E1C880202EAF61DF31DE831E7AC97B0FFDBFFFFA7019D3995",
        "53F37B645913238443F4C560A59A5BA6AA6BFAEB159FD2291FCDA49A23E80091",
        "96B8062DD424F45D3B43538C68B2C070A845C260052DD3C266659FA6C0C6A8FF",
        "36FBFF8DAB36E06BEB5E18AFE38027FDA45E65884A503402840E21C1869101F4",
        "C613E9EDA61C2EB0AA987046F8069C2C002EE48A95844DD62E0E4B612257391B",
        "014D3B043D7C193F936352F9D799CC4017CA544896BB09B3B5B8B70C2C5E4829",
        "5A82CF13BCF5FB0BA52991EC5C25188AB783CBCED3573BABB255E82741EAF494",
        "1609AEFF960A4F2B41F92E78595141EBC73677E09104B8690D3E3C30FCA78BB0",
        "F97B4336E925BCB0C85A805458EE7B8DAD7599388C338394FB317C0C6BD5EC2C",
        "D564177EC059E9705EFAF2048ED0B014EA89C60EE48CC547FE51CE4D0F13DEA2",
        "ABEA9F2425B3FA2987D960EDEF619B67921B2E92219C81F709C2924414932257",
        "D5F3EEB12D0AAF2B29511988DACC966792D7E26F04EF5F78CC18F8A9C620668F",
        "D76A668AC6E6D43474BFB5CFC927C15B5D1FD531F50B7EDFFCD50F6F04A08845",
        "66CC858D05A2B846D69A8799D36022112ACCCA567D6B5EFE799AC93DE439A7D1",
        "6CE61C16B02F89AD9ACBD045111BC5F1"
    ));
    let decrypted = aes_ige_decrypt(&encrypted, &key, &iv).unwrap();
    assert_eq!(&decrypted[20..24], hex("BA0D89B5").as_slice());
    assert_eq!(&decrypted[..20], sha1(&decrypted[20..584]).as_slice());

    let prime = BigUint::from_bytes_be(&decrypted[64..320]);
    let server_public = BigUint::from_bytes_be(&decrypted[324..580]);
    validate_dh_parameters(&prime, 3).unwrap();
    validate_dh_public_value(&server_public, &prime).unwrap();
}
