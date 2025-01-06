# gtts-batch   
   
Wrapper over gtts to handle batch converting many files. gtts-cli need to be installed for gtts-batch to function.   
To use just copy executable into a folder with the .txt files you want to convert to audio and run.    

You need to make sure the files are not too big, I believe the system is used as a screen reader but I have done up to 40kb without any problems as long as the wait time is big enough. You can reduce the wait time if the files are small enough but Google stops responding if you push it too far. You want to remove any weird characters you sometimes get on webpages, I think it is anything other than the standard ascii, if their is a character that gtts-cli does not like it returns an audio file up to that character.

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
