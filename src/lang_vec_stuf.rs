use core::borrow;

use tree_sitter::{Node, Parser};

pub trait Language {
    fn get_name(&self) -> &'static str;
    fn analyze_to_vec(&self, source_code: String) -> Vec<LocalVecBlock>;
}

#[derive(Debug, PartialEq)]
enum CodeBlock {
    Else(i32, i32),
    Match(i32, i32, usize),
    If(i32, i32, i32),
    For(i32, i32),
    While(i32, i32),
    Loop(i32, i32),
    Func,
    Return,
    Continue,
}

#[derive(Debug, PartialEq)]
pub enum BlockType {
    Start,
    End,
    Action,
    Print,
    Condition,
    Cycle,
    Else,
    EndMatchArm,
}

pub struct LocalVecBlock {
    pub r#type: BlockType,
    pub text: String,
    pub x: i32,
    pub y: i32,
}

#[derive(Default)]
pub struct Rust;

impl Language for Rust {
    fn get_name(&self) -> &'static str {
        "Rust"
    }

    fn analyze_to_vec(&self, source_code: String) -> Vec<LocalVecBlock> {
        fn traverse_ast(
            node: Node,
            source: &[u8],
            blocks: &mut Vec<LocalVecBlock>,
            y_offset: &mut i32,
            x_offset: &mut i32,
            block_vec: &mut Vec<CodeBlock>,
            skip_until_brace: &mut bool,
            is_return: &mut bool,
            if_else_stack: &mut Vec<(i32, i32)>,
            y_if_max: &mut i32,
        ) {
            //println!("y_offset == {}", y_offset);
            let text = node.utf8_text(source).unwrap_or("").to_string();

            if *skip_until_brace {
                if text.as_str() == "{" {
                    *skip_until_brace = false;
                    let mut cursor = node.walk();
                    for child in node.children(&mut cursor) {
                        traverse_ast(
                            child,
                            source,
                            blocks,
                            y_offset,
                            x_offset,
                            block_vec,
                            skip_until_brace,
                            is_return,
                            if_else_stack,
                            y_if_max,
                        );
                    }
                    return;
                } else {
                    let mut cursor = node.walk();
                    for child in node.children(&mut cursor) {
                        traverse_ast(
                            child,
                            source,
                            blocks,
                            y_offset,
                            x_offset,
                            block_vec,
                            skip_until_brace,
                            is_return,
                            if_else_stack,
                            y_if_max,
                        );
                    }
                    return;
                }
            }
            println!("Processing node: kind={}, text={}", node.kind(), text);

            //*y_offset += (text.lines().count() * 10) as i32;
            let mut local_block = LocalVecBlock {
                r#type: BlockType::Action,
                text: text.clone(),
                x: *x_offset,
                y: *y_offset,
            };

            match node.kind() {
                "let_declaration" | "parameters" | "->" | "primitive_type" | ";"
                | "block_comment" | "line_comment" | "match" | "=>" | "use_declaration"
                | "generic_type" => {
                    return;
                }
                "loop_expression"
                | "function_item"
                | "block"
                | "expression_statement"
                | "source_file"
                | "match_arm"
                | "fn" => {
                    let mut cursor = node.walk();
                    for child in node.children(&mut cursor) {
                        traverse_ast(
                            child,
                            source,
                            blocks,
                            y_offset,
                            x_offset,
                            block_vec,
                            skip_until_brace,
                            is_return,
                            if_else_stack,
                            y_if_max,
                        );
                    }
                    return;
                }
                "{" => {
                    /*if let Some(CodeBlock::Match(_, _)) = block_vec.last() {
                        println!("push continue");
                        block_vec.push(CodeBlock::Continue);
                    }*/
                    return;
                }
                "call_expression" => {
                    blocks.push(local_block);
                    *y_offset += 100;
                    return;
                }
                "identifier" => {
                    identifier(y_offset, text, block_vec, blocks, local_block);
                    return;
                }
                "if_expression" => {
                    if_expression(
                        &mut local_block,
                        skip_until_brace,
                        y_if_max,
                        y_offset,
                        x_offset,
                        if_else_stack,
                        block_vec,
                        text,
                    );
                }
                "else_clause" => {
                    else_clause(
                        text,
                        &mut local_block,
                        skip_until_brace,
                        y_if_max,
                        y_offset,
                        x_offset,
                        if_else_stack,
                        block_vec,
                    );
                    //*y_offset -= 100;
                }
                "else" => {
                    else_handler(&mut local_block, y_offset, x_offset, block_vec);
                }
                "match_expression" => {
                    match_expression(&mut local_block, x_offset, y_offset, text, block_vec);
                    //return;
                }
                "match_pattern" => {
                    //возможно насрал, посмотрим по поведению
                    if let Some(CodeBlock::Match(_, to_y, count)) = block_vec.last_mut() {
                        //*count -= 1;
                        *x_offset += 300;
                        *y_offset = *to_y;
                    } else {
                        for i in block_vec.iter_mut().rev() {
                            if let CodeBlock::Match(_, to_y, count) = i {
                                //*count -= 1;
                                *x_offset += 300;
                                *y_offset = *to_y;
                                break;
                            }
                        }
                    }
                    local_block.x = *x_offset;
                    local_block.y = *y_offset;
                    blocks.push(local_block);
                    *y_offset += 100;
                    return;
                }
                "return_expression" => {
                    return_expression(y_offset, y_if_max, local_block, blocks, is_return);
                    return;
                }
                "macro_invocation" => {
                    macro_invocation(y_offset, y_if_max, text, local_block, blocks);
                    return;
                }
                "binary_expression"
                | "compound_assignment_expr"
                | "break_expression"
                | "assignment_expression"
                | "continue_expression" => {
                    blocks.push(local_block);
                    *y_offset += 100;
                    return;
                }
                "loop" => {
                    loop_handler(y_offset, x_offset, y_if_max, block_vec, &mut local_block);
                }
                "for_expression" => {
                    for_expression(
                        y_offset,
                        x_offset,
                        y_if_max,
                        text,
                        block_vec,
                        skip_until_brace,
                        &mut local_block,
                    );
                }
                "while_expression" => {
                    while_expression(
                        y_offset,
                        x_offset,
                        y_if_max,
                        text,
                        block_vec,
                        skip_until_brace,
                        &mut local_block,
                    );
                }
                "," => {
                    local_block.r#type = BlockType::EndMatchArm;
                    println!("push end match arm");
                    if *y_if_max < *y_offset {
                        *y_if_max = *y_if_max;
                    }
                }
                "}" => {
                    closing_brecket_handler(
                        if_else_stack,
                        y_if_max,
                        y_offset,
                        x_offset,
                        is_return,
                        block_vec,
                        &mut local_block,
                    );
                }
                _ => {
                    if text.contains("else") {
                        local_block.text = "blablabla".to_string();
                    }
                    //panic!("unknown type \"{}\"", node.kind())
                }
            }

            blocks.push(local_block);
            *y_offset += 100;

            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                traverse_ast(
                    child,
                    source,
                    blocks,
                    y_offset,
                    x_offset,
                    block_vec,
                    skip_until_brace,
                    is_return,
                    if_else_stack,
                    y_if_max,
                );
            }
        }

        let mut block_vec: Vec<CodeBlock> = Vec::new();
        let mut if_else_stack: Vec<(i32, i32)> = Vec::new();
        let mut y_if_max: i32 = 0;

        //                             x    y
        //let mut coord_check: HashSet<(i32, i32)> = HashSet::new();
        let mut is_return = false;

        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_rust::language())
            .expect("Error loading Rust grammar");
        let tree = parser.parse(source_code.clone(), None).unwrap();

        let root_node = tree.root_node();
        let mut blocks = Vec::<LocalVecBlock>::new();
        let mut y_offset = 0;
        let mut x_offset = 0;
        let mut skip_until_brace = false;

        traverse_ast(
            root_node,
            source_code.as_bytes(),
            &mut blocks,
            &mut y_offset,
            &mut x_offset,
            &mut block_vec,
            &mut skip_until_brace,
            &mut is_return,
            &mut if_else_stack,
            &mut y_if_max,
        );
        /*if !block_vec.is_empty() {
            println!("!stack contents!\n{:#?}", block_vec);
            panic!("WRONE CODE")
        }*/
        blocks
    }
}

#[derive(Default)]
pub struct C;

impl Language for C {
    fn get_name(&self) -> &'static str {
        "C"
    }

    fn analyze_to_vec(&self, _source_code: String) -> Vec<LocalVecBlock> {
        todo!()
    }
}

#[derive(Default)]
pub struct CPlusPlus;

impl Language for CPlusPlus {
    fn get_name(&self) -> &'static str {
        "CPlusPlus"
    }

    fn analyze_to_vec(&self, _source_code: String) -> Vec<LocalVecBlock> {
        todo!();
    }
}

#[derive(Default)]
pub struct Java;

impl Language for Java {
    fn get_name(&self) -> &'static str {
        "Java"
    }

    fn analyze_to_vec(&self, _source_code: String) -> Vec<LocalVecBlock> {
        todo!();
    }
}

//handlers
/*
node: Node,
source: &[u8],
blocks: &mut Vec<LocalVecBlock>,
y_offset: &mut i32,
x_offset: &mut i32,
block_vec: &mut Vec<CodeBlock>,
skip_until_brace: &mut bool,
is_return: &mut bool,
if_else_stack: &mut Vec<(i32, i32)>,
y_if_max: &mut i32,
mut local_block: LocalVecBlock,
*/

fn else_handler(
    local_block: &mut LocalVecBlock,
    y_offset: &mut i32,
    x_offset: &mut i32,
    block_vec: &mut Vec<CodeBlock>,
) {
    println!("create else info block");
    local_block.x = *x_offset;
    *y_offset -= 100;
    local_block.r#type = BlockType::Else
}

fn else_clause(
    text: String,
    local_block: &mut LocalVecBlock,
    skip_until_brace: &mut bool,
    y_if_max: &mut i32,
    y_offset: &mut i32,
    x_offset: &mut i32,
    if_else_stack: &mut Vec<(i32, i32)>,
    block_vec: &mut Vec<CodeBlock>,
) {
    if text.contains("if") {
        if_expression(
            local_block,
            skip_until_brace,
            y_if_max,
            y_offset,
            x_offset,
            if_else_stack,
            block_vec,
            text,
        );
    } else {
        println!("mr penis");
        let return_to = if_else_stack.pop().unwrap();
        *x_offset = return_to.0;
        *y_offset = return_to.1;
        //пока хз как себя поведет
        if *y_offset > *y_if_max {
            *y_if_max = *y_offset;
        }
        block_vec.push(CodeBlock::Else(*x_offset, *y_offset));
        local_block.x = *x_offset;
        return;
    }
    //continue;
}

fn if_expression(
    local_block: &mut LocalVecBlock,
    skip_until_brace: &mut bool,
    y_if_max: &mut i32,
    y_offset: &mut i32,
    x_offset: &mut i32,
    if_else_stack: &mut Vec<(i32, i32)>,
    block_vec: &mut Vec<CodeBlock>,
    text: String,
) {
    println!("push if");

    let else_count = text.matches("else").count() as i32;
    let else_if_count = text.matches("else if").count() as i32;
    let local_offset = ((else_count - else_if_count) * 100) as i32;

    println!("local_offset == {local_offset}, else_count = {else_count}, else_if_count == {else_if_count}");
    block_vec.push(CodeBlock::If(*x_offset, *y_offset, local_offset));
    if_else_stack.push((*x_offset - local_offset, *y_offset));
    //пока хз как себя поведет
    if *y_offset > *y_if_max {
        *y_if_max = *y_offset;
    }
    local_block.r#type = BlockType::Condition;
    let mut first_line = text.lines().next().unwrap_or("").to_string();
    first_line.pop();
    local_block.text = first_line;
    //*y_offset += 100;
    local_block.x = *x_offset;
    *x_offset += local_offset;
    *skip_until_brace = true;
    //blocks.push(local_block);
    //return;
}

fn closing_brecket_handler(
    if_else_stack: &mut Vec<(i32, i32)>,
    y_if_max: &mut i32,
    y_offset: &mut i32,
    x_offset: &mut i32,
    is_return: &mut bool,
    block_vec: &mut Vec<CodeBlock>,
    local_block: &mut LocalVecBlock,
) {
    println!("len of block mass = {}", block_vec.len());
    if *is_return && *block_vec.last().unwrap() == CodeBlock::Func {
        *is_return = false;
        block_vec.pop();
        println!("skip");
        return;
    }
    //local_block.y = *y_offset;
    //*y_offset += 10;
    match block_vec.last_mut().unwrap() {
        CodeBlock::Return => {
            println!("add return");
            local_block.text = "Конец".to_string();
            local_block.r#type = BlockType::End;
            block_vec.pop();
        }
        CodeBlock::If(x, y, offset) => {
            println!("Handling If block at {x}:{y}");
            local_block.r#type = BlockType::End;
            //local_block.text = format!("{x}:{y}");
            local_block.text = "condition".to_string();
            block_vec.pop();
            *y_if_max = *y_offset
        }
        CodeBlock::For(x, y) => {
            println!("Handling For block at {x}:{y}");
            local_block.r#type = BlockType::End;
            local_block.text = format!("{x}:{y}");
            block_vec.pop();
        }
        CodeBlock::While(x, y) => {
            println!("Handling While block at {x}:{y}");
            local_block.r#type = BlockType::End;
            local_block.text = format!("{x}:{y}");
            block_vec.pop();
        }
        CodeBlock::Loop(x, y) => {
            println!("Handling Loop block at {x}:{y}");
            local_block.r#type = BlockType::End;
            local_block.text = format!("{x}:{y}");
            block_vec.pop();
        }
        CodeBlock::Func => {
            println!("Handling Func block");
            //local_block.r#type = BlockType::End;
            block_vec.pop();
        }
        CodeBlock::Continue => {
            block_vec.pop();
            return;
        }
        CodeBlock::Match(back_x, back_y, count) => {
            if *count > 0 {
                *count -= 1; // Уменьшаем счетчик на 1
                println!("skip pop");
                local_block.r#type = BlockType::EndMatchArm;
            } else {
                *x_offset = *back_x;
                *y_offset = *y_if_max + 50;
                block_vec.pop();
                local_block.text = String::from("match");
                local_block.r#type = BlockType::End;
            }
        }
        CodeBlock::Else(_, _) => {
            println!("pop else");
            local_block.r#type = BlockType::End;
            local_block.text = "condition".to_string();
            //local_block.r#type = BlockType::End;
            block_vec.pop();
            //if_else_stack.pop();
            if *y_if_max > *y_offset {
                *y_offset = *y_if_max
            }
            /*if if_else_stack.is_empty() {
                *y_if_max = 0;
            }*/
        }
    }
}

fn identifier(
    y_offset: &mut i32,
    text: String,
    block_vec: &mut Vec<CodeBlock>,
    blocks: &mut Vec<LocalVecBlock>,
    mut local_block: LocalVecBlock,
) {
    if text.len() > 2 {
        println!("{}", text.len());
        println!("push fn");
        block_vec.push(CodeBlock::Func);
        if text.contains("main") {
            local_block.text = "Начало".to_string();
        }
        local_block.r#type = BlockType::Start;
        blocks.push(local_block);
        *y_offset += 100;
    }
}

fn while_expression(
    y_offset: &mut i32,
    x_offset: &mut i32,
    y_if_max: &mut i32,
    text: String,
    block_vec: &mut Vec<CodeBlock>,
    skip_until_brace: &mut bool,
    local_block: &mut LocalVecBlock,
) {
    let mut first_line = text.lines().next().unwrap_or("").to_string();
    first_line.pop();
    local_block.text = first_line;
    block_vec.push(CodeBlock::While(*x_offset, *y_offset));
    //пока хз как себя поведет
    if *y_offset > *y_if_max {
        *y_if_max = *y_offset;
    }
    local_block.r#type = BlockType::Cycle;
    *skip_until_brace = true;
}

fn loop_handler(
    y_offset: &mut i32,
    x_offset: &mut i32,
    y_if_max: &mut i32,
    block_vec: &mut Vec<CodeBlock>,
    local_block: &mut LocalVecBlock,
) {
    println!("push loop");
    block_vec.push(CodeBlock::Loop(*x_offset, *y_offset));
    //пока хз как себя поведет
    if *y_offset > *y_if_max {
        *y_if_max = *y_offset;
    }
    local_block.r#type = BlockType::Cycle;
}

fn for_expression(
    y_offset: &mut i32,
    x_offset: &mut i32,
    y_if_max: &mut i32,
    text: String,
    block_vec: &mut Vec<CodeBlock>,
    skip_until_brace: &mut bool,
    local_block: &mut LocalVecBlock,
) {
    println!("push for");
    *skip_until_brace = true;
    let mut first_line = text.lines().next().unwrap_or("").to_string();
    first_line.pop();
    local_block.text = first_line;
    local_block.r#type = BlockType::Cycle;
    //пока хз как себя поведет
    if *y_offset > *y_if_max {
        *y_if_max = *y_offset;
    }
    block_vec.push(CodeBlock::For(*x_offset, *y_offset));
    //return;
}

fn macro_invocation(
    y_offset: &mut i32,
    y_if_max: &mut i32,
    text: String,
    mut local_block: LocalVecBlock,
    blocks: &mut Vec<LocalVecBlock>,
) {
    if text.contains("print") {
        if text.contains("}") {
            local_block.text = String::from("Вывод переменной");
        } else {
            local_block.text = String::from("Вывод строки");
        }
        local_block.r#type = BlockType::Print;
    }
    blocks.push(local_block);
    //пока хз как себя поведет
    if *y_offset > *y_if_max {
        *y_if_max = *y_offset;
    }
    *y_offset += 100;
}

fn return_expression(
    y_offset: &mut i32,
    y_if_max: &mut i32,
    mut local_block: LocalVecBlock,
    blocks: &mut Vec<LocalVecBlock>,
    is_return: &mut bool,
) {
    *is_return = true;
    println!("push return");
    local_block.text = "Конец".to_string();
    local_block.r#type = BlockType::End;
    //пока хз как себя поведет
    if *y_offset > *y_if_max {
        *y_if_max = *y_offset;
    }
    blocks.push(local_block);
    *y_offset += 100;
}

fn match_expression(
    local_block: &mut LocalVecBlock,
    x_offset: &mut i32,
    y_offset: &mut i32,
    text: String,
    block_vec: &mut Vec<CodeBlock>,
) {
    if text.matches("match").count() > 1 {
        panic!("incorrect use of macth")
    }
    println!("push match");
    let arrow_count = text.matches("=>").count();
    let inter_block_count = arrow_count - text.matches(",").count();
    block_vec.push(CodeBlock::Match(
        *x_offset,
        *y_offset + 100,
        inter_block_count,
    ));
    *x_offset -= (arrow_count * 150 as usize) as i32;
    //block_vec.push(CodeBlock::Match(*x_offset, *y_offset));
    let mut first_line = text.lines().next().unwrap_or("").to_string();
    first_line.pop();
    local_block.text = first_line;
    local_block.r#type = BlockType::Condition;
    //blocks.push(local_block);
    *y_offset -= 100;
}
