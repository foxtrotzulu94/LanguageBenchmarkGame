#!/usr/bin/env python3

# This is a basic runner "harness" to manage the different languages and their trials
# check the "runner.py" under each directory for more info

import sys, os, inspect

# Python 3 ONLY!
# Throw now before we go any further
if sys.version_info.major < 3:
    raise RuntimeError("This script must be run with Python 3 or higher!")

def __get_results_directory():
    results_directory = "Results"
    if not os.path.exists(results_directory):
        os.mkdir(results_directory)

    return results_directory
#end

def help(args = None):    
    __name__ = "__main__"
    supported_operations = [name for name, obj in inspect.getmembers(sys.modules[__name__]) if inspect.isfunction(obj) and "__" not in name]
    print()
    print("Language Benchmark Runner")
    print(supported_operations)
    print()
    print(" 'help' for this text")
    print(" 'init <language name>' to start implementing a new <language>")
    print(" 'clean <language>' to reset environment and remove all any previous setups")
    print(" 'do_setup <language>' to provision the environment for that language")
    print(" 'run <language> [space-separated arguments]' to run a given <language> implementation with a set of [arguments]")
    print(" 'verify <language> [space-separated arguments]' to check a given <language> against the reference")
    print(" 'benchmark <repetitions> <language> [space-separated arguments]' run an implementation and take an average time")
    print(" 'compare <comma-separated list of languages> <repetitions> [space-separated arguments]' run some implementations and compare the average time")
    print(" 'plot/boxplot <comma-separated list of languages> <repetitions> [space-separated arguments]' benchmark and plot the results")
    print(" 'table <comma-separated list of languages> <repetitions> [space-separated arguments]' benchmark and save a table with the results")
    print()
# end help

def __import_from(module, name):
    # answer taken from https://stackoverflow.com/a/8790077
    module = __import__(module, fromlist=[name])
    return getattr(module, name)
# end __import_from

def __game_runner_name(name):
    # Technically, this means we support both '/' and '\' as valid separators
    name = name.strip('/').strip('\\')
    name = name.replace('/','.')
    name = name.replace('\\','.')

    return name + '.run'
#end

def __load_game(name):
    module_name = __game_runner_name(name)
    setup = __import_from(module_name, 'setup')
    build = __import_from(module_name, 'build')
    run_implementation = __import_from(module_name, 'run')
    return setup, build, run_implementation
#end

def __run_game(dir, setup, build, run_implementation, sub_args, repetitions=1):
    import time
    working_dir = os.getcwd()
    os.chdir(dir)

    setup()
    build()

    print("========== Starting Run ==========")
    times = []
    for _ in range(repetitions):
        start_time = time.perf_counter()
        run_implementation(sub_args)
        end_time = time.perf_counter()
        times.append(end_time - start_time)
    #end for
    print("========== Finishing Run ==========")
    os.chdir(working_dir)

    round_digits = 3
    if repetitions == 1:
        print("")
        print("Run time: {} seconds".format(round(times[0], round_digits)))
        print("")
        return times

    average = round(sum(times) / repetitions, round_digits)
    print("")
    print("{} repetitions - average run time: {} seconds".format(repetitions, average))
    print("")

    return times
#end

def init(args):
    dir_name = args[0]
    if ',' in dir_name:
        raise ValueError("Cannot have ',' in the implementation name.")

    resolved_path = os.path.join(os.getcwd(), dir_name)
    if not os.path.exists(resolved_path):
        os.mkdir(dir_name)

    run_file_path = os.path.join(resolved_path, "run.py")
    if os.path.exists(run_file_path):
        raise SystemError("This directory already has a 'run.py'")
    
    with open(os.path.join(dir_name,"run.py"), 'w') as run_file:
        run_file.write("""#!/usr/bin/env python

def setup():
    raise NotImplementedError("This must be implemented before running")
#end run

def build():
    raise NotImplementedError("This must be implemented before running")
#end run

def run(cmd_args):
    raise NotImplementedError("This must be implemented before running")
#end run

if __name__=="__main__":
    import sys, os

    setup()
    build()
    if os.path.basename(sys.argv[0]) == os.path.basename(__file__):
        run(sys.argv[1:])
# end main
        """)

    print("Initialized '{}'".format(dir_name))
    print("Make sure to fill in the 'setup', 'build' and 'run' methods")
#end init

def run(args, reps=1):
    dir_name = args[0]
    if not os.path.exists(os.path.join(os.getcwd(), dir_name) ) :
        raise NameError("Could not find directory '{}'".format(dir_name))

    print("Running trial implemented in '{}'".format(args[0]))

    sub_args = args[1:]
    print("Arguments: {}".format(sub_args))
    print("")

    setup, build, run_implementation = __load_game(dir_name)
    return __run_game(dir_name, setup, build, run_implementation, sub_args, repetitions=reps)
#end run

def verify(args):
    from difflib import context_diff

    baseline = 'python'
    comparison = args[0]
    output_file = 'reference.patch'

    run([baseline] + args[1:])
    run(args)
    
    diff = None
    with open(os.path.join(baseline, output_file), 'r') as baseline_output:
        with open(os.path.join(comparison, output_file), 'r') as comparison_output:
            diff = list(context_diff(baseline_output.readlines(), comparison_output.readlines()))

    # This lambda is a shorthand for when verification might fail
    verification_fails_with = lambda entries,pattern : len(entries) > 2 or not all(lines.startswith(pattern) for lines in entries)
    verification_failure = False

    conflicting_lines = [x for x in diff if x[0]=='!']
    if verification_fails_with(conflicting_lines, '! # Results'):
        print("'{}' verification failure: more than one conflicting line found (other than file header)".format(comparison))
        verification_failure = True
    
    additional_lines = [x for x in diff if x[0]=='+']
    if verification_fails_with(additional_lines, '+ {}'.format(os.linesep)):
        print("'{}' verification failure: additional lines of difference are not newlines ({})".format(comparison, os.linesep))
        verification_failure = True

    if verification_failure:
        raise NotImplementedError("Check the implementation of {} against {}".format(comparison, baseline))

    print()
    print("'{}' meets the implementation criteria".format(comparison))
#end verify

def benchmark(args, return_times = False):
    import time

    reps = int(args[0])
    times = run(args[1:], reps)

    if return_times:
        return times

    return round( sum(times) / reps , 5)
#end benchmark

def __find_all_implementations():
    return [ root[2:]
             for root, dirs, files in os.walk('.')
             for name in files
             if name.endswith('run.py')]
#end find all

def clean(args):
    working_dir = os.getcwd()
    dir_names = args[0].split(',')
    if len(dir_names) == 1 and dir_names[0] == 'all':
        dir_names = __find_all_implementations()
        print("Selected by wildcard: {}".format(dir_names))

    for a_dir in dir_names:
        curr_dir = os.path.join(working_dir, a_dir)

        setup_file = os.path.join(curr_dir, 'setup.log')
        if os.path.exists(setup_file):
            os.remove(setup_file)
            print("Deleted {} setup".format(a_dir))
        else:
            print("Implementation '{}' did not have a setup.log file".format(a_dir))
    #end for
#end clean

def do_setup(args):
    working_dir = os.getcwd()
    dir_names = args[0].split(',')
    if len(dir_names) == 1 and dir_names[0] == 'all':
        dir_names = __find_all_implementations()
        print("Selected by wildcard: {}".format(dir_names))

    for a_dir in dir_names:
        setup, _, __ = __load_game(a_dir)

        os.chdir(a_dir)
        try:
            setup()
        except Exception as e:
            print("Setup failed for {}!".format(module_name))
            print(e)
        finally:
            os.chdir(working_dir)
    #end for
#end clean

def compare(args, return_time_list = False, print_results = True):
    working_dir = os.getcwd()
    dir_names = args[0].split(',')
    using_all_dirs = False
    if len(dir_names) == 1 and dir_names[0] == 'all':
        dir_names = __find_all_implementations()
        print("Selected by wildcard: {}".format(dir_names))
        using_all_dirs = True

    repetitions = args[1]
    results = {}

    # check all implementations exist before proceeding
    if not using_all_dirs:
        for implementation in dir_names:
            if not os.path.exists(implementation):
                import errno
                print("\n '{}' is not a valid implementation. Check below for valid implementations!".format(implementation))
                print("Languages: {}".format(__find_all_implementations()))
                raise FileNotFoundError(errno.ENOENT, os.strerror(errno.ENOENT), implementation)

    for implementation in dir_names:
        sub_args = [repetitions, implementation]
        sub_args.extend(args[2:])
        try:
            results[implementation] = benchmark(sub_args, return_times=return_time_list)
        except ValueError as ve:
            print("Check your arguments!")
            raise ve
        except Exception as e:
            print(e)
            print()
            # We need to go back to our working directory to continue
            os.chdir(working_dir)
            results[implementation] = None
    # end for

    if not print_results:
        return results

    print("Ran {} iterations of implementation: {}".format(repetitions, dir_names))
    keys = list(results.keys())
    keys.sort()
    for lang in keys:
        print("{}: {} seconds".format(lang, results[lang]))

    return results
#end compare

def __plot(args, use_mean = False):
    # internal, shared implementation of plot

    # do a comparison, but get all results back
    benchmark_results = compare(args, return_time_list = not use_mean, print_results= False)

    ordered_results = []
    avg = lambda collection: sum(collection)/len(collection)
    key_func = lambda x : avg(x[1]) if isinstance(x[1],(float, int)) else 0
    
    if use_mean:
        # We want avg to be a no-op since it is already averaged
        avg = lambda x: x
    
    ordered_results = [(lang, results) for lang,results in benchmark_results.items()]
    ordered_results.sort(key = key_func)
    
    return ordered_results
#end __plot

def __render_and_save(a_plot, data_dictionary):
    import datetime, json

    timestamp = str(datetime.datetime.now().isoformat()).split('.')[0].replace(":","").replace("-","")

    # Try saving the interactive file
    try:
        filename = "{}-Results-interactive.html".format(timestamp) 
        a_plot.render_to_file(os.path.join(__get_results_directory(), filename))
    except Exception as e:
        print("Could not render in browser!")
        print(e)

    # Try opening it in a web browser
    try:
        print("Opening table in web browser...")
        #a_plot.render_in_browser()
    except Exception as e:
        print("Could not render in browser!")
        print(e)

    # Try saving the png locally
    try:
        print("Saving locally...")
        filename = "{}-Results.png".format(timestamp)
        a_plot.render_to_png(os.path.join(__get_results_directory(), filename))
    except Exception as e:
        print("Unable to save PNG chart")
        print(e)
    
    # Try saving a local copy as a table of values
    try:
        result_table = a_plot.render_table(style=True, transpose=True)
        if result_table is not None:
            filename ="{}-Results.html".format(timestamp) 
            with open(os.path.join(__get_results_directory(), filename), 'w') as output:
                output.write(result_table)
    except Exception as e:
        print("Unable to save table of results")
        print(e)

    # Last, make a json copy
    try:        
        filename = "{}-Results.json".format(timestamp) 
        with open(os.path.join(__get_results_directory(), filename), 'w') as output:
            json.dump(data_dictionary, output, sort_keys=True, indent=4, separators=(',', ': '))
    except Exception as e:
        print("Unable to save json data")
        print(e)
#end

def __get_dir_size(dir_path):
    import os

    total_size = 0
    total_files = 0

    for directory, _, filenames in os.walk(dir_path):
        for a_file in filenames:
            filepath = os.path.join(directory, a_file)
            total_files += 1
            try:
                total_size += os.path.getsize(filepath)
            except:
                pass
    #end for

    # return a tuple containing files at [0] and total size at [1]
    return (total_files,total_size)
#end 

# Taken from https://stackoverflow.com/a/1094933
def __sizeof_standard(num, suffix='B'):
    for unit in ['','K','M','G','T','P','E','Z']:
        if abs(num) < 1000.0:
            return "%3.1f %s%s" % (num, unit, suffix)
        num /= 1000.0
    return "%.1f %s%s" % (num, 'Y', suffix)
#end sizeof

def __get_run_metadata(args, method=None):
    # return the metadata of the run when using compare, plot and table commands

    metadata = {}
    metadata['languages'] = args[0]
    metadata['repetitions'] = args[1]
    metadata['directories'] = []
    
    # For the two directories
    for a_dir in [args[2], args[3]]:
        dir_metadata = {}
        dir_metadata['name'] = a_dir
        dir_stat = __get_dir_size(a_dir)
        dir_metadata['size'] = __sizeof_standard(dir_stat[1])
        dir_metadata['files'] = dir_stat[0]
        metadata['directories'].append(dir_metadata)

    # Log additional options
    metadata['options'] = args[4:]

    # if we were given the operation name, put that in
    if method is not None:
        metadata['operation'] = method
    
    return metadata
#end 

def __get_additional_title_info(args):
    # retrieves the text to put between parens on the title
    additional_text = "in seconds, lower is better"

    try:
        # check the number of repetitions
        repetitions = int(args[1])
        if repetitions == 1:
            repetitions_text = "single repetition"
        else:
            repetitions_text = "{} repetitions".format(repetitions)

        # Stat the two directories
        dir_a, dir_b = args[2], args[3]
        dir_a_stat = __get_dir_size(dir_a)
        dir_b_stat = __get_dir_size(dir_b)

        # sum up the contribution in size and files for both
        total_files = dir_a_stat[0] + dir_b_stat[0]
        total_size = dir_a_stat[1] + dir_b_stat[1]

        # format and return
        additional_text = "{}, {} files, {}".format(repetitions_text,total_files, __sizeof_standard(total_size))
    finally:
        return additional_text
#end 

def plot(args):
    import pygal
    
    benchmark_results = __plot(args, use_mean= True)
    bar_chart = pygal.HorizontalBar()
    bar_chart.title = 'Language benchmark results\n({})'.format(__get_additional_title_info(args))

    [bar_chart.add(x[0], x[1]) for x in benchmark_results]
    result_data = { x[0]:x[1] for x in benchmark_results}
    result_data['_metadata'] = __get_run_metadata(args, 'plot')

    __render_and_save(bar_chart, result_data)

    print("Done")
#end plot

def boxplot(args):
    import pygal

    benchmark_results = __plot(args, use_mean= False)

    box_plot = pygal.Box()
    box_plot.title = 'Language benchmark results\n({})'.format(__get_additional_title_info(args))

    [box_plot.add(lang, results) for lang,results in benchmark_results]
    result_data = { lang:results for lang,results in benchmark_results}
    result_data['_metadata'] = __get_run_metadata(args, 'boxplot')

    __render_and_save(box_plot, result_data)

    print("Done")
#end plot

def table(args):
    import pygal, webbrowser, datetime, json

    # remove any "--" args since we want to inject our own
    removable_args = [flag for flag in args if flag.startswith('--')]
    for flag in removable_args:
        print("Removing argument '{}'".format(flag))
        args.remove(flag)

    checksums = ["md5", "sha1", "sha256", "adler32", "crc32"]
    languages = args[0].split(',')
    results = {}
    for entry in checksums:
        results[entry] = compare(args + ['--{}'.format(entry)], return_time_list = False, print_results= False)
    
    chart = pygal.Bar()
    chart.title = 'Language benchmark results\n({})'.format(__get_additional_title_info(args))
    chart.x_labels = checksums

    # for every language, insert a list into the chart with the values of each checksum
    if len(languages) == 1 and languages[0] == 'all':
        dir_names = __find_all_implementations()
        languages = dir_names
        print("Selected by wildcard: {}".format(languages))

    [chart.add(lang, [results[sample][lang] for sample in checksums]) for lang in languages]

    chart.value_formatter = lambda x: '%.3f s' % x if x is not None else 'N/A'
    timestamp = str(datetime.datetime.now().isoformat()).split('.')[0].replace(":","").replace("-","")

    # Save the HTML Table
    result_file_name = os.path.join(__get_results_directory(), '{}-Results.'.format(timestamp))
    result_table = chart.render_table(style=True, transpose=True)
    if result_table is not None:
        with open(result_file_name + 'html', 'w') as output:
            output.write(result_table)

    # Save the JSON Data
    data_dictionary = { lang:{ sample:results[sample][lang] for sample in checksums} for lang in languages }
    data_dictionary['_metadata'] = __get_run_metadata(args, 'table')
    with open(result_file_name + 'json', 'w') as output:
        json.dump(data_dictionary, output, sort_keys=True, indent=4, separators=(',', ': '))

    print("Opening table in web browser...")
    try:
        #webbrowser.open("file://{}".format(os.path.join(os.getcwd(), result_file_name+'html')))
        pass
    finally:
        print("Done")
# end table

# Program entry point
if __name__=="__main__":
    args = sys.argv[1:]
    working_dir = os.getcwd()
    __name__ = "__runner__"

    try:
        operation_name = args[0]
        operation = eval(operation_name)
        operation(args[1:])
    except Exception as e:
        if len(args) > 0:
            print("")
            print("Encountered an issue while running: \"{}\"".format(e))
        help()
        os.chdir(working_dir)
# end main
