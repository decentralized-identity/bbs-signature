use std::{env, fmt, str::FromStr};

use bls12_381::{
    hash_to_curve::{ExpandMessage, ExpandMsgXmd, ExpandMsgXof, HashToField},
    G2Projective, Scalar,
};
use ff::Field;
use group::Curve;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use sha3::Shake256;
use structopt::StructOpt;

const DEFAULT_IKM_HEX: &str = "746869732d49532d6a7573742d616e2d546573742d494b4d2d746f2d67656e65726174652d246528724074232d6b6579";
const DEFAULT_CIPHERSUITE: &str = "sha256";

/// Processing error
#[derive(Debug, Clone)]
struct Invalid(&'static str);

impl fmt::Display for Invalid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid parameters: {c}", c = self.0)
    }
}

#[derive(Debug, Clone)]
struct SecretKey(pub Scalar);

impl SecretKey {
    pub fn to_bytes(&self) -> [u8; 32] {
        let mut sk_bytes = self.0.to_bytes();
        // transform secret key from LE to BE
        sk_bytes.reverse();
        sk_bytes
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
struct KeyPair {
    secretKey: String,
    publicKey: String,
}

impl From<&SecretKey> for KeyPair {
    fn from(sk: &SecretKey) -> Self {
        // transform secret key from LE to BE
        let sk_bytes = sk.to_bytes();
        let pk = G2Projective::generator() * sk.0;
        let pk_bytes = pk.to_affine().to_compressed();

        Self {
            secretKey: hex::encode(sk_bytes),
            publicKey: hex::encode(pk_bytes),
        }
    }
}

fn hash_to_scalar<X: ExpandMessage>(msg_octets: &[u8], dst: &[u8]) -> Result<Scalar, Invalid> {
    let mut counter: usize = 0;
    let mut hashed_scalar = Scalar::zero();
    let mut msg_prime = Vec::with_capacity(msg_octets.len() + 1);
    while hashed_scalar.is_zero().into() {
        if counter > 255 {
            return Err(Invalid("Exceeded hash_to_scalar counter"));
        }
        msg_prime.clear();
        msg_prime.extend_from_slice(&msg_octets);
        msg_prime.push(counter as u8);
        Scalar::hash_to_field::<X>(&msg_prime, dst, std::slice::from_mut(&mut hashed_scalar));
        counter += 1;
    }
    Ok(hashed_scalar)
}

fn keygen<C: Ciphersuite>(
    key_material: &[u8],
    key_info: Option<&[u8]>,
    key_dst: Option<&[u8]>,
) -> Result<KeyPair, Invalid> {
    let key_info = key_info.unwrap_or_default();
    let key_dst = key_dst.unwrap_or(C::DST.as_bytes());

    if key_material.len() < 32 {
        return Err(Invalid(
            "Invalid key material length. Must be at least 32 bytes.",
        ));
    }
    if key_info.len() > 65535 {
        return Err(Invalid(
            "Invalid key info length. Must be less than 65536 bytes.",
        ));
    }

    let mut derive_input = Vec::with_capacity(key_material.len() + 2 + key_info.len());
    derive_input.extend_from_slice(key_material);
    derive_input.extend_from_slice(&(key_info.len() as u16).to_be_bytes());
    derive_input.extend_from_slice(key_info);

    let sk = SecretKey(hash_to_scalar::<C::Expander>(&derive_input, key_dst)?);
    Ok(KeyPair::from(&sk))
}

trait Ciphersuite {
    const DST: &'static str;

    type Expander: ExpandMessage;
}

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
struct Bls12381_XmdSha256;

impl Ciphersuite for Bls12381_XmdSha256 {
    const DST: &'static str = "BBS_BLS12381G1_XMD:SHA-256_SSWU_RO_KEYGEN_DST_";

    type Expander = ExpandMsgXmd<Sha256>;
}

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
struct Bls12381_XofShake256;

impl Ciphersuite for Bls12381_XofShake256 {
    const DST: &'static str = "BBS_BLS12381G1_XOF:SHAKE-256_SSWU_RO_KEYGEN_DST_";

    type Expander = ExpandMsgXof<Shake256>;
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

#[derive(Debug)]
enum CiphersuiteOpt {
    SHA256,
    SHAKE256,
}

impl FromStr for CiphersuiteOpt {
    type Err = Invalid;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sha256" => Ok(Self::SHA256),
            "shake256" => Ok(Self::SHAKE256),
            _ => Err(Invalid("Unknown ciphersuite name")),
        }
    }
}

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(long, default_value = &DEFAULT_CIPHERSUITE)]
    ciphersuite: CiphersuiteOpt,
    #[structopt(long, default_value = &DEFAULT_IKM_HEX)]
    key_material: String,
    #[structopt(long)]
    key_info: Option<String>,
    #[structopt(long)]
    key_dst: Option<String>,
    #[structopt(short, long, default_value = "Print")]
    out: OutputType,
    #[structopt(short, long)]
    file: Option<String>,
}

fn write_keypair_to_file(
    ikm: &str,
    key_info: Option<&str>,
    key_pair: KeyPair,
    file: Option<String>,
) {
    println!("Writing to file...");

    #[derive(Deserialize, Serialize, Debug)]
    #[allow(non_snake_case)]
    struct FileToWrite<'a> {
        ikm: &'a str,
        keyInfo: &'a str,
        keyPair: KeyPair,
    }

    let key_pair_to_write: FileToWrite = FileToWrite {
        ikm,
        keyInfo: key_info.unwrap_or(&""),
        keyPair: key_pair,
    };

    let file = file.unwrap_or(String::from("../fixtures/fixture_data/keyPair.json"));
    let current_path = env::current_dir().unwrap();
    let file_to_write = current_path.join(file);

    std::fs::write(
        &file_to_write,
        serde_json::to_string_pretty(&key_pair_to_write).expect("failed to serializing key pair"),
    )
    .expect(&format!(
        "Failed to write key pair to file: {}",
        file_to_write.to_str().unwrap()
    ));
}

fn main() {
    let opt: Opt = Opt::from_args();

    let key_material = hex::decode(&opt.key_material).expect("Invalid key material");
    let key_info = opt
        .key_info
        .as_ref()
        .map(|ki| hex::decode(ki).expect("Invalid key info"));
    let key_dst = opt
        .key_dst
        .as_ref()
        .map(|ki| hex::decode(ki).expect("Invalid key DST"));

    let key_pair = match opt.ciphersuite {
        CiphersuiteOpt::SHA256 => {
            keygen::<Bls12381_XmdSha256>(&key_material, key_info.as_deref(), key_dst.as_deref())
        }
        CiphersuiteOpt::SHAKE256 => {
            keygen::<Bls12381_XofShake256>(&key_material, key_info.as_deref(), key_dst.as_deref())
        }
    }
    .expect("key generation failed");

    match opt.out {
        OutputType::Print => {
            println!("key pair = {:?}", key_pair)
        }
        OutputType::File => write_keypair_to_file(
            &opt.key_material,
            opt.key_info.as_deref(),
            key_pair,
            opt.file,
        ),
    }
}
