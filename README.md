# RustMon

### Only windows supported currently. Cross platform support underway.

RustMon is a utility program written in Rust that allows you to watch a directory and execute a command when the timestamp in any file changes. It uses a json file for it's configuration.
### How to use the JSON settings file?
File name - `rm_conf.json`
 - `process` - What process you want to run?
 - `process_dir` - Useful for processes where you want to run the process somewhere else. ( Not much of use, will find a better way, I promise! )
 - `watch_directory` - What directory do you want to watch? Remove this field to default to the directory the process in run in.
 
 Keep the `rm_conf.json` file in the folder you run the process in. This ensures that it can find the settings required for it to run, for example, the process ( currently it doesn't assume any default process! )

**More help coming soon people!**
