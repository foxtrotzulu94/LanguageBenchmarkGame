#include "program.h"

/* global arg_xxx structs */
struct arg_lit *ignore, *help, *version, *md5, *sha1, *sha256;
struct arg_file *dir_a, *dir_b;
struct arg_end *end;

// Parsed args
static struct arg_holder args;

void print_time_now(char* fmt_string){
    char* time = format_time_now();
    printf(fmt_string, time);
    free(time);
}

char* format_timespec(time_t time){
    int buffer_size = 80;
    char* buffer = calloc(buffer_size, sizeof(char*));

    struct tm * timeinfo = localtime (&time);

    strftime(buffer, buffer_size, time_format, timeinfo);
    return buffer;
}

char* format_time_now(){
    time_t rawtime;
    time (&rawtime);
    return format_timespec(rawtime);
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

    // if a user specified more than one checksum, error out
    if (md5->count + sha1->count + sha256->count > 1){
        printf("More than one checksum function specified!\n");
        printf("Try '%s --help' for more information.\n", progname);
        return 1;
    }

    args.dir_a = dir_a->filename[0];
    args.dir_b = dir_b->filename[0];
    args.ignoreUnchanged = ignore->count > 0;

    args.hash_implementation = MD5_checksum;
    strncpy(args.hash_name, "MD5", sizeof(args.hash_name));

    if(sha1->count > 0){
        strncpy(args.hash_name, "SHA1", sizeof(args.hash_name));
        args.hash_implementation = SHA1_checksum;
    }
    else if(sha256->count > 0){
        strncpy(args.hash_name, "SHA256", sizeof(args.hash_name));
        args.hash_implementation = SHA256_checksum;
    }

    // Everything was successful, start work
    return 0;
}

int main(int argc, char** argv){
    /* the global arg_xxx structs are initialised within the argtable */
    void *argtable[] = {
        dir_a    = arg_filen(NULL, NULL, "<dir_a>", 1, 1, "Directory to Parse"),
        dir_b    = arg_filen(NULL, NULL, "<dir_b>", 1, 1, "Directory to Parse"),
        ignore   = arg_litn("u", "ignore-unchanged", 0, 1, "Ignore unchanged files in the final output"),
        md5      = arg_litn(NULL, "md5", 0, 1, "Use MD5 Hash"),
        sha1     = arg_litn(NULL, "sha1", 0, 1, "Use SHA1 Hash"),
        sha256   = arg_litn(NULL, "sha256", 0, 1, "Use SHA-256 Hash"),
        help    = arg_litn(NULL, "help", 0, 1, "Display this help and exit"),
        version = arg_litn(NULL, "version", 0, 1, "Version info"),
        end     = arg_end(2),
    };

    int exitcode = parse_args(argc, argv, argtable);

    // If we had no issue parsing the arguments, let's do the work now
    if(exitcode == 0){
        printf("Starting diff of \"%s\" and \"%s\" (%s)\n", args.dir_a, args.dir_b, args.hash_name);
        print_time_now("Start time %s\n");
        // TODO: do work
        // TODO: Do the work!
        // khash_t(testy) *h;
        // h = kh_init(testy);
        // int ret;
        // khint_t iter = kh_put(testy, h, "hi", &ret);
        // kh_val(h, iter) = 10;
        // kh_destroy(testy, h);
        // TODO:

        print_time_now("End time %s\n");
    }

    arg_freetable(argtable, sizeof(argtable) / sizeof(argtable[0]));
    return exitcode;
}
