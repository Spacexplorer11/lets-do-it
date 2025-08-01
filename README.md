# Let's do it!
## What's this?
Let's do it is a Rust CLI tool for your to-do tasks. 
As of now, it doesn't have much uniqueness, its just your run-of-the-mill CLI to-do list.  
It does now save your tasks in tasks.txt so they persist between runs!
Simple functions:
- Add
- Update
- Delete
- List
- Help  

Nothing else at the moment.  
However, I have planned to allow single-liners like ldt --list, 
or something like cargo run -- --list. I'll use clap (the library) for this.

## What libraries do I use?
Check cargo.toml!  
But right now, I use rustyline for input, indexmap for well the IndexMap and serde_json for file saving & loading!.

## How can I use it?
Just download the [latest release](https://github.com/Spacexplorer11/lets-do-it/releases/latest) for your OS from the Github Releases!  
Rust compiles to native binaries making it super simple for you!

## AI Transparency
AI, that brand new thing everyone uses. Yeah, well I use it too. However, I'd like to be transparent.  
I used AI for the first commit (after initial) to setup the base of the app and take input.   
After that, I used AI purely for learning and it taught me about IndexMap, however the code was written by me.  
Then I used AI to generate the release.yml file. That file was 100% AI generated. This is because of my lack of knowledge about Github Actions and YAML.  
However, after that, no more AI generated code was used. I did use AI to find out how to save the tasks, but no AI code was used.  
And in release.yml, Gemini 2.5 Pro is used for making release notes, so obviously, all the releases are AI generated.

## Please consider leaving a star