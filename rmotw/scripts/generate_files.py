#!/bin/env python
import os, time, sys, shutil, string, random, time
from datetime import datetime, timedelta
from pathlib import Path

bird_choices = string.ascii_letters

def generate_photoset(path, count, date, start=0):
    prefix = "IMG_"
    
    for i in range(count):
        date = date + timedelta(seconds=1)
        subjects = "".join(random.choices(bird_choices, k=random.randrange(1,4)))
        filename = f"{prefix}{subjects}_{i+start:04}.jpg"
        filepath = path / filename
        with open(filepath, "w") as f:
            f.write("some bytes")
        modTime = time.mktime(date.timetuple())
        os.utime(filepath, (modTime, modTime))

months = ["qlyrk", "epont", "pikith"]

if __name__ == "__main__":
    if len(sys.argv) == 1:
        print("Usage: python scripts/generate_files.py <PATH_FOR_IMAGES>")
        print("Where <PATH_FOR_IMAGES> is the directory to use for the fake images")
        sys.exit(-1)
    dest_path = sys.argv[1]
    dest_path = Path(dest_path)
    if dest_path.exists():
        shutil.rmtree(dest_path)
    if not dest_path.exists():
        os.mkdir(dest_path)
    now = datetime.now() - timedelta(seconds=1000)
    for (c, i) in enumerate(range(10000, 0, -100)):
        generate_photoset(dest_path, 100, now, c*100)
        now = now - timedelta(days=random.randrange(10, 30))

    for month in months:
        mpath = dest_path / f"{month}{now.year}"
        os.mkdir(mpath)
        generate_photoset(mpath, random.randrange(30, 100), now)
        now = now - timedelta(weeks=random.randrange(10, 30))

    print(f"(Fake) images now ready at {dest_path}")
