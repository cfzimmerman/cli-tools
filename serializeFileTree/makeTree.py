import sys
import re
import json
from typing import List, Dict, Self, Optional


class FileTree:
    """Represents a collection of file paths as a JSON object"""

    __paths: List[str]
    __tree: Dict[str, Dict]
    __exclude: Optional[str]

    def __init__(self):
        self.__tree = {}
        self.__paths = []
        self.__exclude = None
        if len(sys.argv) > 0:
            self.__exclude = sys.argv[1]

    def readIn(self) -> Self:
        """Reads and stores a list of file paths from stdin"""
        for line in sys.stdin:
            self.__paths.append(line.strip())
        return self

    def buildTree(self) -> Self:
        """Builds and stores a JSON file tree from the input paths"""
        for path in self.__paths:
            pathList = path.split("/")
            self.__appendPath(pathList, 0, self.__tree)
        return self

    def writeOut(self) -> ():
        """Writes the tree to stdout as a JSON string"""
        sys.stdout.write(json.dumps(self.__tree))

    def __appendPath(self, path: List[str], ind: int, parent: Dict) -> ():
        """Adds a filepath to the JSON file tree"""
        name = path[ind]
        if ind == len(path) - 1:
            if "files" not in parent:
                parent["files"] = []
            parent["files"].append(name)
            return
        if name not in parent:
            parent[name] = {}
        newParent = parent[name]
        if self.__exclude and len(name) and re.search(self.__exclude, name):
            return
        self.__appendPath(path, ind + 1, newParent)


FileTree().readIn().buildTree().writeOut()
