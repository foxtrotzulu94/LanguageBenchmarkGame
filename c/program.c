#include "program.h"

/* global arg_xxx structs */
struct arg_lit *ignore, *help, *version, *md5, *sha, *sha256;
struct arg_file *dir_a, *dir_b;
struct arg_end *end;

void print_time(char* fmt_string){
    char* time = format_time();
    printf(fmt_string, time);
    free(time);
}

char* format_time(){
    time_t rawtime;
    struct tm * timeinfo;
    int buffer_size = 80;
    char* buffer = calloc(buffer_size, sizeof(char*));

    time (&rawtime);
    timeinfo = localtime (&rawtime);

    strftime(buffer, buffer_size, time_format, timeinfo);
    return buffer;
}

int parse_args(int argc, char** argv, void *argtable[]){
    char progname[] = "program";

    int nerrors;
    nerrors = arg_parse(argc,argv,argtable);

    /* special case: '--help' takes precedence over error reporting */
    if (help->count > 0)
    {
        printf("Usage: %s", progname);
        arg_print_syntax(stdout, argtable, "\n");
        printf("Demonstrate command-line parsing in argtable3.\n\n");
        arg_print_glossary(stdout, argtable, "  %-25s %s\n");
        return 0;
    }

    /* If the parser returned any errors then display them and exit */
    if (nerrors > 0)
    {
        /* Display the error details contained in the arg_end struct.*/
        arg_print_errors(stdout, end, progname);
        printf("Try '%s --help' for more information.\n", progname);
        return 1;
    }

    // Actually do the work now
    printf("Starting diff of \"%s\" and \"%s\"\n", dir_a->filename[0], dir_b->filename[0]);
    print_time("Start time %s\n");

    struct arg_holder args;
    args.dir_a = dir_a->filename[0];
    args.dir_b = dir_b->filename[0];
    args.ignoreUnchanged = ignore->count > 0;
    strncpy(args.checksum, "MD5", sizeof(args.checksum));

    // TODO: Do the work!

    print_time("End time %s\n");

    return 0;
}

int main(int argc, char** argv){
    /* the global arg_xxx structs are initialised within the argtable */
    void *argtable[] = {
        dir_a    = arg_filen(NULL, NULL, "<dir_a>", 1, 1, "Directory to Parse"),
        dir_b    = arg_filen(NULL, NULL, "<dir_b>", 1, 1, "Directory to Parse"),
        ignore   = arg_litn("u", "ignore-unchanged", 0, 1, "Ignore unchanged files in the final output"),
        help    = arg_litn(NULL, "help", 0, 1, "Display this help and exit"),
        version = arg_litn(NULL, "version", 0, 1, "Version info"),
        end     = arg_end(2),
    };

    int exitcode = parse_args(argc, argv, argtable);

    arg_freetable(argtable, sizeof(argtable) / sizeof(argtable[0]));
    return exitcode;
}
