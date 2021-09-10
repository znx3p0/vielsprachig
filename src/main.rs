
use std::{io::{Read, Write, stdin, stdout}, path::PathBuf};
use structopt::StructOpt;
use structopt::clap::arg_enum;
use erased_serde::Serialize;

#[derive(StructOpt, Debug)]
#[structopt(name = "vsp")]
struct Opts {
    #[structopt(flatten)]
    file_cmd: FileCmd
}

#[derive(StructOpt, Debug)]
#[structopt(name = "vsp_main")]
struct FileCmd {
    // if no input is given, then switch to console mode
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,
    #[structopt(parse(from_os_str))]
    output: Option<PathBuf>,
    #[structopt(short, long, possible_values = &FromVariant::variants(), case_insensitive = true)]
    from: Option<FromVariant>,
    #[structopt(short, long, possible_values = &ToVariant::variants(), case_insensitive = true)]
    to: Option<ToVariant>,
}

arg_enum! {
    #[derive(Debug, Clone, Copy)]
    enum FromVariant {
        Json,
        Yaml,
        Cbor,
        Ron,
        Toml,
        Bson,
    }
}

arg_enum! {
    #[derive(Debug, Clone, Copy)]
    enum ToVariant {
        Pickle,
        Bincode,
        Postcard,
        Flexbuffers,
        Json,
        PrettyJson,
        Yaml,
        Cbor,
        Ron,
        PrettyRon,
        Toml,
        Bson,
    }
}

impl FromVariant {
    fn serialize<T>(&self, input: Vec<u8>, s: T)
    where T: Fn(&dyn Serialize)
    {
        match self {
            FromVariant::Json => {
                let v: serde_json::Value = serde_json::from_slice(&input).unwrap();
                s(&v)
            },
            FromVariant::Yaml => {
                let v: serde_yaml::Value = serde_yaml::from_slice(&input).unwrap();
                s(&v)
            },
            FromVariant::Cbor => {
                let v: serde_cbor::Value = serde_cbor::from_slice(&input).unwrap();
                s(&v)
            },
            FromVariant::Ron => {
                let v: ron::Value = ron::de::from_bytes(&input).unwrap();
                s(&v)
            },
            FromVariant::Toml => {
                let v: toml::Value = toml::from_slice(&input).unwrap();
                s(&v)
            },
            FromVariant::Bson => {
                let v: bson::Bson = bson::from_slice(&input).unwrap();
                s(&v)
            },
        }
    }
}

impl From<&PathBuf> for FromVariant {
    fn from(path: &PathBuf) -> Self {
        let p = path.extension().expect("Extension not found, the type of the file could not be inferred.");
        match p.to_str().unwrap() {
            "bson" | "bs" => FromVariant::Bson,
            "cbor" | "cb" => FromVariant::Cbor,
            "json" => FromVariant::Json,
            "ron"  => FromVariant::Ron,
            "toml" => FromVariant::Toml,
            "yaml" | "yml" => FromVariant::Yaml,
            _ => panic!("Type of the file could not be inferred"),
        }
    }
}

impl ToVariant {
    fn to_buf(self, obj: &dyn Serialize) -> Vec<u8> {
        match self {
            ToVariant::Pickle => {
                serde_pickle::to_vec(&obj, true).unwrap()
            },
            ToVariant::Bincode => {
                bincode::serialize(&obj).unwrap()
            },
            ToVariant::Postcard => {
                postcard::to_allocvec(&obj).unwrap()
            },
            ToVariant::Flexbuffers => {
                flexbuffers::to_vec(&obj).unwrap()
            },
            ToVariant::Json => {
                serde_json::to_vec(&obj).unwrap()
            },
            ToVariant::PrettyJson => {
                serde_json::to_vec_pretty(&obj).unwrap()
            },
            ToVariant::Yaml => {
                serde_yaml::to_vec(&obj).unwrap()
            },
            ToVariant::Cbor => {
                serde_cbor::to_vec(&obj).unwrap()
            },
            ToVariant::Ron => {
                let s = ron::to_string(&obj).unwrap();
                s.into_bytes()
            },
            ToVariant::PrettyRon => {
                let s = ron::ser::PrettyConfig::new();
                let s = ron::ser::to_string_pretty(&obj, s).unwrap();
                s.into_bytes()
            },
            ToVariant::Toml => {
                toml::to_vec(&obj).unwrap()
            },
            ToVariant::Bson => {
                bson::to_vec(&obj).unwrap()
            },
        }
    }
}

impl From<&PathBuf> for ToVariant {
    fn from(path: &PathBuf) -> Self {
        let p = path.extension().expect("Extension not found, the type of the file could not be inferred.");
        match p.to_str().unwrap() {
            "bincode" | "bc" => Self::Bincode,
            "bson" | "bs" => Self::Bson,
            "cbor" | "cb" => Self::Cbor,
            "yaml" | "yml" => Self::Yaml,
            "flexbuffers" | "fb" => Self::Flexbuffers,
            "postcard" | "pc" => Self::Postcard,
            "pickle" | "pkl" => Self::Pickle,
            "json" => Self::Json,
            "hjson" => Self::PrettyJson,
            "ron" => Self::Ron,
            "hron" => Self::PrettyRon,
            "toml" => Self::Toml,
            _ => panic!("Type of the file could not be inferred"),
        }
    }
}

fn main() {
    let opts = Opts::from_args();
    match opts.file_cmd.input {
        Some(inp) => {
            let input = std::fs::read(&inp).unwrap();
            let output = opts.file_cmd.output.clone().unwrap();
            let variant = opts.file_cmd.to.unwrap_or(ToVariant::from(&output));
            opts.file_cmd.from.unwrap_or(FromVariant::from(&inp)).clone().serialize(input, |obj| {
                let buf = variant.to_buf(obj);
                std::fs::write(&output, buf).unwrap();
            });
        },
        None => {
            let mut input = vec![];
            let from = opts.file_cmd.from.unwrap_or_else(|| {
                println!("Called with no arguments. Use --help for the manual");
                std::process::exit(1)
            });
            let to = opts.file_cmd.to.expect("Cannot infer type. Please specify a type with -t");
            stdin().lock().read_to_end(&mut input).unwrap();
            from.serialize(input, |obj| {
                let buf = to.to_buf(obj);
                stdout().lock().write(&buf).unwrap();
            })
        },
    }
}
