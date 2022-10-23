use bls12_381_plus::{ExpandMsg, G1Projective, G2Projective, Scalar};
use ff::Field;
use group::{Curve};
use structopt::StructOpt;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use serde::ser::{Serialize, Serializer, SerializeStruct};

mod ciphersuites;
use ciphersuites::{BbsCiphersuite, Bls12381Shake256, Bls12381Sha256};

struct Generators {
    g1_base_point: G1Projective,
    q1: G1Projective,
    q2: G1Projective,
    message_generators: Vec<G1Projective>
}

impl Serialize for Generators {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let result: Vec<String> = self.message_generators.iter()
            .map(|item| hex::encode(item.to_affine().to_compressed())).collect();

        let mut state = serializer.serialize_struct("Generators", 4)?;
        state.serialize_field("BP",
            &hex::encode(self.g1_base_point.to_affine().to_compressed()))?;

        state.serialize_field("Q1",
            &hex::encode(self.q1.to_affine().to_compressed()))?;
        state.serialize_field("Q2", 
            &hex::encode(self.q2.to_affine().to_compressed()))?;

        state.serialize_field("MsgGenerators", &result)?;
        state.end()
    }
}

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short, long, default_value = "Shake")]
    suite: Ciphersuite,
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

#[derive(Debug)]
enum Ciphersuite {
    SHA256,
    SHAKE256
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

impl std::str::FromStr for Ciphersuite {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "sha" | "sha2" | "sha25" | "sha256" | "xmd" => Ok(Ciphersuite::SHA256),
            "shake" | "shake2" | "shake25" | "shake256" | "xof" => Ok(Ciphersuite::SHAKE256),
            _ => Err("Invalid Value".to_string())
        }
    }
}


fn main() {
    let opt: Opt = Opt::from_args();

    // Suite specific create generators function
    let get_generators_fn = match opt.suite {
        Ciphersuite::SHAKE256 => make_generators::<Bls12381Shake256>,
        Ciphersuite::SHA256 => make_generators::<Bls12381Sha256>,
    };

    let generators = match opt.generator_type {
        GenType::Global => global_generators(get_generators_fn, opt.length),
        GenType::SignerSpecific => signer_specific_generators(get_generators_fn, opt.length),
    };

    match opt.out_type {
        OutputType::Print => print_generators(&generators),
        OutputType::File => write_generators_to_file(&generators, opt.file_name.unwrap())
    }
}

fn global_generators<F>(make_generators_fn: F, len: usize) -> Generators
where
    F: for<'r> Fn(Option<&'r [u8]>, usize) -> Generators
{
    make_generators_fn(None, len)
}

fn signer_specific_generators<F>(make_generators_fn: F, len: usize) -> Generators
where
    F: for<'r> Fn(Option<&'r [u8]>, usize) -> Generators
{
    let sk = Scalar::random(rand::thread_rng());
    let pk = G2Projective::generator() * sk;
    make_generators_fn(Some(&pk.to_affine().to_compressed()), len)
}

fn print_generators(generators: &Generators) {
    println!("G1 BP = {}", hex::encode(
        generators.g1_base_point.to_affine().to_compressed()
    ));

    println!("Q_1 = {}", hex::encode(
        generators.q1.to_affine().to_compressed()
    ));

    println!("Q_2 = {}", hex::encode(
        generators.q2.to_affine().to_compressed()
    ));
    
    generators.message_generators.iter().enumerate().for_each(|(i, g)| {
        println!(
            "G_{} = {}",
            i + 1,
            hex::encode(g.to_affine().to_compressed())
        );
    });
}

fn write_generators_to_file(generators: &Generators, file_name: String) {
    let path = env::current_dir().unwrap();

    let file_path = path.join(file_name);

    let file = File::create(file_path).unwrap();

    let mut writer = BufWriter::new(file);

    serde_json::to_writer_pretty(&mut writer, &generators).unwrap();

    writer.flush().unwrap();
}

fn make_generators<'a, X>(seed: Option<&[u8]>, len: usize) -> Generators
where
    X: BbsCiphersuite<'a>
{
    const EXPAND_LEN: usize = 48; //TODO make a property of X or calculate?
    let default_seed = &X::generator_seed();
    let seed = seed.unwrap_or(default_seed);

    let base_point = make_g1_base_point::<X>();
    let mut generators = Vec::new();

    let mut v = vec!(0u8; EXPAND_LEN);
    let mut buffer = vec!(0u8; EXPAND_LEN);
    X::Expander::expand_message(seed, &X::generator_seed_dst(), &mut v);

    let mut n = 1u32;
    while generators.len() < len {
        v.append(n.to_be_bytes().to_vec().as_mut());
        X::Expander::expand_message(&v, &X::generator_seed_dst(), &mut buffer);
        v = buffer.clone();
        n += 1;
        let candidate = G1Projective::hash::<<X as BbsCiphersuite>::Expander>(&v, &X::generator_dst());
        if !generators.contains(&candidate) {
            generators.push(candidate);
        }
    }

    Generators {
        g1_base_point: base_point,
        q1: generators[0],
        q2: generators[1],
        message_generators: generators[2..].to_vec()
    }
}

fn make_g1_base_point<'a, X>() -> G1Projective
where
    X: BbsCiphersuite<'a>
{
    let mut v = [0u8; 48];
    X::Expander::expand_message(&X::bp_generator_seed(), &X::generator_seed_dst(), &mut v);

    // TODO: implement a proper I2OSP
    let extra = 1u32.to_be_bytes().to_vec();
    let buffer = [v.as_ref(), &extra].concat();

    X::Expander::expand_message(&buffer, &X::generator_seed_dst(), &mut v);

    G1Projective::hash::<<X as BbsCiphersuite>::Expander>(
        &v, &X::generator_dst()
    )
}