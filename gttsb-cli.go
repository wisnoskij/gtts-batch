package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Flags struct {
	code       string
	nocount    bool
	file       bool
	nointeract bool
	overwrite  bool
	path       string
	recurse    uint
	shutdown   bool
	test       bool
	version    bool
	wait       uint
}

var arg_flags = Flags{
	code:       "",
	nocount:    false,
	file:       false,
	nointeract: false,
	overwrite:  false,
	path:       ".",
	recurse:    0,
	shutdown:   false,
	test:       false,
	version:    false,
	wait:       5,
}

var CL_FLAGS = map[string]func(string, string) bool{
	"--code":       ff_code,
	"-c":           ff_code,
	"--help":       ff_help,
	"-h":           ff_help,
	"--nointeract": ff_nointeract,
	"-i":           ff_nointeract,
	"--nocount":    ff_nocount,
	"-n":           ff_nocount,
	"--overwrite":  ff_overwrite,
	"-o":           ff_overwrite,
	"--path":       ff_path,
	"-p":           ff_path,
	"--recurse":    ff_recurse,
	"-r":           ff_recurse,
	"--shutdown":   ff_shutdown,
	"-s":           ff_shutdown,
	"--test":       ff_test,
	"-t":           ff_test,
	"--version":    ff_version,
	"-v":           ff_version,
	"--wait":       ff_wait,
	"-w":           ff_wait,
}

// The flag specific parsers and checkers
// val = the supplied value after the "=" in the flag arg if any
// return true if next value in args should be skipped
var ff_code = func(val string, look_ahead_val string) (look_ahead bool) {
	if val == "" {
		if look_ahead_val == "" {
			fmt.Println("No code supplied.")
			os.Exit(1)
		}
		val = look_ahead_val
		look_ahead = true
	}
	arg_flags.code = val
	return
}

var ff_nointeract = func(val string, look_ahead_val string) bool {
	var nointeract bool
	var is_bool bool
	if val != "" {
		is_bool, nointeract = truthy(val)
		if !is_bool {
			fmt.Println("The supplied value for nointeract is not a bool: " + val)
			os.Exit(1)
		}
		arg_flags.nointeract = nointeract
	} else if look_ahead_val != "" {
		is_bool, nointeract = truthy(look_ahead_val)
		if is_bool {
			arg_flags.nointeract = nointeract
			return true
		}
	}
	//If no value supplied, assume true.
	arg_flags.nointeract = true
	return false
}

var ff_path = func(val string, look_ahead_val string) (look_ahead bool) {
	if val == "" {
		if look_ahead_val == "" {
			fmt.Println("No path supplied.")
			os.Exit(1)
		}
		val = look_ahead_val
		look_ahead = true
	}

	stat, err := os.Stat(val)
	if err != nil {
		fmt.Println("Their was a problem with the given path: " + val)
		fmt.Println("Error: " + err.Error())
		os.Exit(1)
	}
	if stat == nil {
		fmt.Println("The given path does not exist: " + val)
		os.Exit(1)
	}
	if !stat.IsDir() {
		arg_flags.file = true
	}
	arg_flags.path = val

	return
}

var ff_nocount = func(val string, look_ahead_val string) bool {
	var nocount bool
	var is_bool bool
	if val != "" {
		is_bool, nocount = truthy(val)
		if !is_bool {
			fmt.Println("The supplied value for nocount is not a bool: " + val)
			os.Exit(1)
		}
		arg_flags.nocount = nocount
	} else if look_ahead_val != "" {
		is_bool, nocount = truthy(look_ahead_val)
		if is_bool {
			arg_flags.nocount = nocount
			return true
		}
	}
	//If no value supplied, assume true.
	arg_flags.nocount = true
	return false
}
var ff_overwrite = func(val string, look_ahead_val string) bool {
	var overwrite bool
	var is_bool bool
	if val != "" {
		is_bool, overwrite = truthy(val)
		if !is_bool {
			fmt.Println("The supplied value for overwrite is not a bool: " + val)
			os.Exit(1)
		}
		arg_flags.overwrite = overwrite
	} else if look_ahead_val != "" {
		is_bool, overwrite = truthy(look_ahead_val)
		if is_bool {
			arg_flags.overwrite = overwrite
			return true
		}
	}
	//If no value supplied, assume true.
	arg_flags.overwrite = true
	return false
}
var ff_shutdown = func(val string, look_ahead_val string) bool {
	var shutdown bool
	var is_bool bool
	if val != "" {
		is_bool, shutdown = truthy(val)
		if !is_bool {
			fmt.Println("The supplied value for shutdown is not a bool: " + val)
			os.Exit(1)
		}
		arg_flags.shutdown = shutdown
	} else if look_ahead_val != "" {
		is_bool, shutdown = truthy(look_ahead_val)
		if is_bool {
			arg_flags.shutdown = shutdown
			return true
		}
	}
	//If no value supplied, assume true.
	arg_flags.shutdown = true
	return false
}
var ff_recurse = func(val string, look_ahead_val string) bool {
	if val != "" {
		if !set_recurse(val) {
			fmt.Println("The supplied value for recurse is not a bool or int: " + val)
			os.Exit(1)
		}
		return false
	} else if look_ahead_val != "" {
		if set_recurse(look_ahead_val) {
			return true
		}
	}
	//If no value supplied, assume true.
	arg_flags.recurse = 4294967295
	return false
}
var ff_test = func(val string, look_ahead_val string) bool {
	var test bool
	var is_bool bool
	if val != "" {
		is_bool, test = truthy(val)
		if !is_bool {
			fmt.Println("The supplied value for test is not a bool: " + val)
			os.Exit(1)
		}
		arg_flags.test = test
	} else if look_ahead_val != "" {
		is_bool, test = truthy(look_ahead_val)
		if is_bool {
			arg_flags.test = test
			return true
		}
	}
	//If no value supplied, assume true.
	arg_flags.test = true
	return false
}
var ff_wait = func(val string, look_ahead_val string) (look_ahead bool) {
	if val == "" {
		if look_ahead_val == "" {
			fmt.Println("Wait flag supplied with no value.")
			os.Exit(1)
		}
		val = look_ahead_val
		look_ahead = true
	}

	val_int, err := strconv.ParseUint(val, 10, 0)
	if err != nil {
		fmt.Println("Their was a problem with the given wait value: " + val)
		fmt.Println("Error: " + err.Error())
		os.Exit(1)
	}

	arg_flags.wait = uint(val_int)
	return
}

var ff_help = func(val string, look_ahead_val string) bool {
	fmt.Println("usage: gtts-batch [OPTIONS] [FILE/FOLDER]")
	print_controls()
	fmt.Println("OPTIONS:")
	fmt.Println("-c, --code\n	Supply your own gtts code. \"--file <FILE> --output <FILE>_gtts.mp3\" will be appended")
	fmt.Println("-n, --nocount\n	Don't precount the files that will be converted to audio. [default: false]")
	fmt.Println("-i, --nointeract\n	Turn off all interactive options. [default: false]")
	fmt.Println("-o, --overwrite\n	Overwrite existing mp3 files. [default: false]")
	fmt.Println("-p, --path=<file/folder>\n	The path to the file or folder to run on. [default: .]")
	fmt.Println("-r, --recurse\n	Also include subfolders. This can be treated as a boolean or you can specify the max depth of subfolder traversal. [default: false]")
	fmt.Println("-s, --shutdown\n	Shutdown computer after finishing. [default: false]")
	fmt.Println("-t, --test\n	Run in test mode allowing you to see what files will be converted. [default: false]")
	fmt.Println("-w, --wait=<minutes>\n	How long to wait between requests to the server. [default: 5]")
	fmt.Println("-h, --help\n	Print help information.")
	fmt.Println("-v, --version\n	Print version information.")
	ff_version("", "")
	return false
}
var ff_version = func(val string, look_ahead_val string) bool {
	fmt.Println("GTTS Batch Version: 1.0")
	os.Exit(1)
	return false
}

func print_controls() {
	fmt.Println("CONTROLS:")
	fmt.Println("(Press the key and then enter to confirm)")
	fmt.Println("c, x, q\n	Exit the program after the current file is done.")
	fmt.Println("i, n\n	Turns off interactive mode, preventing typing from affecting the running of this process.")
	fmt.Println("s\n	Toggle if the program will shutdown the computer after finishing.")
	fmt.Println("h\n	Print this help information")
}

// Tests if a string is boolean-y, if it is, also return if true or false
// Returns (is_bool, truthy)
func truthy(val string) (bool, bool) {
	if strings.EqualFold(val, "false") || strings.EqualFold(val, "f") || strings.EqualFold(val, "0") {
		return true, false
	} else if strings.EqualFold(val, "true") || strings.EqualFold(val, "t") || strings.EqualFold(val, "1") {
		return true, true
	} else {
		return false, false
	}
}

// Checks the return value for correctness, and returns true if correct.
// Also sets arg_flags.recurse if found to be correct.
// recurse accepts bools or ints representing the depth of recursion.
func set_recurse(val string) bool {
	val_int, err := strconv.ParseUint(val, 10, 0)
	if err == nil {
		arg_flags.recurse = uint(val_int)
		return true
	}

	is_bool, recurse := truthy(val)
	if is_bool {
		if recurse {
			arg_flags.recurse = 4294967295 //big enough
		} else {
			arg_flags.recurse = 0
		}
		return true
	}
	return false
}

// Only requirement is a path to a file/folder to convert
func check_requirements() {
	if arg_flags.path == "" {
		fmt.Println("No Path supplied. You must supply a path to a file of folder to run on.")
		os.Exit(1)
	}
}

// Gets the command line flags
// Will exit with error if flags inappropriately set
func get_flags() {
	var look_ahead bool
	var look_ahead_val string
	var arg_pair []string

	for i, v := range os.Args[1:] {
		if look_ahead {
			look_ahead = false
			continue
		}

		//if arg is not flag, maybe it is a path
		if v[0] != '-' {
			ff_path(v, "")
			continue
		}

		//separate flat into "name=val" pair
		arg_pair = strings.SplitN(v, "=", 2)
		arg_pair[0] = strings.ToLower(arg_pair[0])
		if len(arg_pair) == 1 {
			arg_pair = append(arg_pair, "")
		}

		//fmt.Printf("test: |%v| |%v| |%v|", arg_pair[1], len(os.Args), (i + 2))
		if CL_FLAGS[arg_pair[0]] != nil {
			//look ahead is empty if i already have a value or
			if arg_pair[1] != "" || (i+2) >= len(os.Args) || os.Args[i+2][0] == '-' {
				look_ahead_val = ""
			} else {
				look_ahead_val = os.Args[i+2]
			}
			look_ahead = CL_FLAGS[arg_pair[0]](arg_pair[1], look_ahead_val)
			continue
		}

		fmt.Println("Given flag does not exist: " + arg_pair[0])
		ff_help("", "")
	}

	check_requirements()
}
