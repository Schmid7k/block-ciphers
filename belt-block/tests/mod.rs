use belt_block::BeltBlock;
use cipher::{BlockDecrypt, BlockEncrypt, KeyInit};
use hex_literal::hex;

/// Example vectors from STB 34.101.31 (2020):
/// http://apmi.bsu.by/assets/files/std/belt-spec371.pdf
#[test]
fn belt_block() {
    // Table A.1
    let key1 = hex!("E9DEE72C 8F0C0FA6 2DDB49F4 6F739647 06075316 ED247A37 39CBA383 03A98BF6");
    let pt1 = hex!("B194BAC8 0A08F53B 366D008E 584A5DE4");
    let ct1 = hex!("69CCA1C9 3557C9E3 D66BC3E0 FA88FA6E");
    // Table A.2
    let key2 = hex!("92BD9B1C E5D14101 5445FBC9 5E4D0EF2 682080AA 227D642F 2687F934 90405511");
    let pt2 = hex!("0DC53006 00CAB840 B38448E5 E993F421");
    let ct2 = hex!("E12BDC1A E28257EC 703FCCF0 95EE8DF1");

    for (key, pt, ct) in [(key1, pt1, ct1), (key2, pt2, ct2)] {
        let cipher = BeltBlock::new(&key.into());
        let mut block = pt.into();
        cipher.encrypt_block(&mut block);
        assert_eq!(block, ct.into());
        cipher.decrypt_block(&mut block);
        assert_eq!(block, pt.into());
    }
}
