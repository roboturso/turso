#!/usr/bin/env python3
"""Regression test for issue #3729.

`.exit` must checkpoint the WAL on shutdown just like `.quit` does, so that
the main database file is self-contained. Before the fix, `.exit` called
`std::process::exit` without closing the connection, leaving all committed
data only in the `-wal` file; copying or moving just the `.db` file then
silently lost the data.
"""

import os
import shutil
import subprocess
import tempfile
from pathlib import Path

from cli_tests import console


def run_shell(exe: str, script: str) -> str:
    # No extra flags: the default `scripts/limbo-sqlite3` wrapper already
    # passes `-q`, and repeating it is an error.
    result = subprocess.run(
        [exe],
        input=script,
        capture_output=True,
        text=True,
        timeout=60,
    )
    return result.stdout


def main():
    exe = os.environ.get("SQLITE_EXEC", "./scripts/limbo-sqlite3")
    with tempfile.TemporaryDirectory() as tmp:
        db = Path(tmp) / "rr.db"
        script = (
            f".open {db}\n"
            "CREATE TABLE t31(a,b);\n"
            "INSERT INTO t31 VALUES(1,4), (2,3), (1,3);\n"
            ".exit\n"
        )
        run_shell(exe, script)

        # Copy only the main database file, deliberately leaving the WAL
        # behind. If `.exit` checkpointed properly, the copy is
        # self-contained and the data is visible.
        copy = Path(tmp) / "copy" / "rr.db"
        copy.parent.mkdir()
        shutil.copyfile(db, copy)

        out = run_shell(exe, f".open {copy}\nSELECT count(*) FROM t31;\n.exit\n")
        lines = [line.strip() for line in out.splitlines() if line.strip()]
        if "3" not in lines:
            console.error(f"expected 3 rows in t31 from the copied database file, got output: {out!r}")
            console.error("`.exit` did not checkpoint the WAL into the main database file (issue #3729)")
            exit(1)
    console.info("test-exit-checkpoint passed")


if __name__ == "__main__":
    main()
