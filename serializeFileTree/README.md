This tool accepts a list of file paths from stdin and returns
a JSON object representing those paths via stdout.
To use the tool, call the Python script with
`bash run.sh`, try with your own preferred stdin, or use the
jsonifyDir tool to explore your own filesystem.
(paths.txt is just modified ChatGPT sample data)
The script accepts a cli arg for regex patterns to not
recursively search. See run.sh for an example.
