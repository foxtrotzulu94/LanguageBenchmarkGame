class Program {
    public class Arguments
    {
        public String DirectoryA;
        public String DirectoryB;
        public Boolean IgnoreUnchanged;
        public String HashName;

        public Arguments(){}
    }

    public static void main(String[] args){
        ArgumentHolder arg = new ArgumentHolder();
        if(!arg.parse(args)){
            System.out.println("Error parsing arguments!");
            System.exit(1);
        }

        System.out.println(String.format("Starting diff of %s and %s (%s)", arg.DirectoryA, arg.DirectoryB, arg.ChecksumName));
    }
}