- [x] your first task is to make sure that all places in the tokenizer and parse where were using temperary ways of storing state like using         │
│   strings for what should be enums get turned into propper enums for good type safety, try to apply these principles to the entire codebase 

- [x] make sure there are a set of keywords that will also yeild a tokentype of keyword when the previous identifier is in the list of keywords

- [x] write basic parser tests
- [x] organize the file structure




- [x] as of now, when showing an ast it gets displayed as something like: "{ name: "num", type_: Int, value: Some(OperatorUse(OperatorUse { left: OperatorUse(OperatorUse { left: Token(Token { type_: Identifier, value: "b" }), operator: "*", right: Token(Token { type_: Number, value: "9" }) }), operator: "+", right: FunctionCall(FunctionCall { name: "a", args: [Token(Token { type_: Identifier, value: "a" }), OperatorUse(OperatorUse { left: Token(Token { type_: Number, value: "3" }), operator: "*", right: Token(Token { type_: Number, value: "0" }) })] }) })) }", i would like if displaying an ast was alot neater, weather you overide the normal display for each ast node or you made a display function that took in a depth and so on




- [x] add more parser tests that assert the entire structure of ast nodes at a time instead of a bunch of individual asserts



- [x] put ast in its own module with a file for the core ast structure with other files for other concerns like display and so on. same idea for the tokenizer where it gets its own module with a file for the core tokenizer logic and other files for other concerns like token and token type

