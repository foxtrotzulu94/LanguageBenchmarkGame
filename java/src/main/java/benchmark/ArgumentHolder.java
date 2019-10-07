package benchmark;

import java.nio.file.Path;
import java.nio.file.FileSystem;
import java.nio.file.FileSystems;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;


class ArgumentHolder{
    public final Path DirectoryA;
    public final Path DirectoryB;
    public final Boolean IgnoreUnchanged;
    public final MessageDigest Checksum;

    public ArgumentHolder(String dirA, String dirB, String checksum, Boolean ignoreUnchanged)
        throws NoSuchAlgorithmException{
        // Parse the directories
        FileSystem fs = FileSystems.getDefault();
        this.DirectoryA = fs.getPath(dirA);
        this.DirectoryB = fs.getPath(dirB);
        IgnoreUnchanged = false;
        Checksum = MessageDigest.getInstance(checksum);
    }
    
    // No delegating constructors in Java means duplicate code :(
    public ArgumentHolder(Path dirA, Path dirB, String checksum, Boolean ignoreUnchanged)
        throws NoSuchAlgorithmException{
        this.DirectoryA = dirA;
        this.DirectoryB = dirB;
        IgnoreUnchanged = false;
        Checksum = MessageDigest.getInstance(checksum);
    }
}