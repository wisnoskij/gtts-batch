# gtts-batch

Wrapper over gtts to handle large conversions. Batch converting many files. gtts-cli need to be installed for gtts-batch to function

usage: gtts-batch [OPTIONS] [FILE/FOLDER]
CONTROLS:
(Press the key and then enter to confirm)
c, x, q
        Exit the program after the current file is done.
i, n
        Turns off interactive mode, preventing typing from affecting the running of this process.
s
        Toggle if the program will shutdown the computer after finishing.
h
        Print this help information
OPTIONS:
-c, --code
        Supply your own gtts code. "--file <FILE> --output <FILE>_gtts.mp3" will be appended
-n, --nocount
        Don't precount the files that will be converted to audio. [default: false]
-i, --nointeract
        Turn off all interactive options. [default: false]
-o, --overwrite
        Overwrite existing mp3 files. [default: false]
-p, --path=<file/folder>
        The path to the file or folder to run on. [default: .]
-r, --recurse
        Also include subfolders. This can be treated as a boolean or you can specify the max depth of subfolder traversal. [default: false]
-s, --shutdown
        Shutdown computer after finishing. [default: false]
-t, --test
        Run in test mode allowing you to see what files will be converted. [default: false]
-w, --wait=<minutes>
        How long to wait between requests to the server. [default: 5]
-h, --help
        Print help information.
-v, --version
        Print version information.
