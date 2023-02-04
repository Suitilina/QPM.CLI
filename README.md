# QPM.CLI
QPM command line tool

# Improvements over [QPM-Rust v1](https://github.com/RedBrumbler/QuestPackageManager-Rust)
- `qpm-rust version update` Updates qpm-rust to a newer version. 
- `qpm-rust s/scripts build` Script system similar to NPM for configuring workspaces.
- `qpm-rust ndk download {version}/list/available` Downloading and managing NDK installations
- `qpm-rust download cmake/ninja` for setting up CMake and Ninja.
- `qpm-rust doctor` for checking if everything is setup properly.
- `qpm-rust templatr` embedded.
- Reports progress when downloading or cloning
- Leverages local cache for faster restores (and even offline usage, TODO)
- Rewritten from the ground up to use functional patterrns, immutability and declarative code style. Results in better reliability and consistency.
- ~~Supports locked restore `qpm-rust restore --locked`~~ This is the default now
- More modular
- Easier to maintain
- Could support mirrors or other backends
- Better error handlingg
- Is tested (not thoroughly yet)
