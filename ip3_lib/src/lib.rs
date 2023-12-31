use bitvec::prelude::*;
use const_str::{split, strip_suffix, unwrap};
use std::net::Ipv4Addr;

macro_rules! wordlist {
    ($file:expr) => {
        split!(unwrap!(strip_suffix!(include_str!($file), "\n")), "\n")
    };
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ip3(String, String, String);

impl std::fmt::Display for Ip3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.0, self.1, self.2)
    }
}

impl From<(String, String, String)> for Ip3 {
    fn from(words: (String, String, String)) -> Self {
        Self(words.0, words.1, words.2)
    }
}

impl From<(&str, &str, &str)> for Ip3 {
    fn from(words: (&str, &str, &str)) -> Self {
        Self(
            words.0.to_string(),
            words.1.to_string(),
            words.2.to_string(),
        )
    }
}

impl From<Ip3> for String {
    fn from(ip: Ip3) -> Self {
        format!("{}", ip)
    }
}

impl From<String> for Ip3 {
    fn from(ip: String) -> Self {
        let mut ip = ip.split('.');
        let word1 = ip.next().unwrap();
        let word2 = ip.next().unwrap();
        let word3 = ip.next().unwrap();
        Self(word1.to_string(), word2.to_string(), word3.to_string())
    }
}

impl From<Ipv4Addr> for Ip3 {
    fn from(ip: Ipv4Addr) -> Self {
        ipv4_to_ip3(ip)
    }
}

impl From<Ip3> for Ipv4Addr {
    fn from(ip: Ip3) -> Self {
        ip3_to_ipv4(&ip)
    }
}

pub const WORDLIST_EN: [&str; 2048] = wordlist!("../wordlists/english.txt");

#[cfg(feature = "en")]
pub const WORDLIST: [&str; 2048] = WORDLIST_EN;

/// Converts a word to a bitvec
pub fn word_to_bytes(word: &str) -> BitVec {
    // O(1) since constant number of words :D
    let index_in_wordlist = WORDLIST
        .iter()
        .position(|&x| x == word)
        .expect("Word not found in wordlist");

    index_in_wordlist.view_bits::<Lsb0>()[..11].to_bitvec()
}

/// Checks if a word is in the wordlist
pub fn in_word_list(word: &str) -> bool {
    WORDLIST.contains(&word)
}

/// Converts 3 words to an ipv4 address
pub fn ip3_to_ipv4(words: &Ip3) -> Ipv4Addr {
    assert!(in_word_list(&words.0), "{} not in wordlist", words.0);
    assert!(in_word_list(&words.1), "{} not in wordlist", words.1);
    assert!(in_word_list(&words.2), "{} not in wordlist", words.2);
    let word1_bytes = word_to_bytes(&words.0);
    let word2_bytes = word_to_bytes(&words.1);
    let word3_bytes = word_to_bytes(&words.2);
    let mut bytes = bitvec![u64,Lsb0;];

    bytes.extend(&word1_bytes);
    bytes.extend(&word2_bytes);
    bytes.extend(&word3_bytes);
    // remove last byte as we only want 32
    bytes.pop();

    // bytes to u32
    let ip_int: u32 = bytes.load();
    // u32 to ipv4

    Ipv4Addr::from(ip_int)
}

/// Converts an ipv4 address to 3 words
pub fn ipv4_to_ip3(ip: Ipv4Addr) -> Ip3 {
    let ip_int = u32::from(ip);
    let bytes = ip_int.view_bits::<Lsb0>();
    let mut bytes = bytes[..32].to_bitvec();

    let word1_bytes = bytes.drain(..11).collect::<BitVec>();
    let word2_bytes = bytes.drain(..11).collect::<BitVec>();
    let word3_bytes = bytes.drain(..10).collect::<BitVec>();

    let word1: &str = WORDLIST[word1_bytes.load::<usize>()];
    let word2: &str = WORDLIST[word2_bytes.load::<usize>()];
    let word3: &str = WORDLIST[word3_bytes.load::<usize>()];

    (word1.to_string(), word2.to_string(), word3.to_string()).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_to_bytes() {
        assert_eq!(word_to_bytes("abandon").as_raw_slice(), [0]);
        assert_eq!(word_to_bytes("zoo").as_raw_slice(), [2047]);
    }

    #[test]
    fn test_words_to_ip() {
        assert_eq!(
            ip3_to_ipv4(&("abandon", "abandon", "abandon").into()),
            Ipv4Addr::new(0, 0, 0, 0)
        );
        assert_eq!(
            ip3_to_ipv4(&("ability", "abandon", "display").into()),
            Ipv4Addr::new(127, 0, 0, 1)
        );
        assert_eq!(
            ip3_to_ipv4(&("cage", "advice", "above").into()),
            Ipv4Addr::new(1, 1, 1, 1)
        )
    }

    #[test]
    fn test_ip_to_words() {
        assert_eq!(
            ipv4_to_ip3(Ipv4Addr::new(0, 0, 0, 0)),
            ("abandon", "abandon", "abandon").into()
        );
        assert_eq!(
            ipv4_to_ip3(Ipv4Addr::new(127, 0, 0, 1)),
            ("ability", "abandon", "display").into()
        );
        assert_eq!(
            ipv4_to_ip3(Ipv4Addr::new(1, 1, 1, 1)),
            ("cage", "advice", "above").into()
        )
    }
}
