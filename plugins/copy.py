"""
Helper scripts for developing operations.
This code automatically copy the plugins inside the folder and put them inside the application folder used.
This helps run the app as if it alredy had the plugins installed.
"""
import os
import shutil
import sys

def ignore_files(src: str, names: list[str]):
    return [".venv", "node_modules"]
    

def main():
    for f in os.listdir("./plugins/"):
        if f != "copy.py":
            shutil.copytree(
                os.path.join("plugins", f),
                os.path.join(sys.argv[1], f),
                ignore=ignore_files,
                dirs_exist_ok=True
            )
    
if __name__ == "__main__":
    main()