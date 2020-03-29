#!/usr/bin/env python

def setup():
    print("If you can run this script, setup is complete")
#end run

def build():
    print("No build needed for reference implementation")
#end run

def run(cmd_args):
    import subprocess, os
    # The only change is that we run pypy instead of python3
    process_args = ["pypy3", os.path.join(os.getcwd(), '..', "program.py")] + cmd_args
    
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