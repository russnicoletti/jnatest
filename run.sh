cd rustlib
cargo build
cp target/debug/librustlib.dylib ..
cd -
javac -cp /Users/russ/Tools/jna-platform/jna-4.2.1.jar RustLibraryBridgeTest.java 
java -cp /Users/russ/sandbox/java/jnatest:/Users/russ/Tools/jna-platform/jna-4.2.1.jar RustLibraryBridgeTest hello! 
