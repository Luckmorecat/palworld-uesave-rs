use std::fs::{self, File, OpenOptions};
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use uesave::{Property, Save, StructType, StructValue, Types};

#[derive(Debug, Clone)]
enum ActionPreset {
    None,
    Palworld,
}

impl ActionPreset {
    fn get_types(&self) -> Types {
        match self {
            Self::None => Types::new(),
            Self::Palworld => {
                let mut types = Types::new();
                for st in vec![
                    ".worldSaveData.CharacterSaveParameterMap.Key",
                    ".worldSaveData.FoliageGridSaveDataMap.Key",
                    ".worldSaveData.CharacterSaveParameterMap.Value",
                    ".worldSaveData.FoliageGridSaveDataMap.ModelMap.InstanceDataMap.Key",
                    ".worldSaveData.MapObjectSpawnerInStageSaveData.Key",
                    ".worldSaveData.ItemContainerSaveData.Key",
                    ".worldSaveData.CharacterContainerSaveData.Key",
                    ".worldSaveData.FoliageGridSaveDataMap.Value.ModelMap.Value",
                    ".worldSaveData.FoliageGridSaveDataMap.Value.ModelMap.Value.InstanceDataMap.Key",
                    ".worldSaveData.FoliageGridSaveDataMap.Value.ModelMap.Value.InstanceDataMap.Value",
                    ".worldSaveData.FoliageGridSaveDataMap.Value",
                    ".worldSaveData.MapObjectSaveData.MapObjectSaveData.ConcreteModel.ModuleMap.Value",
                    ".worldSaveData.MapObjectSaveData.MapObjectSaveData.Model.EffectMap.Value",
                    ".worldSaveData.MapObjectSpawnerInStageSaveData.Value",
                    ".worldSaveData.MapObjectSpawnerInStageSaveData.Value.SpawnerDataMapByLevelObjectInstanceId.Value",
                    ".worldSaveData.MapObjectSpawnerInStageSaveData.Value.SpawnerDataMapByLevelObjectInstanceId.Value.ItemMap.Value",
                    ".worldSaveData.WorkSaveData.WorkSaveData.WorkAssignMap.Value",
                    ".worldSaveData.BaseCampSaveData.Value",
                    ".worldSaveData.BaseCampSaveData.Value.ModuleMap.Value",
                    ".worldSaveData.ItemContainerSaveData.Value",
                    ".worldSaveData.CharacterContainerSaveData.Value",
                    ".worldSaveData.GroupSaveDataMap.Value",
                    ".worldSaveData.EnemyCampSaveData.EnemyCampStatusMap.Value",
                    ".worldSaveData.DungeonSaveData.DungeonSaveData.MapObjectSaveData.MapObjectSaveData.Model.EffectMap.Value",
                    ".worldSaveData.DungeonSaveData.DungeonSaveData.MapObjectSaveData.MapObjectSaveData.ConcreteModel.ModuleMap.Value"

                ] {
                    types.add(st.to_string(), StructType::Struct(None));
                }
                return types;
            }
        }
    }
}

impl From<&str> for ActionPreset {
    fn from(t: &str) -> Self {
        match t {
            "palworld" => Self::Palworld,
            _ => Self::None,
        }
    }
}

#[derive(Parser, Debug)]
struct ActionToJson {
    #[arg(short, long, default_value = "-")]
    input: String,

    #[arg(short, long, default_value = "-")]
    output: String,

    /// Save files do not contain enough context to parse structs inside MapProperty or SetProperty.
    /// uesave will attempt to guess, but if it is incorrect the save will fail to parse and the
    /// type must be manually specified.
    ///
    /// Examples:
    ///   -t .UnlockedItemSkins.Skins=Guid
    ///   -t .EnemiesKilled.Key=Guid
    ///   -t .EnemiesKilled.Value=Struct
    #[arg(short, long, value_parser = parse_type)]
    r#type: Vec<(String, StructType)>,

    #[arg(short, long, default_value = "palworld")]
    preset: ActionPreset,

    #[arg(long, default_value = "false")]
    pretty: bool,
}

#[derive(Parser, Debug)]
struct ActionFromJson {
    #[arg(short, long, default_value = "-")]
    input: String,

    #[arg(short, long, default_value = "-")]
    output: String,
}

#[derive(Parser, Debug)]
struct ActionTestResave {
    #[arg(required = true, index = 1)]
    path: String,

    /// If resave fails, write input.sav and output.sav to working directory for debugging
    #[arg(short, long)]
    debug: bool,

    /// Save files do not contain enough context to parse structs inside MapProperty or SetProperty.
    /// uesave will attempt to guess, but if it is incorrect the save will fail to parse and the
    /// type must be manually specified.
    ///
    /// Examples:
    ///   -t .UnlockedItemSkins.Skins=Guid
    ///   -t .EnemiesKilled.Key=Guid
    ///   -t .EnemiesKilled.Value=Struct
    #[arg(short, long, value_parser = parse_type)]
    r#type: Vec<(String, StructType)>,
}

#[derive(Subcommand, Debug)]
enum Action {
    /// Convert binary save to plain text JSON
    ToJson(ActionToJson),
    /// Convert JSON back to binary save
    FromJson(ActionFromJson),
    /// Test resave
    TestResave(ActionTestResave),
}

#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    #[command(subcommand)]
    action: Action,
}

fn parse_type(t: &str) -> Result<(String, StructType)> {
    if let Some((l, r)) = t.rsplit_once('=') {
        Ok((l.to_owned(), r.into()))
    } else {
        Err(anyhow!("Malformed type"))
    }
}

pub fn main() -> Result<()> {
    let args = Args::parse();

    match args.action {
        Action::ToJson(action) => {
            let mut types = action.preset.get_types();
            for (path, t) in action.r#type {
                types.add(path, t);
            }

            let save = Save::read_with_types(&mut input(&action.input)?, &types)?;




            
            let val = match &save.root.properties["worldSaveData"] {
                Property::Struct { id, value, struct_type, struct_id } => value,
                _ => panic!("error 1.")
            };

            let res = match &val {
                StructValue::Struct(val) => &val["CharacterSaveParameterMap"],
                _ => panic!("error 2.")
            };


            if action.pretty {
                serde_json::to_writer_pretty(output(&action.output)?, &res)?;
            } else {
                serde_json::to_writer(output(&action.output)?, &res)?;
            }
        }
        Action::FromJson(io) => {
            let save: Save = serde_json::from_reader(&mut input(&io.input)?)?;
            save.write(&mut output(&io.output)?)?;
        }
        Action::TestResave(action) => {
            let mut types = Types::new();
            for (path, t) in action.r#type {
                types.add(path, t);
            }

            let mut input = std::io::Cursor::new(fs::read(action.path)?);
            let mut output = std::io::Cursor::new(vec![]);
            Save::read_with_types(&mut input, &types)?.write(&mut output)?;
            let (input, output) = (input.into_inner(), output.into_inner());
            if input != output {
                if action.debug {
                    fs::write("input.sav", input)?;
                    fs::write("output.sav", output)?;
                }
                return Err(anyhow!("Resave did not match"));
            }
            println!("Resave successful");
        }
    }
    Ok(())
}

fn input<'a>(path: &str) -> Result<Box<dyn BufRead + 'a>> {
    Ok(match path {
        "-" => Box::new(BufReader::new(stdin().lock())),
        p => Box::new(BufReader::new(File::open(p)?)),
    })
}

fn output<'a>(path: &str) -> Result<Box<dyn Write + 'a>> {
    Ok(match path {
        "-" => Box::new(BufWriter::new(stdout().lock())),
        p => Box::new(BufWriter::new(
            OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(p)?,
        )),
    })
}
