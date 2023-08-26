This tool uses my filepath serialization tool to print the contents
of the current directory and all child directories as a JSON object.
I put the function below in my .zshenv, and it's actually really nice
for quickly exploring new projects.

```zsh
function explore_dir () {
  bash /Users/cory/projects/cli-tools/jsonifyDir/jsonifyDir.sh "$(pwd)"
}
```
