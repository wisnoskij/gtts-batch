# gtts-batch

Wrapper over gtts to handle large conversions. Batch converting many files, and spitting up large files into multiple smaller files. gtts-cli need to be installed for gtts-batch to function

Usage: gtts-batch [OPTIONS] [FOLDER/FILE]

Arguments:
  [FOLDER/FILE]
          Folder to convert
          
          [default: .]

Options:
  -f, --from <FILE>
          Start processing at FILE.txt alpha numerically (inclusive). [TODO]

  -t, --to <FILE>
          Stop processing at FILE.txt alpha numerically (inclusive). [TODO]

  -o, --overwrite
          Overwrite existing mp3 files instead of skipping

  -r, --recurse
          Recursively go into directories

  -s, --shutdown
          Shutdown system after finishing. [TODO]

  -n, --normalize
          Remove non alphanumeric characters and normal punctuation. [TODO]

  -a, --abbreviations
          Fix troublesome abbreviations. [TODO]
          
          "LV" is considered some currency, so I fix that as well as other level abreviations.

  -w, --wait <MINUTES>
          The <MINUTES> to wait in minutes
          
          [default: 5]

  --waitms <MILI>
          The <MILI> to wait in miliseconds (1/1000th of a second).
          
          wait<time in munutes> is ignored if this argument is present.

  --split <STRING>
          Split file(s) at every occurance of <STRING>. [TODO]
          
          <STRING> begins the split. This happens first, before checking for max length

  -m, --max <BYTES>
          The max length in bytes a single file can be before it gets split. [TODO]: Figure out if I am splitting by character or byte
          
          [default: 40000]

  --splitstr <STRING>
          The string(s) to split at. [TODO]
          
          Tries to split at first string, if this fails moves to second and so on. If all fail, just splits at the exact character. Split happens after STRING.
          
          [default: "\n\n" "\n" .]

  -h, --help
          Print help information (use `-h` for a summary)

  -V, --version
          Print version information
