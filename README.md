# uesave for PalWorld

This is a fork of [trumank/uesave-rs](https://github.com/trumank/uesave-rs), with hacks to fully support serialization/deserialization of PalWorld savs, including **Level.sav**

Note that this is not a general-purpose update, it should not be merged back into uesave-rs.

```sh
$ uesave to-json -h
Convert binary save to plain text JSON

Usage: uesave to-json [OPTIONS]

Options:
  -i, --input <INPUT>    [default: -]
  -o, --output <OUTPUT>  [default: -]
  -t, --type <TYPE>      Save files do not contain enough context to parse structs inside MapProperty or SetProperty. uesave will attempt to guess, but if it is incorrect the save will fail to parse and the type must be manually specified
  -p, --preset <PRESET>  [default: palworld]
      --pretty           
  -h, --help             Print help (see more with '--help')
```

### Partial example of the output (RawData: Level/HP/MaxHP/CraftSpeeds, etc.)

```json
"root":{"save_game_type":"/Script/Pal.PalWorldSaveGame","properties":{"Version":{"Int":{"value":100}},"Timestamp":{"Struct":{"value":{"DateTime":638419313583420000},"struct_type":"DateTime","struct_id":"00000000-0000-0000-0000-000000000000"}},"worldSaveData":{"Struct":{"value":{"Struct":{"CharacterSaveParameterMap":{"Map":{"key_type":"StructProperty","value_type":"StructProperty","value":[{"key":{"Struct":{"Struct":{"PlayerUId":{"Struct":{"value":{"Guid":"dfa44d81-0000-0000-0000-000000000000"},"struct_type":"Guid","struct_id":"00000000-0000-0000-0000-000000000000"}},"InstanceId":{"Struct":{"value":{"Guid":"3f655adc-d6c1-4ada-baec-efedc9284f43"},"struct_type":"Guid","struct_id":"00000000-0000-0000-0000-000000000000"}},"DebugName":{"Str":{"value":""}}}}},"value":{"Struct":{"Struct":{"RawData":{"Struct":{"value":{"RawDataParsed":{"props":{"SaveParameter":{"Struct":{"value":{"Struct":{"Level":{"Int":{"value":50}},"NickName":{"Str":{"value":"DKingAlpha"}},"HP":{"Struct":{"value":{"Struct":{"Value":{"Int64":{"value":11100000}}}},"struct_type":{"Struct":"FixedPoint64"},"struct_id":"00000000-0000-0000-0000-000000000000"}},"FullStomach":{"Float":{"value":58.272976}},"IsPlayer":{"Bool":{"value":true}},"MaxHP":{"Struct":{"value":{"Struct":{"Value":{"Int64":{"value":11100000}}}},"struct_type":{"Struct":"FixedPoint64"},"struct_id":"00000000-0000-0000-0000-000000000000"}},"Support":{"Int":{"value":100}},"CraftSpeed":{"Int":{"value":3000}},"CraftSpeeds":{"Array":{"array_type":"StructProperty","value":{"Struct":{"_type":"CraftSpeeds","name":"StructProperty","struct_type":{"Struct":"PalWorkSuitabilityInfo"},"id":"00000000-0000-0000-0000-
```

## upstream README

[![docs.rs](https://img.shields.io/docsrs/uesave)](https://docs.rs/uesave)
[![Crates.io](https://img.shields.io/crates/v/uesave)](https://crates.io/crates/uesave)

A library for reading and writing Unreal Engine save files (commonly referred to
as GVAS).

It has been tested on an extensive set of object structures and can fully read
and write Deep Rock Galactic save files (and likely a lot more).

There is a small binary utility to quickly convert saves to and from a plain
text JSON format which can be used for manual save editing.

## Usage

```sh
$ cargo install --git https://github.com/DKingAlpha/palworld-uesave-rs
$ uesave --help
Usage: uesave <COMMAND>

Commands:
  to-json      Convert binary save to plain text JSON
  from-json    Convert JSON back to binary save
  edit         Launch editor to edit a save file as JSON in place
  test-resave  Test resave
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

![edit](https://user-images.githubusercontent.com/1144160/210157064-234da188-ad20-416f-9ea5-7d2956168a20.svg)

## Alternative projects
in no particular order:
- https://github.com/rob0rt/drg-save-parser
- https://github.com/ch1pset/UESaveTool
- https://github.com/13xforever/gvas-converter
- https://github.com/RagingLightning/gvas-converter
- https://github.com/SparkyTD/UnrealEngine.Gvas
- https://github.com/localcc/gvas
- https://github.com/oberien/gvas-rs
- https://github.com/scottanderson/railroad.studio
- https://github.com/CrystalFerrai/UeSaveGame
- https://github.com/agc93/unsave
