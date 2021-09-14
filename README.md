# qrbadge

Generate simple event badges from the command line.

## Usage

```
qrbadge --input <input-file> --output <output-dir> --spec <spec-file> 
```


- `input_file` - a tab-separated text file with column headings
- `output_dir` - writeable folder for output image files
- `spec_file` - spec file describing the output image layout/formatting


## Config file format

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

## Data file format

The data file format is tab-delimited, with the first row containing the field names.

## Running the Example

`cargo run -- --input example/data/data.txt  --output example/out --spec example/spec/badge.xml`
