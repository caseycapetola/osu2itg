# osu2itg

A tool to convert osu!std beatmaps to ITG charts

## `.osu` Fields

Documentation for fields in the `.osu` file format is well documented and can be found [here](https://osu.ppy.sh/wiki/en/Client/File_formats/osu_%28file_format%29).

## `.ssc/.sm` Fields

- `#BPMS:<start-beat>=<bpm>, ...;` &#8594; `start-beat = time-signature*measure-count`
