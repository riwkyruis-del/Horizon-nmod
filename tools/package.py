#!/usr/bin/env python3
"""Build a deterministic, source-only Horizon native mixin archive (Python 3.11+)."""
from pathlib import Path
import hashlib
import json
import tomllib
import zipfile

ROOT = Path(__file__).resolve().parents[1]
BUNDLE = ROOT / "mixins" / "dreadfall"
OUTPUT = ROOT / "releases" / "dreadfall-0.1.0.hnmod"
MIXINS_REVISION = "d9c83d7096b7ea5b83678da2efdfd6161f74997e"


def build():
    config = tomllib.loads((BUNDLE / "horizon-mixin.toml").read_text())
    mod = config["mod"]
    assert mod["id"] == BUNDLE.name and mod["version"] == "0.1.0"
    for entry in config["injection"]:
        assert (BUNDLE / entry["helper"]).is_file()
    for entry in config["module"]:
        assert (BUNDLE / entry["source"]).is_file()
    manifest = {
        "schema": 1, "id": mod["id"], "version": mod["version"], "game": "26.3",
        "horizon": config["compatibility"]["horizon_revision"], "mixins": MIXINS_REVISION,
    }
    entries = {"horizon-native.json": (json.dumps(manifest, indent=2, sort_keys=True) + "\n").encode()}
    for file in sorted(BUNDLE.iterdir()):
        if file.is_file() and file.suffix in (".rs", ".toml"):
            entries[file.relative_to(ROOT).as_posix()] = file.read_bytes()
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    with zipfile.ZipFile(OUTPUT, "w") as archive:
        for name, content in sorted(entries.items()):
            info = zipfile.ZipInfo(name, (2026, 1, 1, 0, 0, 0))
            info.compress_type = zipfile.ZIP_DEFLATED
            info.external_attr = 0o100644 << 16
            archive.writestr(info, content)
    print(f"{OUTPUT.relative_to(ROOT)}  sha256={hashlib.sha256(OUTPUT.read_bytes()).hexdigest()}")
    return OUTPUT


if __name__ == "__main__":
    build()
