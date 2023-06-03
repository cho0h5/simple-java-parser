#! /usr/bin/python3

# https://jsmachines.sourceforge.net/machines/slr.html의 parsing table의
# html을 parsing하여 rust 코드를 출력하는 프로그램입니다.

from bs4 import BeautifulSoup as bs

label = ("dummy","Vtype","Id","Semi","Assign","Literal","Character","Boolstr","Addsub","Multdiv","Lparen","Rparen","Num","Lbrace","Rbrace","Comma","If","While","Comp","Else","Return","Class","EOL","CODE_","CODE","VDECL","ASSIGN","RHS","EXPR","EXPR_","EXPR__","FDECL","ARG","MOREARGS","BLOCK","STMT","COND","ELSE","RETURN","CDECL","ODECL")

i = 0

def parsing(line):
    print("    // for state", i)
    print("    let mut hashmap = HashMap::new();")
    j = 0
    for l in line.find_all('td'):
        content = l.get_text()
        if len(content.strip()) != 0 and j != 0:
            # hashmap.insert(Vtype, Shift(5));
            print("    hashmap.insert({}, ".format(label[j]), end='')
            if content[0] == 'r':
                print("Reduce({}));".format(content[1:]))
            elif content[0] == 's':
                print("Shift({}));".format(content[1:]))
            else:
                print("Goto({}));".format(content))


        j += 1
    print("    table.push(hashmap);")


f = open("table.html", 'r')

while True:
    line = f.readline()
    if not line: break

    print()
    soup = bs(line, 'html.parser').select_one('tr')
    parsing(soup)
    i += 1

f.close()
