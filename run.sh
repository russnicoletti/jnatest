cd rustlib
cargo build
cp target/debug/libmyrustlib.dylib ..
cd -

# The rust library uses the following environment variable to determine the log level
export MY_RUST_LIB_LOG=info

javac -cp /Users/russ/Tools/jna-platform/jna-4.2.1.jar RustLibraryBridgeTest.java 
java -cp /Users/russ/sandbox/java/jnatest:/Users/russ/Tools/jna-platform/jna-4.2.1.jar RustLibraryBridgeTest hello! 
