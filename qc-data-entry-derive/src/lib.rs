use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Nothing, Parser},
    parse_macro_input, DeriveInput, Error,
};

// macro_rules! unwrap_or_return {
//     ( $e:expr ) => {
//         match $e {
//             Ok(x) => x,
//             Err(_) => return,
//         }
//     };
// }
// pub use unwrap_or_return;

#[proc_macro_attribute]
pub fn derive_mass(args: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let _ = parse_macro_input!(args as Nothing);

    match derive_mass_impl(ast) {
        Ok(val) => val,
        Err(err) => err.into_compile_error(),
    }
    .into()
}

fn derive_mass_impl(mut ast: DeriveInput) -> Result<proc_macro2::TokenStream, Error> {
    let data_ident = &ast.ident;

    match &mut ast.data {
        syn::Data::Struct(struct_data) => {
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    let layout_field = fields
                        .named
                        .iter()
                        .find(|field| {
                            field
                                .attrs
                                .iter()
                                .find(|attr| attr.path().is_ident("nwg_layout"))
                                .is_some()
                        })
                        .ok_or(Error::new(
                            data_ident.span(),
                            format!(
                                "`{}` needs a #nwg_layout field to implement `derive_mass`",
                                data_ident
                            ),
                        ))?
                        .ident
                        .clone();

                    let _ = std::mem::replace(
                        fields
                            .named
                            .iter_mut()
                            .find(|field| field.ident.as_ref().unwrap().to_string() == "mass")
                            .ok_or(Error::new(
                                data_ident.span(),
                                format!(
                                    "`{}` needs a `mass` field to implement `derive_mass`",
                                    data_ident
                                ),
                            ))?,
                        syn::Field::parse_named
                            .parse2(quote! {
                                #[nwg_control(label: "Mass", places: 2, precision: 2)]
                                #[nwg_layout_item(layout: #layout_field)]
                                #[nwg_events (OnTextInput:[#data_ident::have_mass])]
                                mass: FixedNumEdit

                            })
                            .unwrap(),
                    );

                    fields.named.push(
                        syn::Field::parse_named
                            .parse2(quote! {
                                #[nwg_control(label: "Specific Gravity", units:"g/mL")]
                                #[nwg_layout_item(layout: #layout_field)]
                                #[nwg_events( OnTextInput:[#data_ident::have_sg])]
                                sg: NumberUnitsEdit
                            })
                            .unwrap(),
                    );
                    fields.named.push(
                        syn::Field::parse_named
                            .parse2(quote! {
                                #[nwg_control(label: "Density", units:"lb/gal")]
                                #[nwg_layout_item(layout: #layout_field)]
                                #[nwg_events( OnTextInput:[#data_ident::have_density])]
                                density: NumberUnitsEdit
                            })
                            .unwrap(),
                    );
                }
                _ => (),
            }
            let impl_block = quote! {
            impl #data_ident {


                fn have_mass(&self) {
                    use qc_data_entry::{
                        convert::{density_from_sg,  sg_from_mass},
                        formats::{format_sg_mass, format_density, format_mass}
                    };
                    let mass = match self.mass.parse() {Ok(mass) => mass,Err(_) => return,};
                    let sg = sg_from_mass(mass);
                    let density = density_from_sg(sg);
                    self.sg.set_text(&format_sg_mass(sg));
                    self.density.set_text(&format_density(density));

                }

                fn have_sg(&self) {
                    use qc_data_entry::{
                        convert::{density_from_sg, mass_from_sg, sg_from_density, sg_from_mass},
                        formats::{format_sg_mass, format_density, format_mass}

                    };
                    let sg = match self.sg.text().trim().parse() {Ok(sg) => sg,Err(_) => return,};
                    let mass = mass_from_sg(sg);
                    let density = density_from_sg(sg);
                    self.mass.set_text(&format_mass(mass));
                    self.density.set_text(&format_density(density));
                }

                fn have_density(&self) {
                    use qc_data_entry::{
                        convert::{density_from_sg, mass_from_sg, sg_from_density, sg_from_mass},
                        formats::{format_sg_mass, format_density, format_mass}

                    };
                    let density = match self.density.text().trim().parse() {Ok(density) => density,Err(_) => return,};
                    let sg = sg_from_density(density);
                    let mass = mass_from_sg(sg);
                    self.sg.set_text(&format_sg_mass(sg));
                    self.mass.set_text(&format_mass(mass));
                }
            }
            };

            Ok(quote! {
                #ast

                #impl_block
            })
        }
        _ => Err(Error::new(
            data_ident.span(),
            "`derive_mass` has to be used with structs",
        )),
    }
}
