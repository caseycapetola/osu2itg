# osu2itg

A tool to convert osu!std beatmaps to ITG charts. osu2itg will parse through an osu!std beatmap and convert it into a playable ITG chart.

<!-- ## `.osu` Fields

Documentation for fields in the `.osu` file format is well documented and can be found in the [osu! wiki](https://osu.ppy.sh/wiki/en/Client/File_formats/osu_%28file_format%29). -->

## Run Locally

To run osu2itg locally, first install Rust and Cargo (guide can be found in the [Rust documentation](https://doc.rust-lang.org/cargo/getting-started/installation.html))

Once installed, run the following commands in your local terminal:

For Mac/Linux/Windows Powershell:

```[bash]
git clone https://github.com/caseycapetola/osu2itg.git
cd osu2itg
cargo run -- <INPUT_FILEPATH>
```

Where `INPUT_FILEPATH` is the absolute path of the `.osu` file you would like to convert.

The `.ssc` output file will be written into the same directory as the `.osu` file.

## Known Limitations

For songs with a high number of BPM changes, it is possible that the chart will become out of sync with the song. Some manual tuning of offsets may be required.
