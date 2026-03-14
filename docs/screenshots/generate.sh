cargo run -p cmakefmt-cli -- input.cmake > output.cmake
sed -i 's/# Before/# After/g' output.cmake
freeze -c freeze.json output.cmake -o output.svg
freeze -c freeze.json input.cmake -o input.svg
