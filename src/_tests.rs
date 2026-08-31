mod ast;
mod lexer;
mod parser;
mod token;

fn main() {
// Тест 1: Простое условие IF с равенством
let t = lexer::tokenize("if 5 = 5 then print 42");
println!("{:?}", t);
let a = parser::parse(t);
println!("{:?}\n", a);

// Тест 2: IF с неравенством (если поддерживаете)
let t = lexer::tokenize("if 10 <> 5 then print hello");
println!("{:?}", t);
let a = parser::parse(t);
println!("{:?}\n", a);

// Тест 3: IF с ложным условием
let t = lexer::tokenize("if 3 = 7 then print no");
println!("{:?}", t);
let a = parser::parse(t);
println!("{:?}\n", a);

// Тест 4: Присваивание переменной
let t = lexer::tokenize("let x = 100");
println!("{:?}", t);
let a = parser::parse(t);
println!("{:?}\n", a);

// Тест 5: PRINT с числом
let t = lexer::tokenize("print 123");
println!("{:?}", t);
let a = parser::parse(t);
println!("{:?}\n", a);

// Тест 6: PRINT с переменной
let t = lexer::tokenize("print x");
println!("{:?}", t);
let a = parser::parse(t);
println!("{:?}\n", a);

// Тест 7: Арифметика в LET
let t = lexer::tokenize("let y = 5 + 3 * 2");
println!("{:?}", t);
let a = parser::parse(t);
println!("{:?}\n", a);

// Тест 8: GOTO
let t = lexer::tokenize("goto 100");
println!("{:?}", t);
let a = parser::parse(t);
println!("{:?}\n", a);

// Тест 9: IF с GOTO
let t = lexer::tokenize("if a = b then goto 200");
println!("{:?}", t);
let a = parser::parse(t);
println!("{:?}\n", a);
// Тест 10: PRINT с несколькими значениями через запятую
let t = lexer::tokenize("print 1, 2, 3");
println!("{:?}", t);
let a = parser::parse(t);
println!("{:?}\n", a);

// Тест 11: IF с ELSE (если поддерживаете)
let t = lexer::tokenize("if x = 10 then print yes else print no");
println!("{:?}", t);
let a = parser::parse(t);
println!("{:?}\n", a);

// Тест 12: Вложенные арифметические выражения со скобками
let t = lexer::tokenize("let z = (5 + 3) * 2");
println!("{:?}", t);
let a = parser::parse(t);
println!("{:?}\n", a);

// Тест 13: PRINT строки (если поддерживаете строковые литералы)
let t = lexer::tokenize("print \"Hello World\"");
println!("{:?}", t);
let a = parser::parse(t);
println!("{:?}\n", a);

// Тест 14: INPUT (если есть)
let t = lexer::tokenize("input x");
println!("{:?}", t);
let a = parser::parse(t);
println!("{:?}\n", a);

// Тест 15: REM (комментарий)
let t = lexer::tokenize("rem this is a comment");
println!("{:?}", t);
let a = parser::parse(t);
println!("{:?}\n", a);

// Тест 16: END
let t = lexer::tokenize("end");
println!("{:?}", t);
let a = parser::parse(t);
println!("{:?}\n", a);

// Тест 17: LIST
// let t = lexer::tokenize("list");
// println!("{:?}", t);
// let a = parser::parse(t);
// println!("{:?}\n", a);

// // Тест 18: RUN
// let t = lexer::tokenize("run");
// println!("{:?}", t);
// let a = parser::parse(t);
// println!("{:?}\n", a);

// // Тест 19: CLEAR
// let t = lexer::tokenize("clear");
// println!("{:?}", t);
// let a = parser::parse(t);
// println!("{:?}\n", a);

// Тест 20: IF с арифметикой в условии
let t = lexer::tokenize("if 5 + 3 = 8 then print ok");
println!("{:?}", t);
let a = parser::parse(t);
println!("{:?}\n", a);
}
