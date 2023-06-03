# simple-java-parser
2023-1 Compiler Assignment: Making Parser for Simple Java  
team_id: 67

## Todo
- 코드에 주석 달기
- 문서 pdf로 변환하기

## 실행
```bash
$ ./syntax_analyzer testcase/sample_input0.sj
[1/4] File name: testcase/sample_input0.sj

[2/4] File contents:
vtype id semi

[3/4] Read tokens:
[Vtype Id Semi EOL]

[4/4] Parse tree:
CODE
├── VDECL
│   ├── Vtype
│   ├── Id
│   └── Semi
└── CODE

Accepted!
```
## build
```bash
# rust 설치 (https://www.rust-lang.org/learn/get-started)
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# 컴파일
$ cargo build --release
$ cp target/release/simple-java-parser syntax_analyzer
# 실행
$ ./syntax_analyzer testcase/sample_input0.sj
```
## 수정된 CFG
```
00: CODE' -> CODE
01: CODE -> VDECL CODE
02: CODE -> FDECL CODE
03: CODE -> CDECL CODE
04: CODE -> ''
05: VDECL -> vtype id semi
06: VDECL -> vtype ASSIGN semi
07: ASSIGN -> id assign RHS
08: RHS -> EXPR
09: RHS -> literal
10: RHS -> character
11: RHS -> boolstr
12: EXPR -> EXPR addsub EXPR'
13: EXPR -> EXPR'
14: EXPR' -> EXPR' multdiv EXPR''
15: EXPR' -> EXPR''
16: EXPR'' -> lparen EXPR rparen
17: EXPR'' -> id
18: EXPR'' -> num
19: FDECL -> vtype id lparen ARG rparen lbrace BLOCK RETURN rbrace
20: ARG -> vtype id MOREARGS
21: ARG -> ''
22: MOREARGS -> comma vtype id MOREARGS
23: MOREARGS -> ''
24: BLOCK -> STMT BLOCK
25: BLOCK -> ''
26: STMT -> VDECL
27: STMT -> ASSIGN semi
28: STMT -> if lparen COND rparen lbrace BLOCK rbrace ELSE
29: STMT -> while lparen COND rparen lbrace BLOCK rbrace
30: COND -> COND comp boolstr
31: COND -> boolstr
32: ELSE -> else lbrace BLOCK rbrace
33: ELSE -> ''
34: RETURN -> return RHS semi
35: CDECL -> class id lbrace ODECL rbrace
36: ODECL -> VDECL ODECL
37: ODECL -> FDECL ODECL
38: ODECL -> ''
```
### 변경사항 1 (CODE)
최상단인 `CODE`를 가리키는 `CODE' -> CODE`를 추가하였습니다.
```
CODE -> VDECL CODE
CODE -> FDECL CODE
CODE -> CDECL CODE
CODE -> ''
```
->
```
CODE' -> CODE
CODE -> VDECL CODE
CODE -> FDECL CODE
CODE -> CDECL CODE
CODE -> ''
```
### 변경사항 2 (COND)
`COND comp COND comp COND`의 ambiguous를 해결하기 위해 수정하였습니다.
```
COND -> COND comp COND
COND -> boolstr
```
->
```
COND -> COND comp boolstr
COND -> boolstr
```
### 변경사항 3 (EXPR)
`addsub`, `multdiv`, `lparen`, `rparen`, `id`, `num`의 우선순위에 대한 ambiguous를 해결하기 위해 수정하였습니다.
```
EXPR -> EXPR addsub EXPR
EXPR -> EXPR multdiv EXPR
EXPR -> lparen EXPR rparen
EXPR -> id
EXPR -> num
```
->
```
EXPR -> EXPR addsub EXPR'
EXPR -> EXPR'
EXPR' -> EXPR' multdiv EXPR''
EXPR' -> EXPR''
EXPR'' -> lparen EXPR rparen
EXPR'' -> id
EXPR'' -> num
```
## parsing table
https://jsmachines.sourceforge.net/machines/slr.html
![parsing table](img/parsing_table.jpg)

## 동작 과정
이 parser는 네가지 단계에 거쳐 parsing tree를 생성합니다.  
Step 1. 인자로부터 파일 이름 가져오기  
Step 2. 파일 읽기  
Step 3. String을 whitespace으로 나누어 token 인식하기  
Step 4. 인식된 토큰을 parsing하여 parsing tree 생성하기  
### Step 3 주요 struct, enum, procedure
#### read_tokens (In src/token_reader.rs)
String을 white space로 나누어 문자여 비교를 통해 Token의 배열을 return하는 함수입니다.
```rust
pub fn read_tokens(contents: &String) -> Result<Tokens, UnknownTokenError> {
    let mut tokens = VecDeque::new();

    for word in contents.split_whitespace() {
        let token = match word {
            "vtype" => Token::Vtype,
            "num" => Token::Num,
            "character" => Token::Character,
	    (생략...)
            // token 인식을 실패하면 UnknownTokenError에 정보를 담아 return합니다.
            unknown_token => return Err(UnknownTokenError(unknown_token)),
        };
        tokens.push_back(Terminal(token));
    }

    // token을 모두 인식하였다면 마지막에 EOL token을 추가하고 return합니다.
    tokens.push_back(Terminal(Token::EOL));
    Ok(Tokens(tokens))
}
```
#### Token (In src/token_reader.rs)
terminal과 non-terminal, EOL을 나타내는 enum입니다.
```rust
pub enum Token {
    // terminals
    Vtype,      // for the types of variables and function
    Num,        // for signed integers
    Character,  // for a single character
    (생략...)

    // for EOL
    EOL,

    // non-terminals
    CODE,
    CODE_,
    VDECL,
    (생략...)
}
```
#### Node (In src/parser/mod.rs)
terminal과 non-terminal을 나타내며 동시에 tree의 node를 나타내는 enum입니다.  
read_tokens에서는 `Terminal(Token)`만 사용되며 `NonTerminal(Token, Vec<Node>)`는 Step 4 parse함수에서 사용됩니다.
```rust
pub enum Node {
    Terminal(Token),
    NonTerminal(Token, Vec<Node>),
}
```
#### Tokens (In src/parser/formatting.rs)
Node의 배열을 나타내는 struct입니다.  
여기서 VecDeque는 double-ended queue를 표현하는 Rust의 내장 collection입니다.
```rust
pub struct Tokens(pub VecDeque<Node>);
```
### Step 4 주요 struct, enum, procedure

## test case
### case 0
```
// In testcase/sample_input0.sj
vtype id semi
```
```bash
$ ./syntax_analyzer testcase/sample_input0.sj
[1/4] File name: testcase/sample_input0.sj

[2/4] File contents:
vtype id semi

[3/4] Read tokens:
[Vtype Id Semi EOL]

[4/4] Parse tree:
CODE
├── VDECL
│   ├── Vtype
│   ├── Id
│   └── Semi
└── CODE

Accepted!
```
### case 1
```
// In testcase/sample_input1.sj
vtype id semi
vtype id lparen rparen lbrace
    if lparen boolstr comp boolstr rparen lbrace
    rbrace
return id semi rbrace
```
```bash
$ ./syntax_analyzer testcase/sample_input1.sj
[1/4] File name: testcase/sample_input1.sj

[2/4] File contents:
vtype id semi
vtype id lparen rparen lbrace
    if lparen boolstr comp boolstr rparen lbrace
    rbrace
return id semi rbrace

[3/4] Read tokens:
[Vtype Id Semi Vtype Id Lparen Rparen Lbrace If Lparen Boolstr Comp Boolstr Rparen Lbrace Rbrace Return Id Semi Rbrace EOL]

[4/4] Parse tree:
CODE
├── VDECL
│   ├── Vtype
│   ├── Id
│   └── Semi
└── CODE
    ├── FDECL
    │   ├── Vtype
    │   ├── Id
    │   ├── Lparen
    │   ├── ARG
    │   ├── Rparen
    │   ├── Lbrace
    │   ├── BLOCK
    │   │   ├── STMT
    │   │   │   ├── If
    │   │   │   ├── Lparen
    │   │   │   ├── COND
    │   │   │   │   ├── COND
    │   │   │   │   │   └── Boolstr
    │   │   │   │   ├── Comp
    │   │   │   │   └── Boolstr
    │   │   │   ├── Rparen
    │   │   │   ├── Lbrace
    │   │   │   ├── BLOCK
    │   │   │   ├── Rbrace
    │   │   │   └── ELSE
    │   │   └── BLOCK
    │   ├── RETURN
    │   │   ├── Return
    │   │   ├── RHS
    │   │   │   └── EXPR
    │   │   │       └── EXPR_
    │   │   │           └── EXPR__
    │   │   │               └── Id
    │   │   └── Semi
    │   └── Rbrace
    └── CODE

Accepted!
```
### case 2
```
// In testcase/sample_input2.sj
class id lbrace
    vtype id semi
    vtype id assign id addsub lparen num multdiv id rparen semi

    vtype id lparen rparen lbrace
        id assign num multdiv num semi
        while lparen boolstr comp boolstr comp boolstr rparen lbrace
            id assign literal semi
            id assign boolstr semi
        rbrace
        return id semi
    rbrace

    vtype id lparen vtype id comma vtype id rparen lbrace
        if lparen boolstr rparen lbrace
        rbrace
        return num addsub id semi
    rbrace

rbrace
```
```bash
$ ./syntax_analyzer testcase/sample_input2.sj
[1/4] File name: testcase/sample_input2.sj

[2/4] File contents:
class id lbrace
    vtype id semi
    vtype id assign id addsub lparen num multdiv id rparen semi

    vtype id lparen rparen lbrace
        id assign num multdiv num semi
        while lparen boolstr comp boolstr comp boolstr rparen lbrace
            id assign literal semi
            id assign boolstr semi
        rbrace
        return id semi
    rbrace

    vtype id lparen vtype id comma vtype id rparen lbrace
        if lparen boolstr rparen lbrace
        rbrace
        return num addsub id semi
    rbrace

rbrace

[3/4] Read tokens:
[Class Id Lbrace Vtype Id Semi Vtype Id Assign Id Addsub Lparen Num Multdiv Id Rparen Semi Vtype Id Lparen Rparen Lbrace Id Assign Num Multdiv Num Semi While Lparen Boolstr Comp Boolstr Comp Boolstr Rparen Lbrace Id Assign Literal Semi Id Assign Boolstr Semi Rbrace Return Id Semi Rbrace Vtype Id Lparen Vtype Id Comma Vtype Id Rparen Lbrace If Lparen Boolstr Rparen Lbrace Rbrace Return Num Addsub Id Semi Rbrace Rbrace EOL]

[4/4] Parse tree:
CODE
├── CDECL
│   ├── Class
│   ├── Id
│   ├── Lbrace
│   ├── ODECL
│   │   ├── VDECL
│   │   │   ├── Vtype
│   │   │   ├── Id
│   │   │   └── Semi
│   │   └── ODECL
│   │       ├── VDECL
│   │       │   ├── Vtype
│   │       │   ├── ASSIGN
│   │       │   │   ├── Id
│   │       │   │   ├── Assign
│   │       │   │   └── RHS
│   │       │   │       └── EXPR
│   │       │   │           ├── EXPR
│   │       │   │           │   └── EXPR_
│   │       │   │           │       └── EXPR__
│   │       │   │           │           └── Id
│   │       │   │           ├── Addsub
│   │       │   │           └── EXPR_
│   │       │   │               └── EXPR__
│   │       │   │                   ├── Lparen
│   │       │   │                   ├── EXPR
│   │       │   │                   │   └── EXPR_
│   │       │   │                   │       ├── EXPR_
│   │       │   │                   │       │   └── EXPR__
│   │       │   │                   │       │       └── Num
│   │       │   │                   │       ├── Multdiv
│   │       │   │                   │       └── EXPR__
│   │       │   │                   │           └── Id
│   │       │   │                   └── Rparen
│   │       │   └── Semi
│   │       └── ODECL
│   │           ├── FDECL
│   │           │   ├── Vtype
│   │           │   ├── Id
│   │           │   ├── Lparen
│   │           │   ├── ARG
│   │           │   ├── Rparen
│   │           │   ├── Lbrace
│   │           │   ├── BLOCK
│   │           │   │   ├── STMT
│   │           │   │   │   ├── ASSIGN
│   │           │   │   │   │   ├── Id
│   │           │   │   │   │   ├── Assign
│   │           │   │   │   │   └── RHS
│   │           │   │   │   │       └── EXPR
│   │           │   │   │   │           └── EXPR_
│   │           │   │   │   │               ├── EXPR_
│   │           │   │   │   │               │   └── EXPR__
│   │           │   │   │   │               │       └── Num
│   │           │   │   │   │               ├── Multdiv
│   │           │   │   │   │               └── EXPR__
│   │           │   │   │   │                   └── Num
│   │           │   │   │   └── Semi
│   │           │   │   └── BLOCK
│   │           │   │       ├── STMT
│   │           │   │       │   ├── While
│   │           │   │       │   ├── Lparen
│   │           │   │       │   ├── COND
│   │           │   │       │   │   ├── COND
│   │           │   │       │   │   │   ├── COND
│   │           │   │       │   │   │   │   └── Boolstr
│   │           │   │       │   │   │   ├── Comp
│   │           │   │       │   │   │   └── Boolstr
│   │           │   │       │   │   ├── Comp
│   │           │   │       │   │   └── Boolstr
│   │           │   │       │   ├── Rparen
│   │           │   │       │   ├── Lbrace
│   │           │   │       │   ├── BLOCK
│   │           │   │       │   │   ├── STMT
│   │           │   │       │   │   │   ├── ASSIGN
│   │           │   │       │   │   │   │   ├── Id
│   │           │   │       │   │   │   │   ├── Assign
│   │           │   │       │   │   │   │   └── RHS
│   │           │   │       │   │   │   │       └── Literal
│   │           │   │       │   │   │   └── Semi
│   │           │   │       │   │   └── BLOCK
│   │           │   │       │   │       ├── STMT
│   │           │   │       │   │       │   ├── ASSIGN
│   │           │   │       │   │       │   │   ├── Id
│   │           │   │       │   │       │   │   ├── Assign
│   │           │   │       │   │       │   │   └── RHS
│   │           │   │       │   │       │   │       └── Boolstr
│   │           │   │       │   │       │   └── Semi
│   │           │   │       │   │       └── BLOCK
│   │           │   │       │   └── Rbrace
│   │           │   │       └── BLOCK
│   │           │   ├── RETURN
│   │           │   │   ├── Return
│   │           │   │   ├── RHS
│   │           │   │   │   └── EXPR
│   │           │   │   │       └── EXPR_
│   │           │   │   │           └── EXPR__
│   │           │   │   │               └── Id
│   │           │   │   └── Semi
│   │           │   └── Rbrace
│   │           └── ODECL
│   │               ├── FDECL
│   │               │   ├── Vtype
│   │               │   ├── Id
│   │               │   ├── Lparen
│   │               │   ├── ARG
│   │               │   │   ├── Vtype
│   │               │   │   ├── Id
│   │               │   │   └── MOREARGS
│   │               │   │       ├── Comma
│   │               │   │       ├── Vtype
│   │               │   │       ├── Id
│   │               │   │       └── MOREARGS
│   │               │   ├── Rparen
│   │               │   ├── Lbrace
│   │               │   ├── BLOCK
│   │               │   │   ├── STMT
│   │               │   │   │   ├── If
│   │               │   │   │   ├── Lparen
│   │               │   │   │   ├── COND
│   │               │   │   │   │   └── Boolstr
│   │               │   │   │   ├── Rparen
│   │               │   │   │   ├── Lbrace
│   │               │   │   │   ├── BLOCK
│   │               │   │   │   ├── Rbrace
│   │               │   │   │   └── ELSE
│   │               │   │   └── BLOCK
│   │               │   ├── RETURN
│   │               │   │   ├── Return
│   │               │   │   ├── RHS
│   │               │   │   │   └── EXPR
│   │               │   │   │       ├── EXPR
│   │               │   │   │       │   └── EXPR_
│   │               │   │   │       │       └── EXPR__
│   │               │   │   │       │           └── Num
│   │               │   │   │       ├── Addsub
│   │               │   │   │       └── EXPR_
│   │               │   │   │           └── EXPR__
│   │               │   │   │               └── Id
│   │               │   │   └── Semi
│   │               │   └── Rbrace
│   │               └── ODECL
│   └── Rbrace
└── CODE

Accepted!
```
### case 3
```
// In testcase/sample_input3.sj
vtype id lparen rparen lbrace
    id assign id addsub id multdiv id addsub id multdiv id semi

    return id semi
rbrace
```
```bash
$ ./syntax_analyzer testcase/sample_input3.sj
[1/4] File name: testcase/sample_input3.sj

[2/4] File contents:
vtype id lparen rparen lbrace
    id assign id addsub id multdiv id addsub id multdiv id semi

    return id semi
rbrace

[3/4] Read tokens:
[Vtype Id Lparen Rparen Lbrace Id Assign Id Addsub Id Multdiv Id Addsub Id Multdiv Id Semi Return Id Semi Rbrace EOL]

[4/4] Parse tree:
CODE
├── FDECL
│   ├── Vtype
│   ├── Id
│   ├── Lparen
│   ├── ARG
│   ├── Rparen
│   ├── Lbrace
│   ├── BLOCK
│   │   ├── STMT
│   │   │   ├── ASSIGN
│   │   │   │   ├── Id
│   │   │   │   ├── Assign
│   │   │   │   └── RHS
│   │   │   │       └── EXPR
│   │   │   │           ├── EXPR
│   │   │   │           │   ├── EXPR
│   │   │   │           │   │   └── EXPR_
│   │   │   │           │   │       └── EXPR__
│   │   │   │           │   │           └── Id
│   │   │   │           │   ├── Addsub
│   │   │   │           │   └── EXPR_
│   │   │   │           │       ├── EXPR_
│   │   │   │           │       │   └── EXPR__
│   │   │   │           │       │       └── Id
│   │   │   │           │       ├── Multdiv
│   │   │   │           │       └── EXPR__
│   │   │   │           │           └── Id
│   │   │   │           ├── Addsub
│   │   │   │           └── EXPR_
│   │   │   │               ├── EXPR_
│   │   │   │               │   └── EXPR__
│   │   │   │               │       └── Id
│   │   │   │               ├── Multdiv
│   │   │   │               └── EXPR__
│   │   │   │                   └── Id
│   │   │   └── Semi
│   │   └── BLOCK
│   ├── RETURN
│   │   ├── Return
│   │   ├── RHS
│   │   │   └── EXPR
│   │   │       └── EXPR_
│   │   │           └── EXPR__
│   │   │               └── Id
│   │   └── Semi
│   └── Rbrace
└── CODE

Accepted!
```
### case 4
```
// In testcase/sample_input4.sj
vtype id lparen rparen lbrace
    id assign id multdiv id multdiv id addsub id multdiv id semi

    return id semi
rbrace
```
```bash
$ ./syntax_analyzer testcase/sample_input4.sj
[1/4] File name: testcase/sample_input4.sj

[2/4] File contents:
vtype id lparen rparen lbrace
    id assign id multdiv id multdiv id addsub id multdiv id semi

    return id semi
rbrace

[3/4] Read tokens:
[Vtype Id Lparen Rparen Lbrace Id Assign Id Multdiv Id Multdiv Id Addsub Id Multdiv Id Semi Return Id Semi Rbrace EOL]

[4/4] Parse tree:
CODE
├── FDECL
│   ├── Vtype
│   ├── Id
│   ├── Lparen
│   ├── ARG
│   ├── Rparen
│   ├── Lbrace
│   ├── BLOCK
│   │   ├── STMT
│   │   │   ├── ASSIGN
│   │   │   │   ├── Id
│   │   │   │   ├── Assign
│   │   │   │   └── RHS
│   │   │   │       └── EXPR
│   │   │   │           ├── EXPR
│   │   │   │           │   └── EXPR_
│   │   │   │           │       ├── EXPR_
│   │   │   │           │       │   ├── EXPR_
│   │   │   │           │       │   │   └── EXPR__
│   │   │   │           │       │   │       └── Id
│   │   │   │           │       │   ├── Multdiv
│   │   │   │           │       │   └── EXPR__
│   │   │   │           │       │       └── Id
│   │   │   │           │       ├── Multdiv
│   │   │   │           │       └── EXPR__
│   │   │   │           │           └── Id
│   │   │   │           ├── Addsub
│   │   │   │           └── EXPR_
│   │   │   │               ├── EXPR_
│   │   │   │               │   └── EXPR__
│   │   │   │               │       └── Id
│   │   │   │               ├── Multdiv
│   │   │   │               └── EXPR__
│   │   │   │                   └── Id
│   │   │   └── Semi
│   │   └── BLOCK
│   ├── RETURN
│   │   ├── Return
│   │   ├── RHS
│   │   │   └── EXPR
│   │   │       └── EXPR_
│   │   │           └── EXPR__
│   │   │               └── Id
│   │   └── Semi
│   └── Rbrace
└── CODE

Accepted!
```
### fail case 0
```
// In testcase/sample_fail_input0.sj
CODE
```
```bash
$ ./syntax_analyzer testcase/sample_fail_input0.sj
[1/4] File name: testcase/sample_fail_input0.sj

[2/4] File contents:
CODE

[3/4] error: unknown token: CODE
```
### fail case 1
```
// In testcase/sample_fail_input1.sj
vtype id semi vtype id lparen rparen lbrace if lparen boolstr comp boolstr rparen lbrace rbrace
```
```bash
$ ./syntax_analyzer testcase/sample_fail_input1.sj
[1/4] File name: testcase/sample_fail_input1.sj

[2/4] File contents:
vtype id semi vtype id lparen rparen lbrace if lparen boolstr comp boolstr rparen lbrace rbrace

[3/4] Read tokens:
[Vtype Id Semi Vtype Id Lparen Rparen Lbrace If Lparen Boolstr Comp Boolstr Rparen Lbrace Rbrace EOL]

[4/4] error: parsing error
	expected: [ELSE, Vtype, Else, If, Id, While, Return, Rbrace]
	but found: EOL
```
### fail case 2
```
// In testcase/sample_fail_input2.sj
vtype id semi vtype id lparen rparen lbrace if lparen boolstr comp boolstr rparen lbrace return id semi rbrace
```
```bash
$ ./syntax_analyzer testcase/sample_fail_input2.sj
[1/4] File name: testcase/sample_fail_input2.sj

[2/4] File contents:
vtype id semi vtype id lparen rparen lbrace if lparen boolstr comp boolstr rparen lbrace return id semi rbrace

[3/4] Read tokens:
[Vtype Id Semi Vtype Id Lparen Rparen Lbrace If Lparen Boolstr Comp Boolstr Rparen Lbrace Return Id Semi Rbrace EOL]

[4/4] error: parsing error
	expected: [Rbrace]
	but found: Return
```
### fail case 3
```
// In testcase/sample_fail_input3.sj
vtype id lparen rpalren lbrace
    return id semi
rbrace
```
```bash
$ ./syntax_analyzer testcase/sample_fail_input3.sj
[1/4] File name: testcase/sample_fail_input3.sj

[2/4] File contents:
vtype id lparen rpalren lbrace
    return id semi
rbrace

[3/4] error: unknown token: rpalren
```
### fail case 4
no input
```bash
$ ./syntax_analyzer
[1/4] error: no input file
```
### fail case 5
fail to read file

```bash
$ ./syntax_analyzer awkjnviwunalwdj
[1/4] File name: awkjnviwunalwdj

[2/4] error: something went wrong during reading file
```
