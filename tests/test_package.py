import hashlib
import importlib.util
import json
from pathlib import Path
import re
import tomllib
import unittest
import zipfile

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location("package", ROOT / "tools" / "package.py")
package = importlib.util.module_from_spec(spec)
spec.loader.exec_module(package)


class PackageTests(unittest.TestCase):
    def test_archive_manifest_and_reproducibility(self):
        path = package.build()
        first = hashlib.sha256(path.read_bytes()).hexdigest()
        package.build()
        self.assertEqual(first, hashlib.sha256(path.read_bytes()).hexdigest())
        with zipfile.ZipFile(path) as archive:
            names = archive.namelist()
            self.assertEqual(len(names), len(set(names)))
            manifest = json.loads(archive.read("horizon-native.json"))
            self.assertEqual(set(manifest), {"schema", "id", "version", "game", "horizon", "mixins"})
            self.assertEqual(manifest["schema"], 1)
            self.assertEqual(manifest["game"], "26.3")
            for key in ("horizon", "mixins"):
                self.assertRegex(manifest[key], r"^[0-9a-f]{40}$")
            prefix = f"mixins/{manifest['id']}/"
            self.assertTrue(all(n == "horizon-native.json" or n.startswith(prefix) for n in names))
            mixin = tomllib.loads(archive.read(prefix + "horizon-mixin.toml").decode())
            self.assertEqual(mixin["mod"]["id"], manifest["id"])
            self.assertEqual(mixin["mod"]["version"], manifest["version"])
            self.assertEqual(mixin["compatibility"]["horizon_revision"], manifest["horizon"])
            for item in mixin["injection"]:
                self.assertIn(prefix + item["helper"], names)
            for item in mixin["module"]:
                self.assertIn(prefix + item["source"], names)
            self.assertIsNone(archive.testzip())


if __name__ == "__main__":
    unittest.main()
