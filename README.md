# Vielsprachig | vsp

### Command utility that translates file formats.


```bash
# transpiles Cargo.toml to json and writes it under cargo.json
# the file formats are inferred, but they can be overriden.
vsp Cargo.toml cargo.json

# the f stands for `from` and t stands for `to`
vsp Cargo.toml cargo.yaml -f json -t yaml

# vsp can also be piped if no input and output is provided
# file formats need to be specified since they cannot be inferred.
cat Cargo.toml | vsp -f toml -t yaml | cargo.yaml
```

The current input options and their inferred extensions are:
- Json
  - .json
- Yaml
  - .yaml or .yml
- Cbor
  - .cb or .cbor
- Ron
  - .ron
- Toml
  - .toml
- Bson
  - .bson or .bs

The current output options are:
- Pickle
  - .pickle or .pkl
- Bincode
  - .bc or .bincode
- Postcard
  - .pc or .postcard
- Flexbuffers
  - .fb or .flexbuffers
- Json
  - .json
- PrettyJson
  - .hjson
- Yaml
  - .yaml or .yml
- Cbor
  - .cbor or .cb
- Ron
  - .ron
- PrettyRon
  - .hron
- Toml
  - .toml
- Bson
  - .bs or .bson
