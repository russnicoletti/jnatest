# jnatest

Example of calling a rust library from java via jna. Additionally, `jnatest` provides an example
of capturing the log messages from a rust library (the rust library uses the `log` and `env_logger`
crates).

##Usage

Download the [jna package](http://mvnrepository.com/artifact/net.java.dev.jna/jna/4.2.1)

Edit 'run.sh', changing the classpath directories appropriately to point to the location of the the java test program and the jna jar file.

Run:

    ./run.sh 2> logging.log


