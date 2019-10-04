from __future__ import print_function
import sys

if __name__ == "__main__":
    print(len(sys.argv))
    if len(sys.argv) != 3:
        print("Usage: create_run.py min_row max_row", file=sys.stderr)
        exit(-1)

    min_row = int(sys.argv[1])
    max_row = int(sys.argv[2])
    start_num = int(min_row / 1000)
    end_num = int(max_row / 1000)

    f = open("run.sh", "a")
    for i in range(start_num, end_num + 1):
        s = "cargo run ../data/input" + str(i) + ".csv " + str(i * 1000) + " " + str(100) + "\n"
        print(s)
        f.write(s)
    
    f.close()

