import os
import ast
import tokenizer
import commandeer

proc parseArgs() : (seq[string], string) =
    commandline:
        arguments input, string
        option output, string, "output", "o"
        exitoption "help", "h",
                   "Usage: comp [--output|-o] <inputs>..."
        errormsg "Invalid usage"

    for x in input:
        echo("inp: ", x)
    return (input, output)

proc main() =
    let (inp, outp) = parseArgs()
    var content =  ""
    for x in inp:
        content = readFile(x)
    let tokens = tokenizer.tokenize("")


main()