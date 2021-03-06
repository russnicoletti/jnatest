import com.sun.jna.Library;
import com.sun.jna.Native;
import com.sun.jna.Platform;

import java.io.File;
import java.io.FileOutputStream;
import java.io.PrintStream;

public class RustLibraryBridgeTest {

    // This is the standard, stable way of mapping, which supports extensive
    // customization and mapping of Java to native types.

    public interface RustLibrary extends Library {

        RustLibrary INSTANCE = (RustLibrary)
            Native.loadLibrary("myrustlib",
                               RustLibrary.class);

        void jRustPrint(String thing);
    }

    public static void main(String[] args) {
      RustLibrary.INSTANCE.jRustPrint(args[0]);
    }
}

