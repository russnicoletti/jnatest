# jnatest

Example of calling a rust library from java via jna.

##Usage

Download the [jna package](http://mvnrepository.com/artifact/net.java.dev.jna/jna/4.2.1)

Edit 'run.sh', changing the classpath directories appropriately to point to the location of the the java test program and the jna jar file.

Compile the rust library:
    rustc rustlib/mylib.rs

Run:
    ./run.sh


