# Snippet (or code) Generator

## Introduction
Small code generator, basic functionion is divided in three parts:
1.  Parsing a *.dl file content into a string (raw_data)
2.  Tokenize the raw_string (only tokenize valid tokens and keywords) (throw excpection on invalid_token) a valid file shall look like this
   ```
        @binary //tells the generator that the file is a source code for a binary and not a library (@lib)
        struct { //tells the to generate a struct
            readonly attribute u8 m_value // const uint8 m_value
            method getValue{
                in: {}
                out: { m_value } // uint8 getValue() -> m_Value;
            }
        }
   ```
3.  Lexical analizer:

![Alt](https://www.tutorialspoint.com/compiler_design/images/token_passing.jpg "Lexical analizer")

4. Generate code (into constructed stream)
5. Parse it to a source code file
6. Export the file