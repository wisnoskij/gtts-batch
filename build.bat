set GOOS=linux
set GOARCH=amd64
go build -o gtts-batch-linux-x64
set GOARCH=386
go build -o gtts-batch-linux-x32
set GOARCH=arm64
go build -o gtts-batch-linux-ARM64

set GOOS=android
set GOARCH=amd64
go build -o gtts-batch-android-x64
set GOARCH=arm64
go build -o gtts-batch-android-ARM64

set GOOS=ios
set GOARCH=arm64
go build -o gtts-batch-ios-arm64

set GOOS=darwin
set GOARCH=amd64
go build -o gtts-batch-darwin-x64
set GOARCH=386
go build -o gtts-batch-darwin-x32
set GOARCH=arm64
go build -o gtts-batch-darwin-ARM64

set GOOS=windows
set GOARCH=386
go build -o gtts-batch-windows-x32.exe
set GOARCH=arm64
go build -o gtts-batch-windows-ARM64.exe
set GOARCH=amd64
go build -o gtts-batch-windows-x64.exe