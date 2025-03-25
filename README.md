# osu2itg

A tool to convert osu!std beatmaps to ITG charts

## `.osu` Fields

Documentation for fields in the `.osu` file format is well documented and can be found [here](https://osu.ppy.sh/wiki/en/Client/File_formats/osu_%28file_format%29).

## `.ssc/.sm` Fields

- `#BPMS:<start-beat>=<bpm>, ...;` &#8594; `start-beat = time-signature*measure-count`

## Run Locally

Currently, osu2itg is **only supported for Windows platform**

To run osu2itg locally, first install Rust and Cargo (guide can be found [here](https://doc.rust-lang.org/cargo/getting-started/installation.html))

Once installed, run the following commands in your local terminal:

Windows Powershell

```[bash]
git clone https://github.com/caseycapetola/osu2itg.git
cd osu2itg
cargo run
```

Inside the interface, click "Select .osu File", then select the .osu file you would like to convert to a .ssc file. Configure the offset (or you can do this in your stepchart editor), then click "Convert to SSC". This will create a new file in the same directory as your .osu file.

_Note_: The current version of osu2itg only supports osu! charts with a constant BPM. **Variable BPM charts will not convert properly**.
