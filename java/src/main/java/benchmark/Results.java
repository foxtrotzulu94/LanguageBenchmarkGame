package benchmark;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.concurrent.ConcurrentHashMap;

import org.apache.commons.lang3.tuple.ImmutablePair;
import org.apache.commons.lang3.tuple.Pair;

/**
 *
 * Namespace class
 */
public class Results {
    public enum Operation
    {
        ADD('+'),
        UNCHANGED('='),
        CONFLICT('!');

        private final char symbol;
        Operation(final char symbol){
            this.symbol = symbol;
        }

        @Override
        public String toString(){
            return Character.toString(symbol);
        }
    }
    
    public class ScanResult extends ConcurrentHashMap<String, FileResult> {}
    public class PatchResult extends HashMap<Operation, ArrayList<FileResult>> {}
}
