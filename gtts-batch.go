package main

import (
	"bufio"
	"errors"
	"fmt"
	"os"
	"os/exec"
	"strings"
	"time"
)

// TODO: remove gtts dependency. https://cloud.google.com/text-to-speech/docs/libraries#client-libraries-install-go Looks easy.
// TODO: a file input sanitizer
func main() {
	get_flags()
	get_files()
	print_end()

	if arg_flags.shutdown && !arg_flags.test {
		if err := exec.Command("shutdown", "-c", "The computer will shutdown in 10 minutes,\n run \"shutdown /a\" to abort.",
			"-s", "-f", "-t", "600").Run(); err != nil {
			fmt.Println("Failed to initiate shutdown:", err)
		}
	}
}

var finished_count int = 0
var num_folders = 0
var num_files = 0

var control_quit = false

func get_files() {
	if !arg_flags.nointeract {
		go func() { //The key reading thread
			reader := bufio.NewReader(os.Stdin)
			for {
				keypress, _ := reader.ReadString('\n')
				if !control(keypress) {
					break
				}
			}
		}()
	}
	go func() {
		for {
			time.Sleep(20 * time.Second)
			fmt.Print(".")
		}
	}()

	// If this is running on a single file
	if arg_flags.file {
		num_files = 1
		run_command(arg_flags.path)
		return
	}

	if !arg_flags.nocount { //Count files before processing
		for _, err := walk_init(arg_flags.path, arg_flags.recurse); err == nil; _, err = next() {
		}
		if arg_flags.recurse > 0 {
			fmt.Printf("Starting conversion of %v files in %v folders.\n", num_files, num_folders)
		} else {
			fmt.Printf("Starting conversion of %v files.\n", num_files)
		}
	}

	var processed_count = 1
	for file, err := walk_init(arg_flags.path, arg_flags.recurse); err == nil; file, err = next() {
		fmt.Printf("\n%v. (%d:%d) %s -> ", processed_count, time.Now().Hour(), time.Now().Minute(), file)
		processed_count++
		run_command(file)
		if control_quit {
			break
		}
	}
}

var not_first_time = false

func run_command(path string) {
	_ = path
	if arg_flags.test {
		return
	}
	if not_first_time {
		time.Sleep(time.Duration(arg_flags.wait) * time.Minute)
		if control_quit {
			return
		}
	}
	not_first_time = true

	output, err := exec.Command("gtts-cli", "--lang", "en", "--file", path, "--output",
		path+"_gtts.mp3").Output() //TODO test if errors

	if err == nil {
		fmt.Println("Completed Successfully with message: " + string(output))
		finished_count++
	} else {
		var eerr *exec.ExitError
		if errors.As(err, &eerr) {
			fmt.Println("Completed with error (" + string(eerr.Stderr) + ")")
		} else {
			fmt.Println("Completed with error (" + err.Error() + ")")
		}
	}
}

// return false to stop reading keypresses
func control(keypress string) bool {
	keypress = strings.TrimRight(strings.ToLower(keypress), "\n\r")
	switch keypress {
	case "c":
		fallthrough
	case "x":
		fallthrough
	case "q":
		fmt.Print("\nQuiting after finishing the current file.")
		control_quit = true
	case "i":
		fallthrough
	case "n":
		return false
	case "s":
		arg_flags.shutdown = !arg_flags.shutdown
		if arg_flags.shutdown {
			fmt.Print("\nThe program will shutdown after finishing.")
		} else {
			fmt.Print("\nThe program will NOT shutdown after finishing.")
		}
	case "h":
		fmt.Println()
		print_controls()
	}
	return true
}

var start time.Time = time.Now()

func print_end() {
	end := time.Now()
	total_time := end.Sub(start)
	fmt.Printf("\nExecution finished on (%d/%d) files.", finished_count, num_files)
	fmt.Printf("\nTotal Time: %s", total_time.String())
}
