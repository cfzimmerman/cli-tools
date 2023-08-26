import sys
import json
from typing import List, Dict, Self


class FileTree:
    """Represents a collection of file paths as a JSON object"""

    __paths: List[str] = []
    __tree: Dict[str, Dict]

    def __init__(self):
        self.__tree = {}
        self.__paths = []

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
        self.__appendPath(path, ind + 1, newParent)


FileTree().readIn().buildTree().writeOut()
