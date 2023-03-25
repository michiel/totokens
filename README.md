# totokens

## Installation

Download a build for your platform from [Releases](https://github.com/michiel/totokens/releases)

## Example generation

```
/tmp $ ~/bin/totokens export-dir --input ~/src/Vulnerable-Code-Snippets/Format\ String\ Attacks --output dump.txt
2023-03-25T14:08:22.343976Z DEBUG totokens: export-dir: input path : /Users/me/src/Vulnerable-Code-Snippets/Format String Attacks
2023-03-25T14:08:22.344039Z DEBUG totokens: export-dir: output file : dump.txt
2023-03-25T14:08:22.344572Z DEBUG totokens::util: added to ignore pattern : ^/Users/me/src/Vulnerable\-Code\-Snippets/Format String Attacks/\.gptignore
2023-03-25T14:08:22.344579Z DEBUG totokens::util: added to ignore pattern : ^package\-lock\.json
2023-03-25T14:08:22.344582Z DEBUG totokens::util: added to ignore pattern : ^yarn\.lock
2023-03-25T14:08:22.344584Z DEBUG totokens::util: added to ignore pattern : ^\.git
/tmp $ cat dump.txt| pbcopy 
```

## Example prompt

```
The code below is a git repository, with files separated by '--------' followed by the full path of the filename in the repository.

List the security weaknesses found in this repository, score the weaknesses using CWE and provide the output in a concise json format
```
