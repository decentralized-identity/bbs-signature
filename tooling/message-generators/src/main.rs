use bls12_381_plus::{ExpandMsgXof, G1Projective, G2Projective, Scalar};
use ff::Field;
use group::Curve;
use sha3::digest::{ExtendableOutput, Update, XofReader};
use sha3::Shake256;
use structopt::StructOpt;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};

const GLOBAL_SEED: &[u8] =
    b"BBS_BLS12381G1_XOF:SHAKE-256_SSWU_RO_MESSAGE_GENERATOR_SEED";
const DST: &[u8] = b"BBS_BLS12381G1_XOF:SHAKE-256_SSWU_RO_";

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short, long, default_value = "10")]
    length: usize,
    #[structopt(short, long, default_value = "Global")]
    generator_type: GenType,
    #[structopt(short, default_value = "Print")]
    out_type: OutputType,
    #[structopt(required_if("out-type", "file"))]
    file_name: Option<String>,
}

#[derive(Debug)]
enum OutputType {
    Print,
    File,
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

fn main() {
    let opt: Opt = Opt::from_args();
    
    let generators = match opt.generator_type {
        GenType::Global => global_generators(opt.length),
        GenType::SignerSpecific => signer_specific_generators(opt.length),
    };

    match opt.out_type {
        OutputType::Print => print_generators(&generators),
        OutputType::File => write_generators_to_file(&generators, opt.file_name.unwrap())
    }
}

fn global_generators(len: usize) -> Vec<G1Projective> {
    make_generators(GLOBAL_SEED, len)
}

fn signer_specific_generators(len: usize) -> Vec<G1Projective> {
    let sk = Scalar::random(rand::thread_rng());
    let pk = G2Projective::generator() * sk;
    make_generators(&pk.to_affine().to_compressed(), len)
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

fn write_generators_to_file(generators: &[G1Projective], file_name: String) {
    let path = env::current_dir().unwrap();

    let file_path = path.join(file_name);

    let result: Vec<String> = generators.iter().map(|item| hex::encode(item.to_affine().to_compressed())).collect();

    let file = File::create(file_path).unwrap();

    let mut writer = BufWriter::new(file);

    serde_json::to_writer_pretty(&mut writer, &result).unwrap();

    writer.flush().unwrap();
}

fn make_generators(seed: &[u8], len: usize) -> Vec<G1Projective> {
    let mut reader = Shake256::default()
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
