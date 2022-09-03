use std::fs;

fn define_visitor(code: &mut String, base_name: &str, fields: &[&str]) {
    *code = format!("{}pub trait {}Visitor {{\n", code, base_name);
    for field in fields {
        let type_name = field.split('!').next().unwrap();
        *code = format!("{}    fn visit_{}(&mut self, {}: &{}{}) -> f64;\n", code, type_name.to_lowercase(), type_name.to_lowercase(), type_name, base_name);
    }
    *code = format!("{}}}\n\n", code);
}

fn define_type(code: &mut String, base_name: &str, class_name: &str, field_name: &str) {
    *code = format!("{}pub struct {}{} {{\n", code, class_name, base_name);
    let some = field_name.split(',');
    for field in some {
        *code = format!("{}    {},\n", code, field.trim());
    }
    *code = format!("{}}}\n\n", code);

    *code = format!("{}impl {}{} {{\n", code, class_name, base_name);
    *code = format!("{}    pub fn new({}) -> Self {{\n", code, field_name);
    *code = format!("{}        Self {{ ", code);
    let some = field_name.split(',');
    for field in some {
        let mut ok = field.split(':');
        let variable = ok.next().unwrap().trim();
        *code = format!("{} {}, ", code, variable);
    }
    *code = format!("{}}}\n", code);
    *code = format!("{}    }}\n", code);

    let some = field_name.split(',');
    for field in some {
        let mut ok = field.split(':');
        let variable = ok.next().unwrap().trim();
        let type_name = ok.next().unwrap().trim();
        *code = format!("{}    pub fn get_{}(&self) -> &{} {{ &self.{} }}\n", code, variable, type_name, variable);
    }

    *code = format!("{}}}\n\n", code);

    *code = format!("{}impl {} for {}{} {{\n", code, base_name, class_name, base_name);
    *code = format!("{}    fn accept(&self, visitor: &mut dyn {}Visitor) -> f64 {{\n", code, base_name);
    *code = format!("{}        return visitor.visit_{}(self);\n", code, class_name.to_lowercase());
    *code = format!("{}    }}\n", code);
    *code = format!("{}}}\n\n", code);
}

fn define_ast(code: &mut String, base_name: &str, fields: &[&str]) {
    define_visitor(code, &base_name, &fields);

    *code = format!("{}pub trait {} {{\n", code, base_name);
    *code = format!("{}    fn accept(&self, visitor: &mut dyn {}Visitor) -> f64;\n", code, base_name);
    *code = format!("{}}}\n\n", code);

    for field in fields {
        let mut some = field.split('!');
        let class_name = some.next().unwrap().trim();
        let field_name = some.next().unwrap().trim();
        define_type(code, base_name, class_name, field_name);
    }
}

fn main() {
    let mut code = String::new();

    code.push_str("use crate::token::Token;\n\n");

    define_ast(&mut code, "Expr",
        &[
            "Literal! token: Token",
            "Unary! op: Token, right: Box<dyn Expr>",
            "Binary! left: Box<dyn Expr>, op: Token, right: Box<dyn Expr>",
        ]
    );

    define_ast(&mut code, "Stmt",
        &[
            "Expression! expression: Box<dyn Expr>",
            "Print! expression: Box<dyn Expr>"
        ]
    );

    fs::write("../src/expr.rs", code).expect("Unable to write file");
}
