mod variables_constants;
mod primitive;
mod compound_data_types;
mod functions_code_blocks;
mod conditionals;
mod control_flow;
mod comments;
mod compiler_directives;
mod error_messages;
mod operators;
mod associativity_overloading;
mod operator_precedence;
mod mutability_function;

fn main() {
    variables_constants::exec();
    primitive::exec();
    compound_data_types::exec();
    functions_code_blocks::exec();
    conditionals::exec();
    control_flow::exec();
    comments::exec();
    compiler_directives::exec();
    error_messages::exec();
    operators::exec();
    associativity_overloading::exec();
    operator_precedence::exec();
    mutability_function::exec();
}
