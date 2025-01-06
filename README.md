# gtts-batch   
   
Wrapper over gtts to handle large conversions. Batch converting many files. gtts-cli need to be installed for gtts-batch to function   

usage: gtts-batch [OPTIONS] [FILE/FOLDER]   
CONTROLS:   
(Press the key and then enter to confirm)   
c, x, q   
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Exit the program after the current file is done.   
i, n   
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Turns off interactive mode, preventing typing from affecting the running of this process.   
s   
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Toggle if the program will shutdown the computer after finishing.   
h   
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Print this help information   
OPTIONS:   
-c, --code   
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Supply your own gtts code. "--file <FILE> --output <FILE>_gtts.mp3" will be appended   
-n, --nocount   
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Don't precount the files that will be converted to audio. [default: false]   
-i, --nointeract   
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Turn off all interactive options. [default: false]   
-o, --overwrite   
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Overwrite existing mp3 files. [default: false]   
-p, --path=`<file/folder>`   
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;The path to the file or folder to run on. [default: .]   
-r, --recurse   
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Also include subfolders. This can be treated as a boolean or you can specify the max depth of subfolder traversal. [default: false]   
-s, --shutdown   
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Shutdown computer after finishing. [default: false]   
-t, --test   
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Run in test mode allowing you to see what files will be converted. [default: false]   
-w, --wait=`<minutes>`   
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;How long to wait between requests to the server. [default: 5]   
-h, --help   
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Print help information.   
-v, --version   
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Print version information.   
