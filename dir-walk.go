package main

import (
	"errors"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"
)

var RECURSE uint
var PATH string

type FileList struct {
	names []string
	depth []uint
}

var files = FileList{
	names: make([]string, 0, 10000),
	depth: nil,
}
var folders = FileList{
	names: make([]string, 0, 1000),
	depth: make([]uint, 0, 1000),
}

var depth uint

// Sets up the path and if recursive, then returns the first entry
func walk(path string) (string, error) {
	dirs, err := os.ReadDir(path)

	if err != nil {
		fmt.Println("Error reading (" + path + "): " + err.Error())
		return "", err
	}

	for _, file := range dirs {
		if file != nil {
			push(file, path)
		}
	}

	return next()
}

func next() (string, error) {
	if len(files.names) > 0 {
		return pop(), nil
	}
	if len(folders.names) > 0 {
		var folder string
		folder, depth = pop_folder()

		return walk(folder)
	}

	return "", errors.New("walk: no more files to walk")
}

func walk_init(path string, recurse uint) (string, error) {
	RECURSE = recurse
	PATH = home_dir_abs(path)
	depth = 0
	num_folders = 0
	num_files = 0

	return walk(PATH)
}

// Replace ~ with the users home directory (only if ~ is first character of path)
// Fairly certain GO's cl arg handling means trimming is unnecessary
func home_dir_abs(path string) string {
	if len(path) > 0 && path[0] == '~' {
		temp_dir, err := os.UserHomeDir()
		if err == nil {
			path = temp_dir + path[1:]
			temp_dir, err = filepath.Abs(path)
			if err == nil {
				return temp_dir
			}
		}
	}
	return path
}

func push(file fs.DirEntry, path string) {
	if file.IsDir() {
		if RECURSE > depth {
			folders.names = append(folders.names, filepath.Join(path, file.Name()))
			folders.depth = append(folders.depth, depth+1)
			num_folders++
		}
	} else {
		if strings.EqualFold(filepath.Ext(file.Name()), ".txt") {
			if !arg_flags.overwrite {
				stat, err := os.Stat(filepath.Join(path, file.Name()) + "_gtts.mp3")
				if err == nil && stat != nil && !stat.IsDir() {
					return
				}
			}
			files.names = append(files.names, filepath.Join(path, file.Name()))
			num_files++
		}
	}
}
func pop() string {
	//Remove the first element after returning it.
	defer func() { files.names = files.names[1:] }()
	return files.names[0]
}
func pop_folder() (string, uint) {
	//Remove the first element after returning it.
	defer func() {
		folders.names = folders.names[1:]
		folders.depth = folders.depth[1:]
	}()
	return folders.names[0], folders.depth[0]
}
