syntax  keyword exampleComment      Aaaa
syntax  keyword exampleConstant     Bbbb
syntax  match   exampleString       /C/
syntax  match   exampleIdentifier   /D/
syntax  match   exampleError        /Eeee/

highlight   link    exampleComment      Comment
highlight   link    exampleConstant     Constant
highlight   link    exampleString       String
highlight   link    exampleIdentifier   Identifier
highlight   link    exampleError        Error

let b:current_syntax = "exp"
