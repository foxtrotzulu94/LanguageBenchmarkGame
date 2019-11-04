package benchmark;

import static benchmark.Program.dateFormat;
import java.io.BufferedWriter;
import java.io.File;
import java.io.FileInputStream;
import java.io.FileWriter;
import java.io.IOException;
import java.io.Writer;
import java.lang.StringBuilder;
import java.nio.file.Files;
import java.nio.file.Path;
import java.security.MessageDigest;
import java.util.ArrayList;
import java.util.Collections;
import java.util.Date;
import java.util.HashSet;
import java.util.Set;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.Future;
import java.util.logging.Level;
import java.util.logging.Logger;
import java.util.stream.Collectors;

import org.apache.commons.codec.binary.Hex;
import org.apache.commons.lang3.tuple.ImmutablePair;

/**
 * Does the bulk of the work in the game
 */
public class Worker {
    private final String resultFilename = "reference.patch";
    
    private ExecutorService mainExecutor;
    private final int requestedThreads;
    private final ArgumentHolder workingArgs;
    
    private Results.ReconcileResult patches;
    
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
        var pathsA = new HashSet<String>(Collections.list(a.keys()));
        var pathsB = new HashSet<String>(Collections.list(b.keys()));
        
        var suspectedConflicts = new HashSet<String>(Collections.list(a.keys()));
        // A Intersect B
        suspectedConflicts.retainAll(pathsB);
        
        var unchangedFiles = suspectedConflicts.stream()
                .filter(entry -> a.get(entry).equals(b.get(entry)))
                .collect(Collectors.toSet());
        
        var conflicts = suspectedConflicts.stream()
                .filter(entry -> !unchangedFiles.contains(entry))
                .collect(Collectors.toSet());
        
        // Store the result of the reconciliation
        this.patches = new Results.ReconcileResult(
                GeneratePatch(b,a, pathsB, pathsA, unchangedFiles, conflicts), 
                GeneratePatch(a,b, pathsA, pathsB, unchangedFiles, conflicts));
    }
    
    private Results.PatchResult GeneratePatch(
            Results.ScanResult src, Results.ScanResult target,
            Set<String> srcPaths, Set<String> targetPaths,
            Set<String> unchanged, Set<String> conflicts)
    {
        var patch = new Results.PatchResult();
        
        // Additions
        patch.put(
                Results.Operation.ADD, 
                srcPaths.stream()
                        // Any path in src not present in target
                        .filter(path -> !targetPaths.contains(path))
                        .map(path -> src.get(path))
                        .collect(ArrayList::new, ArrayList::add,ArrayList::addAll)
        );
        
        // Unchanged
        patch.put(
                Results.Operation.UNCHANGED, 
                unchanged.stream()
                        // Just map
                        .map(path -> src.get(path))
                        .collect(ArrayList::new, ArrayList::add,ArrayList::addAll)
        );
        
        // Conflicts
        patch.put(
                Results.Operation.CONFLICT, 
                conflicts.stream()
                        // Just map
                        .map(path -> src.get(path))
                        .collect(ArrayList::new, ArrayList::add,ArrayList::addAll)
        );
        
        return patch;
    }
    
    private void WriteResults() throws IOException{
        var resultFile = new FileWriter(this.resultFilename);
        var writer = new BufferedWriter(resultFile);
        
        writer.write(String.format("# Results for %s\n", dateFormat.format(new Date())));
        writer.write(String.format("# Reconciled '%s' '%s'\n", workingArgs.DirectoryA.toString(), workingArgs.DirectoryB.toString()));
        WritePatchResult(writer, workingArgs.DirectoryA.toString(), this.patches.a, workingArgs.IgnoreUnchanged);
        writer.write("\n");
        WritePatchResult(writer, workingArgs.DirectoryB.toString(), this.patches.b, workingArgs.IgnoreUnchanged);
        writer.write("\n");
        writer.close();
        
        resultFile.close();
    }
    
    private void WritePatchResult(Writer out, String directory, Results.PatchResult patch, boolean ignoreUnchanged) throws IOException{
        out.write(directory);
        out.write("\n");
        
        var lines = new ArrayList<ImmutablePair<Results.Operation, FileResult>>();
        
        // flatten the results
        for(var action : patch.entrySet()){
            var operation = action.getKey();
            if(operation == Results.Operation.UNCHANGED && ignoreUnchanged){
                continue;
            }
            
            action.getValue().forEach((entry) -> {
                lines.add(new ImmutablePair(operation, entry));
            });
        }
        
        // Sort by filename
        lines.sort((a,b) -> {
            return a.right.filePath.compareTo(b.right.filePath);
        });
        
        for(var line : lines){
            out.write(String.format("%s %s\n", line.left.toString(), line.right.toString()));
        }
    }
}
