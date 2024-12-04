use std::{borrow::Cow, collections::BTreeMap, fmt};

use specta::TypeMap;
use specta_util::TypeCollection;

use rspc_core::Procedure;

use crate::State;

/// TODO: Examples exporting types and with `rspc_axum`
pub struct Router2<TCtx = ()> {
    setup: Vec<Box<dyn FnOnce(&mut State) + 'static>>,
    types: TypeCollection,
    procedures: BTreeMap<Vec<Cow<'static, str>>, Procedure<TCtx>>, // TODO: This must be a thing that holds a setup function, type and `Procedure`!
}

impl<TCtx> Default for Router2<TCtx> {
    fn default() -> Self {
        todo!()
    }
}

impl<TCtx> fmt::Debug for Router2<TCtx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // f.debug_tuple("Router").field(&self.procedures).finish()
        todo!();
    }
}

impl<TCtx> Router2<TCtx> {
    pub fn new() -> Self {
        Self::default()
    }

    // TODO: Enforce unique across all methods (query, subscription, etc). Eg. `insert` should yield error if key already exists.
    // pub fn procedure(
    //     mut self,
    //     key: impl Into<Cow<'static, str>>,
    //     procedure: UnbuiltProcedure<TCtx>,
    // ) -> Self {
    //     let name = key.into();
    //     self.procedures.insert(name, procedure);
    //     self
    // }

    pub fn nest(mut self, prefix: impl Into<Cow<'static, str>>, mut other: Self) -> Self {
        self.setup.append(&mut other.setup);

        let prefix = prefix.into();

        self.procedures
            .extend(other.procedures.into_iter().map(|(mut k, v)| {
                k.push(prefix.clone());
                (k, v)
            }));

        self
    }

    // TODO: Document the order this is run in for `build`
    // pub fn setup(mut self, func: impl FnOnce(&mut State) + 'static) -> Self {
    //     self.setup.push(Box::new(func));
    //     self
    // }

    pub fn build(
        self,
    ) -> Result<
        (
            impl Iterator<Item = (String, Procedure<TCtx>)>,
            TypeCollection,
        ),
        (),
    > {
        let mut state = ();
        for setup in self.setup {
            setup(&mut state);
        }

        // let mut type_map = TypeMap::default();
        // self.types.collect(&mut type_map);
        // let procedures: BTreeMap<Cow<'static, str>, _> = self
        //     .procedures
        //     .into_iter()
        //     .map(|(key, procedure)| (key.clone(), procedure.build(key, &mut state, &mut type_map)))
        //     .collect();

        // {
        //     struct Procedure {
        //         kind: String,
        //         input: DataType,
        //         result: DataType,
        //         error: DataType,
        //     }

        //     enum ProcedureOrProcedures {
        //         Procedure(Procedure),
        //         Procedures(HashMap<Cow<'static, str>, ProcedureOrProcedures>),
        //     }

        //     impl Into<specta::DataType> for Procedure {
        //         fn into(self) -> specta::DataType {
        //             specta::DataType::Struct(specta::internal::construct::r#struct(
        //                 "".into(),
        //                 None,
        //                 vec![],
        //                 specta::internal::construct::struct_named(
        //                     vec![
        //                         (
        //                             "kind".into(),
        //                             specta::internal::construct::field(
        //                                 false,
        //                                 false,
        //                                 None,
        //                                 Default::default(),
        //                                 Some(specta::DataType::Literal(
        //                                     specta::datatype::LiteralType::String(self.kind),
        //                                 )),
        //                             ),
        //                         ),
        //                         (
        //                             "input".into(),
        //                             specta::internal::construct::field(
        //                                 false,
        //                                 false,
        //                                 None,
        //                                 Default::default(),
        //                                 Some(self.input),
        //                             ),
        //                         ),
        //                         (
        //                             "result".into(),
        //                             specta::internal::construct::field(
        //                                 false,
        //                                 false,
        //                                 None,
        //                                 Default::default(),
        //                                 Some(self.result),
        //                             ),
        //                         ),
        //                         (
        //                             "error".into(),
        //                             specta::internal::construct::field(
        //                                 false,
        //                                 false,
        //                                 None,
        //                                 Default::default(),
        //                                 Some(self.error),
        //                             ),
        //                         ),
        //                     ],
        //                     None,
        //                 ),
        //             ))
        //         }
        //     }

        //     impl Into<specta::DataType> for ProcedureOrProcedures {
        //         fn into(self) -> specta::DataType {
        //             match self {
        //                 Self::Procedure(procedure) => procedure.into(),
        //                 Self::Procedures(procedures) => {
        //                     specta::DataType::Struct(specta::internal::construct::r#struct(
        //                         "".into(),
        //                         None,
        //                         vec![],
        //                         specta::internal::construct::struct_named(
        //                             procedures
        //                                 .into_iter()
        //                                 .map(|(key, value)| {
        //                                     (
        //                                         key,
        //                                         specta::internal::construct::field(
        //                                             false,
        //                                             false,
        //                                             None,
        //                                             Default::default(),
        //                                             Some(value.into()),
        //                                         ),
        //                                     )
        //                                 })
        //                                 .collect(),
        //                             None,
        //                         ),
        //                     ))
        //                 }
        //             }
        //         }
        //     }

        //     let mut types: HashMap<Cow<'static, str>, ProcedureOrProcedures> = Default::default();

        //     {
        //         for (key, procedure) in &procedures {
        //             let mut procedures_map = &mut types;

        //             let path = key.split(".").collect::<Vec<_>>();
        //             let Some((key, path)) = path.split_last() else {
        //                 panic!("how is this empty");
        //             };

        //             for segment in path {
        //                 let ProcedureOrProcedures::Procedures(nested_procedures_map) =
        //                     procedures_map
        //                         .entry(segment.to_string().into())
        //                         .or_insert(ProcedureOrProcedures::Procedures(Default::default()))
        //                 else {
        //                     panic!();
        //                 };

        //                 procedures_map = nested_procedures_map;
        //             }

        //             procedures_map.insert(
        //                 key.to_string().into(),
        //                 ProcedureOrProcedures::Procedure(Procedure {
        //                     kind: match procedure.kind() {
        //                         ProcedureKind::Query => "query",
        //                         ProcedureKind::Mutation => "mutation",
        //                         ProcedureKind::Subscription => "subscription",
        //                     }
        //                     .to_string(),
        //                     input: procedure.ty().input.clone(),
        //                     result: procedure.ty().result.clone(),
        //                     error: DataType::Any,
        //                 }),
        //             );
        //         }
        //     }

        //     #[derive(specta::Type)]
        //     struct Procedures;

        //     let mut named_type =
        //         <Procedures as specta::NamedType>::definition_named_data_type(&mut type_map);

        //     named_type.inner = ProcedureOrProcedures::Procedures(types).into();

        //     type_map.insert(<Procedures as specta::NamedType>::sid(), named_type);
        // }

        todo!();

        Ok((
            BTreeMap::<String, Procedure<TCtx>>::new().into_iter(),
            TypeCollection::default(),
        ))
    }
}

// TODO: `Iterator` or `IntoIterator`?

impl<TCtx> TryFrom<crate::legacy::Router<TCtx, ()>> for Router2<TCtx> {
    type Error = ();

    fn try_from(value: crate::legacy::Router<TCtx>) -> Result<Self, Self::Error> {
        // TODO: Enforce unique across all methods (query, subscription, etc)

        todo!()
    }
}
