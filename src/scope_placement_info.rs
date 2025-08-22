use compiler_11::ast::FunctionDef;

pub struct ScopePlacementInfo<'scope_placement_info> {
    pub function_def: &'scope_placement_info FunctionDef<'scope_placement_info>,
    pub index: usize,
}