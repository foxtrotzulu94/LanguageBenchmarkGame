package benchmark;

import java.io.File;
import java.io.FileInputStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.security.MessageDigest;
import java.util.Date;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.Future;
import java.util.logging.Level;
import java.util.logging.Logger;

import org.apache.commons.codec.binary.Hex;

/**
 * Does the bulk of the work in the game
 */
public class Worker {
    private ExecutorService mainExecutor;
    private final int requestedThreads;
    private final ArgumentHolder workingArgs;
    
    // private 
    
    public Worker(ArgumentHolder args) {
        this.workingArgs = args;
        this.requestedThreads = (Runtime.getRuntime().availableProcessors()*2);
    }
    
    public void Run() throws Exception{
        this.mainExecutor = Executors.newFixedThreadPool(requestedThreads);
        var resultsA = this.ScanDirectory(workingArgs.DirectoryA);
        var resultsB = this.ScanDirectory(workingArgs.DirectoryB);
        
        Reconcile(resultsA.get(), resultsB.get());
        WriteResults();
        
        // no tasks should be running anymore
        this.mainExecutor.shutdownNow();
    }
    
    private Future<Results.ScanResult> ScanDirectory(Path root) throws Exception {
        // Java doesn't have nice async/await, so we have to rely on future for now
        return this.mainExecutor.submit(() -> {
            final int rootPathLength = root.toString().length();
            var results = new Results.ScanResult();
            Files.walk(root)
                    .filter(Files::isRegularFile)
                    .forEach(filePath -> {
                        try {                            
                            String canonicalPath = filePath
                                    .toString()
                                    .substring(
                                            rootPathLength+1, 
                                            filePath.toString().length());
                            
                            results.put(canonicalPath, ProcessFile(canonicalPath, filePath.toFile()));
                        } catch (Exception ex) {
                            Logger.getLogger(Worker.class.getName()).log(Level.SEVERE, null, ex);
                        }
            });
        
            return results;
        });
    }
    
    private FileResult ProcessFile(String pathKey, File file) throws Exception{
        // Old styled buffer read here
        int BUFFER_SIZE = 64 * 1024;
        byte[] buffer = new byte[BUFFER_SIZE];
        
        // Clone the object so we don't run into concurrent use issues
        MessageDigest digest = (MessageDigest) workingArgs.Checksum.clone();
        var stream = new FileInputStream(file);
        
        while(true){
            var bytesRead = stream.read(buffer);
            if(bytesRead == -1)
                break;
            
            digest.update(buffer, 0, bytesRead);
        }
        
        return new FileResult(
                pathKey,
                Hex.encodeHexString(digest.digest()),
                file.length(),
                new Date(file.lastModified())
        );
    }
    
    private void Reconcile(Results.ScanResult a, Results.ScanResult b){
        // TODO
        System.out.println("Reconcile");
    }
    
    private void WriteResults(){
        
    }
}
