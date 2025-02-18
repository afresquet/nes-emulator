use core::panic;

use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Meta};

#[proc_macro_derive(Instruction, attributes(opcode))]
pub fn instruction(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let Data::Enum(data) = input.data else {
        panic!("only enums supported");
    };

    let fields = data
        .variants
        .iter()
        .map(|variant| {
            let name = &variant.ident;
            let Fields::Unnamed(fields) = &variant.fields else {
                panic!("only unnamed fields supported");
            };
            let instruction = &fields.unnamed.first().unwrap().ty;
            let Meta::List(meta) = &variant.attrs.first().unwrap().meta else {
                panic!("only list meta supported");
            };
            let attr = &meta.tokens;

            (name, instruction, attr)
        })
        .collect::<Vec<_>>();

    let fetch = fields.iter().map(|(_name, instruction, attr)| {
        quote! {
             #attr => #instruction::fetch(cpu)
        }
    });

    let execute = fields.iter().map(|(name, _instruction, _attr)| {
        quote! {
             Self::#name(instruction) => instruction.execute(cpu)
        }
    });

    let cycles = fields.iter().map(|(name, _instruction, _attr)| {
        quote! {
             Self::#name(instruction) => instruction.cycles()
        }
    });

    let opcode_name = fields.iter().map(|(name, _instruction, attr)| {
        let string = name.to_string();
        quote! {
             #attr => #string
        }
    });

    quote! {
        impl OpCode for #name {
            fn fetch(cpu: &mut CPU) -> Instruction {
                cpu.current_instruction_register = cpu.mem_read(cpu.program_counter);

                match cpu.current_instruction_register {
                    #(#fetch,)*
                }
            }

            fn execute(self, cpu: &mut CPU) {
                match self {
                    #(#execute,)*
                }
            }

            fn cycles(&self) -> u8 {
                match self {
                    #(#cycles,)*
                }
            }
        }

        impl #name {
            pub fn name(code: u8) -> &'static str {
                match code {
                    #(#opcode_name,)*
                }
            }
        }
    }
    .into()
}
