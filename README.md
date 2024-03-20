# About
    Conical is a modern systems language that is designed with a "simplicity first" mindset. While many modern languages strive to have as much abstraction as possible. Conical tries to be as **readable** as possible. This means that, while there **are** various different abstractions,that is not the goal of the language. For example, types for functions and variables are written like in C style languages (`type function(type arg);`) instead of the more modern convention of using '->' and ':' (`fn function(arg: type)->type`).

    The language also tries to be as explicit as possible; but, unlike C, if there is a choice between being explicit and being simple the simple approach will be prioritized.

    This combination of explicitness and simplicity allows for abstraction while having minimal syntax. For example, the language supports generic functions (`type function<T>(T arg);`) but unlike other languages the compiler will *not* infer the type when it is used. (`function<i32>(1);` instead of `function(1);`) this is done to help prevent bugs occuring where the wrong variable is passed to a function. 


# Getting Started

