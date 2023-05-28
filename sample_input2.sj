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
