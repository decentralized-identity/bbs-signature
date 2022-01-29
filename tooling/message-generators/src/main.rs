use bls12_381_plus::{ExpandMsgXof, G1Projective, G2Projective, Scalar};
use ff::Field;
use group::Curve;
use sha3::digest::{ExtendableOutput, Update, XofReader};
use sha3::Shake256;
use structopt::StructOpt;

const GLOBAL_SEED: &[u8] =
    b"Cowards die many times before their deaths; The valiant never taste of death but once.";
const GLOBAL_DST: &[u8] = b"BBS_SETUP_GENERATOR_IKM_1_0_0\0\0\0";
const DST: &[u8] = b"BBS_BLS12381G1_XOF:SHAKE-256_SSWU_RO_";

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short, long, default_value = "10")]
    length: usize,
    #[structopt(short, long, default_value = "Global")]
    generator_type: GenType,
}

#[derive(Debug)]
enum GenType {
    Global,
    SignerSpecific,
}

impl std::str::FromStr for GenType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "g" | "gl" | "glo" | "glob" | "globa" | "global" => Ok(GenType::Global),
            "s" | "si" | "sig" | "sign" | "signe" | "signer" => Ok(GenType::SignerSpecific),
            _ => Err("Invalid Value".to_string()),
        }
    }
}

fn main() {
    let opt: Opt = Opt::from_args();
    match opt.generator_type {
        GenType::Global => global_generators(opt.length),
        GenType::SignerSpecific => signer_specific_generators(opt.length),
    }
}

fn global_generators(len: usize) {
    let generators = make_generators(GLOBAL_SEED, len);
    print_generators(&generators);
}

fn signer_specific_generators(len: usize) {
    let sk = Scalar::random(rand::thread_rng());
    let pk = G2Projective::generator() * sk;
    let generators = make_generators(&pk.to_affine().to_compressed(), len);
    print_generators(&generators);
}

fn print_generators(generators: &[G1Projective]) {
    generators.iter().enumerate().for_each(|(i, g)| {
        println!(
            "G_{} = {}",
            i + 1,
            hex::encode(g.to_affine().to_compressed())
        );
    });
}

fn make_generators(seed: &[u8], len: usize) -> Vec<G1Projective> {
    let mut reader = Shake256::default()
        .chain(GLOBAL_DST)
        .chain(seed)
        .finalize_xof();

    let mut generators = Vec::new();
    let mut buffer = [0u8; 64];
    for _ in 0..len {
        reader.read(&mut buffer);
        let gi = G1Projective::hash::<ExpandMsgXof<Shake256>>(&buffer, DST);
        generators.push(gi);
    }
    generators
}
