#!/usr/bin/env python

output_file_name = 'cpp.out'

preferred_compiler = 'clang++' # or g++, up to you

def setup():
    import os, datetime, subprocess
    if os.path.exists(os.path.join(os.getcwd(), "setup.log")):
        print("'setup.log' exists. C++ implementation setup correctly")
        return

    # We can't really setup this successfully, we need a build system like CMake or scons for xplat support
    print("Requires libboost-filesystem-dev libcrypto++-dev libcrypto++9v5")
    try:
        with open('setup.log', 'w') as logFile:
            logFile.write("# This is an autogenerated file made by 'run.py' on {}\n".format(datetime.datetime.now()))
            logFile.write("# => DO NOT DELETE THIS FILE OR SETUP WILL BE CALLED AGAIN\n")
            logFile.flush()
            subprocess.run([preferred_compiler, "-v"], stdout = logFile, stderr = logFile, check=True)
            logFile.write("\n# Setup completed on {}".format(datetime.datetime.now()))
        #end logFile
    except Exception as e:
        print(e)
        if os.path.exists('setup.log'):
            os.remove('setup.log')
#end run

def build():
    import subprocess, os

    # remove the previous build
    if os.path.exists(output_file_name):
        os.remove(output_file_name)

    source_files = [x for x in os.listdir('.') if x.endswith('.cpp')]
    c_libs = ['-lboost_system', '-lboost_filesystem', '-lpthread', '-lcryptopp']
    c_defs = ['-DNDEBUG', '-DCRYPTOPP_CXX11', '-DCRYPTOPP_CXX11_NOEXCEPT']

    # For older versions of clang++/g++, the order of the source files matters!
    process_args = [preferred_compiler] + source_files + ['-std=c++14',  '-Wall', '-pedantic',  '-Ofast', '-o', output_file_name] + c_libs + c_defs
    subprocess.call(process_args)

    if os.path.exists(output_file_name):
        print("Built C++ implementation as '{}'".format(output_file_name))
    else:
        raise AssertionError("Build failed")
#end run

def run(cmd_args):
    import subprocess
    process_args = ["./{}".format(output_file_name)] + cmd_args
    retcode = subprocess.call(process_args)
    if retcode != 0:
        raise RuntimeError("Program run returned non-zero exit code")
#end run

if __name__=="__main__":
    import sys, os

    setup()
    build()
    if os.path.basename(sys.argv[0]) == os.path.basename(__file__):
        run(sys.argv[1:])
# end main
        