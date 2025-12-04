# Warnings
This `git repository` is a `git submodule` for reusing `component` and `backend` running on `dioxus`.

# How to use
- Create the main `dioxus` project and change this directory.
```
dx new Aaa
cd Aaa
```
- Add `git submodule` to the main `dioxus` project.
```
mkdir -p src/modules
git submodule add https://github.com/aki-akaguma/browserinfo.git src/modules/browserinfo
```
- Include the modules to be used in `src/main.rs`.
```
vi src/main.rs
```
add next source code:
```
mod modules;
```
- Finally, add it to `Cargo.toml`. See `AddCargo.toml` for items to add.

# `browserinfo`
The `browserinfo` module collects and records client-side information. Please use it for debugging etc.

## Operation
- Collect information on the client side. The information obtained is the `user agent` and `screen size`` obtained by `java script`.
- Send the obtained information to the backend.
- On the backend, the obtained information is stored in SQLite database.

