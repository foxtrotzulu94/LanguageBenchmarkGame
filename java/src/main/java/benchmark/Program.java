package benchmark;

import java.nio.file.Path;
import java.text.DateFormat;
import java.text.SimpleDateFormat;
import java.util.Date;
import java.util.concurrent.Callable;
import picocli.CommandLine;
import picocli.CommandLine.ArgGroup;
import picocli.CommandLine.Command;
import picocli.CommandLine.Option;
import picocli.CommandLine.Parameters;

@Command(
    name = "Java Implementation of the Benchmark Game", 
    mixinStandardHelpOptions = true, 
    version = "1.0",
    description = "Prints the checksum (MD5 by default) of a file to STDOUT.")
class Program implements Callable<Integer> {
    // Universal date time format
    public static DateFormat dateFormat = new SimpleDateFormat("yyyy-mm-dd hh:mm:ss");
    
    // Program args
    public ArgumentHolder args;
    
    @Parameters(index = "0", description = "The file whose checksum to calculate.")
    private Path PathA;
    
    @Parameters(index = "1", description = "The file whose checksum to calculate.")
    private Path PathB;
    
    @Option(names = {"-u", "--ignore-unchanged"}, description = "Ignore unchagned files in the final output")
    private Boolean ignoreUnchanged = false;
    
    @ArgGroup(exclusive = true, multiplicity = "1")
    Exclusive checksumAlgorithm;

    static class Exclusive {
        @Option(names = {"--MD5", "--md5"}, required = false) Boolean MD5;
        @Option(names = {"--SHA1","--sha1"}, required = false) Boolean SHA1;
        @Option(names = {"--SHA256", "--sha256"}, required = false) Boolean SHA256;
    }

    public static void main(String[] cmdArgs){
        int exitCode = new CommandLine(new Program()).execute(cmdArgs);
        System.exit(exitCode);
    }
    
    @Override
    public Integer call() throws Exception {
        String algorithm = "MD5";
        // check which algorithm was chosen
        if (this.checksumAlgorithm.SHA1 != null) {
            algorithm = "SHA-1";
        }
        else if (this.checksumAlgorithm.SHA256 != null) {
            algorithm = "SHA-256";
        }
        
        this.args = new ArgumentHolder(this.PathA, this.PathB, algorithm, this.ignoreUnchanged);
        System.out.println(
                String.format("Starting diff of %s and %s (%s)", 
                        this.args.DirectoryA, 
                        this.args.DirectoryB, 
                        this.args.Checksum.getAlgorithm()));
        System.out.println(
                String.format("Starting at %s", dateFormat.format(new Date())));
        
        var worker = new Worker(this.args);
        try{
            worker.Run();
        }
        catch(Exception e){
            System.out.println(e.getMessage());
            //e.printStackTrace();
            return 1;
        }
        
        System.out.println(
                String.format("Finished at %s", dateFormat.format(new Date())));
        
        return 0;
    }
}