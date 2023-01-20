use sha2::{Digest, Sha256};
use bls12_381::{G2Affine, G2Projective, Scalar};
use hkdf::Hkdf;
use group::{Curve};
use structopt::StructOpt;
use core::fmt;
use std::env;
use serde_derive::{Deserialize, Serialize};

const IKM: &str = "746869732d49532d6a7573742d616e2d546573742d494b4d2d746f2d67656e65726174652d246528724074232d6b6579";
const KEY_INFO: &str = "746869732d49532d736f6d652d6b65792d6d657461646174612d746f2d62652d757365642d696e2d746573742d6b65792d67656e";

// Invalid IKM or KeyInfo error
#[derive(Debug, Clone)]
struct BadParams { cause: String }

impl fmt::Display for BadParams {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid parameters: {c}", c = self.cause)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
struct KeyPair {
    secretKey: String,
    publicKey: String
}

#[derive(Debug)]
enum OutputType {
    Print,
    File,
}

impl std::str::FromStr for OutputType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "f" | "fi" | "fil" | "file" => Ok(OutputType::File),
            "p" | "pr" | "pri" | "print" => Ok(OutputType::Print),
            _ => Err("Invalid Value".to_string()),
        }
    }
}
#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(long, default_value = &IKM)]
    ikm: String,
    #[structopt(long, default_value = &KEY_INFO)]
    key_info: String,
    #[structopt(short, long, default_value = "Print")]
    out: OutputType,
    #[structopt(short, long)] //default_value = "../fixtures/fixture_data/keyPair.json"
    file: Option<String>,
}

fn keygen<T>(ikm: T, key_info: Option<&[u8]>) -> Result<KeyPair, BadParams>
where
    T: AsRef<[u8]>
{
    let ikm = ikm.as_ref();
    let key_info = key_info.unwrap_or(&[]);
    let init_salt = "BBS-SIG-KEYGEN-SALT-".as_bytes();

    if ikm.len() < 32 {
        return Err(BadParams { 
            cause: format!("Invalid ikm length. Needs to be at least 32 bytes long. Got {}", ikm.len())
        })
    }

    // L = ceil((3 * ceil(log2(r))) / 16)
    const L: usize = 48;
    const L_BYTES: [u8; 2] = (L as u16).to_be_bytes();

    // salt = H(salt)
    let mut hasher = Sha256::new();
    hasher.update(init_salt);
    let salt = hasher.finalize();

    // PRK = HKDF-Extract(salt, IKM || I2OSP(0, 1))
    let prk = Hkdf::<Sha256>::new(
        Some(&salt),
        &[ikm, &[0u8; 1][..]].concat()
    );

    // OKM = HKDF-Expand(PRK, key_info || I2OSP(L, 2), L)
    let mut okm = [0u8; 64];

    prk.expand(
        &[&key_info, &L_BYTES[..]].concat(),
        &mut okm[(64-L)..]
    ).expect(
        &format!("The HKDF-expand output cannot be more than {} bytes long", 255 * Sha256::output_size())
    );

    okm.reverse(); // okm is in be format
    let sk = Scalar::from_bytes_wide(&okm);
    let pk: G2Projective = G2Affine::generator() * sk;
    let pk_affine = pk.to_affine();

    // transform secret key from le to be
    let mut sk_bytes = sk.to_bytes();
    sk_bytes.reverse();

    Ok(KeyPair 
        {
            secretKey: hex::encode(sk_bytes),
            publicKey: hex::encode(pk_affine.to_compressed())
        }
    )
}

fn main() {
    let opt: Opt = Opt::from_args();

    let ikm = opt.ikm;
    let key_info = opt.key_info;

    let key_pair = keygen(
        &hex::decode(&ikm).unwrap(),
        Some(&hex::decode(&key_info).unwrap())
    ).expect("key generation failed");

    match opt.out {
        OutputType::Print => {println!("key pair = {:?}", key_pair)},
        OutputType::File => write_keypair_to_file(&ikm, Some(&key_info), key_pair, opt.file)
    }
}

// wright to file
fn write_keypair_to_file(ikm: &str, key_info: Option<&str>, key_pair: KeyPair, file: Option<String>)
{
    println!("writhing to file...");

    #[derive(Deserialize, Serialize, Debug)]
    #[allow(non_snake_case)]
    struct FileToWrite<'a> {
        ikm: &'a str,
        keyInfo: &'a str,
        keyPair: KeyPair
    }

    let key_pair_to_write: FileToWrite = FileToWrite { 
        ikm,
        keyInfo: key_info.unwrap_or(&""),
        keyPair: key_pair
    };

    let file = file.unwrap_or(String::from("../fixtures/fixture_data/keyPair.json"));
    let current_path = env::current_dir().unwrap();
    let file_to_write = current_path.join(file);

    std::fs::write(
        &file_to_write, 
        serde_json::to_string_pretty(
            &key_pair_to_write
        ).expect("failed to serializing key pair")
    ).expect(&format!("failed to write key pair to file: {}", file_to_write.to_str().unwrap()));
}