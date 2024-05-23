@echo off

:: --release - use release profile build
:: --no-typescript - disable .d.ts files output
:: --out-dir - set output directory
:: --out-name - force output file names
:: --target - always use "web"!
:: See https://rustwasm.github.io/wasm-pack/book/commands/build.html
echo Removing old files...wasm
rd /s /q ".\manifest-v3\js\wasm"
echo Removing old files...dist
rd /s /q ".\manifest-v3\js\dist"
echo Building wasm module...
wasm-pack build --release --no-typescript --out-dir "./manifest-v3/js/wasm" --out-name "better_spider" --target bundler
@REM wasm-pack build --release --no-typescript --out-dir "./manifest-v3/js/wasm" --out-name "better_spider" --target web
:: wasm-pack creates bunch of useless files:
:: - Output of typescript files disabled by --no-typescript wasm-pack argument
:: - We should delete the .gitignore and package.json files ourselves
echo Removing trash files...
if exist ".\manifest-v3\js\wasm\.gitignore" del ".\manifest-v3\js\wasm\.gitignore"
if exist ".\manifest-v3\js\wasm\package.json" del ".\manifest-v3\js\wasm\package.json"

echo Done
pause