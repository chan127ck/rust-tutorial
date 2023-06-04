// ignore unused variable error
#![allow(unused)]

//include local lib
mod variabletypes;
mod userinput;
mod numbertypes;
mod if_statement;
mod match_statement;
mod array_use;
mod tuple_use;
mod string_use;
mod cast_use;
mod enum_use;
mod vector_use;
mod function_use;
mod generic_use;
mod stack_heap_use;
mod hash_map_use;
mod struct_use;
mod restaurant;
use crate::restaurant::order_food;
mod file_io_use;
mod iterator_use;
mod closure_use;
mod smart_pointer_use;

fn main() {
    //userinput::io_func();
    //variabletypes::variable_type();
    //numbertypes::num_type();
    //if_statement::if_example();
    //match_statement::match_example();
    //array_use::array_example();
    //tuple_use::tuple_example();
    //string_use::string_example();
    //cast_use::cast_example();
    //enum_use::enum_example();
    //vector_use::vector_example();
    //function_use::function_example();
    //generic_use::generic_example();
    //stack_heap_use::stack_heap_example();
    //hash_map_use::hasp_map_example();
    //struct_use::struct_example();
    //module_use::module_example();
    // ----code in restaurant folder
    //order_food();
    //file_io_use::file_io_example();
    //iterator_use::iterator_example();
    //closure_use::closure_example();
    smart_pointer_use::smart_pointer_example();
}

