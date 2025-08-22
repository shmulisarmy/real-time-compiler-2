use compiler_11::{ast::{ValidInFunctionBody}, data_type::DataType};

use crate::scope_placement_info::ScopePlacementInfo;






pub fn find_var_type_from_local_scope<'compilation_unit>(var_name: &str, scope_placement_info: &'compilation_unit ScopePlacementInfo<'compilation_unit>) -> Option<DataType> {
    let mut i = scope_placement_info.index;
    let function = scope_placement_info.function_def;
    while true {
        match &function.body[i] {
            ValidInFunctionBody::Variable(variable) => {
                if variable.name == var_name {
                    return Some(variable.type_.clone());
                }
            }
            _ => {}
        }
        if i == 0 {
            break;
        }
        i -= 1;
    } 
    for param in scope_placement_info.function_def.args.iter() {
        if param.name == var_name {
            return Some(param.type_.clone());
        }
    }
    None
}