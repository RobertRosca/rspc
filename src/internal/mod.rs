//! Internal types which power rspc. The module provides no guarantee of compatibility between updates, so you should be careful relying on types from it.
//!
//! WARNING: Anything in this module or submodules does not follow semantic versioning as it's considered an implementation detail.
//!

pub(crate) mod build;

pub mod into_response;
pub mod layer;
pub mod middleware;
pub mod pinned_option;
pub mod procedure;
pub mod procedure_store;
pub mod resolvers;

// TODO: Fix this
// #[cfg(test)]
// mod tests {
//     use std::{fs::File, io::Write, path::PathBuf};

//     use specta::{ts::export_named_datatype, DefOpts, Type, TypeMap};

//     use rspc_core::exec;

//     macro_rules! collect_datatypes {
//         ($( $i:path ),* $(,)? ) => {{
//             use specta::DataType;

//             let mut tys = TypeMap::default();

//             $({
//                 let def = <$i as Type>::definition(DefOpts {
//                     parent_inline: true,
//                     type_map: &mut tys,
//                 });

//                 if let Ok(def) = def {
//                     if let DataType::Named(n) = def {
//                         if let Some(sid) = n.ext().as_ref().map(|e| *e.sid()) {
//                             tys.insert(sid, Some(n));
//                         }
//                     }
//                 }
//             })*
//             tys
//         }};
//     }

//     // rspc has internal types that are shared between the frontend and backend. We use Specta directly to share these to avoid a whole class of bugs within the library itself.
//     #[test]
//     fn export_internal_types() {
//         let mut file = File::create(
//             PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("./packages/client/src/bindings.ts"),
//         )
//         .unwrap();

//         file.write_all(
//             b"// DO NOT MODIFY. This file was generated by Specta and is used to keep rspc internally type safe.\n// Checkout the unit test 'export_internal_types' to see where this files comes from!",
//         )
//         .unwrap();

//         let tys = collect_datatypes![
//             rspc_core::internal::ProceduresDef,
//             // crate::Procedures, // TODO
//             exec::Request,
//             exec::Response,
//         ];

//         for (_, ty) in tys
//             .iter()
//             .filter_map(|(sid, v)| v.as_ref().map(|v| (sid, v)))
//         {
//             file.write_all(b"\n\n").unwrap();
//             file.write_all(
//                 export_named_datatype(&Default::default(), &ty, &tys)
//                     .unwrap()
//                     .as_bytes(),
//             )
//             .unwrap();
//         }
//     }
// }
