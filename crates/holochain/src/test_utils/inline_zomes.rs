//! Collection of commonly used InlineZomes

use holo_hash::*;
use holochain_types::prelude::*;

/// A zome with no functions and no entry types
pub fn unit_zome() -> InlineZome {
    InlineZome::new_unique(vec![])
}

/// A DnaFile with just a unit Zome
pub async fn unit_dna() -> DnaFile {
    crate::sweettest::SweetDnaFile::unique_from_inline_zome("zome", unit_zome())
        .await
        .unwrap()
        .0
}

/// An InlineZome with simple Create and Read operations
pub fn simple_create_read_zome() -> InlineZome {
    let entry_def = EntryDef::default_with_id("entrydef");

    InlineZome::new_unique(vec![entry_def.clone()])
        .callback("create", move |api, ()| {
            let entry_def_id: EntryDefId = entry_def.id.clone();
            let entry = Entry::app(().try_into().unwrap()).unwrap();
            let hash = api.create(CreateInput::new(
                entry_def_id,
                entry,
                ChainTopOrdering::default(),
            ))?;
            Ok(hash)
        })
        .callback("read", |api, hash: HeaderHash| {
            api.get(vec![GetInput::new(hash.into(), GetOptions::default())])
                .map(|e| e.into_iter().next().unwrap())
                .map_err(Into::into)
        })
}
