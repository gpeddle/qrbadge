# qrbadge

Generate simple event badges from the command line.

## Usage

```
qrbadge -c config_file -i input_file -o output_dir
```

- `config_file` - config/spec file describing the output format
- `input_file` - a tab-separated text file with column headings
- `output_dir` - writeable folder for output image files


## Config files

Example:

```xml
<?xml version ='1.0'?>
<spec xmlns="spec">
    <fileName>{UniqueId}-{LastName}.png</fileName>
    <document orientation="portrait" height="350" width="350" margin="25" units="px">
        <code width="325" height="325">{UniqueId}^{LastName}^{FirstName}</code>
        <text font="Calibri" fontSize="32">{FirstName}</text>
        <text font="Calibri" fontSize="22">{LastName}</text>
        <text font="Calibri" fontSize="14">{Company}</text>
    </document>
</spec>
```

## Running the Example

`cargo run -- --input example/data/data.txt  --output example/out --spec example/spec/badge.xml`
