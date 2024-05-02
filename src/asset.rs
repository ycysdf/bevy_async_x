use std::future::Future;

use bevy_asset::{Asset, AssetPath, AssetServer, Handle, LoadState, UntypedAssetId};

use crate::when;

pub trait AsyncXAssetExt {
    fn when_loaded_by_id(&self, id: impl Into<UntypedAssetId>) -> impl Future<Output = ()>;
    fn when_loaded<'a, A>(
        &self,
        path: impl Into<AssetPath<'a>>,
    ) -> impl Future<Output = Handle<A>>
    where
        A: Asset;
}

impl AsyncXAssetExt for AssetServer {
    fn when_loaded_by_id(&self, id: impl Into<UntypedAssetId>) -> impl Future<Output = ()> {
        let id = id.into();
        when(move || matches!(self.load_state(id), LoadState::Loaded))
    }

    fn when_loaded<'a, A>(
        &self,
        path: impl Into<AssetPath<'a>>,
    ) -> impl Future<Output = Handle<A>>
    where
        A: Asset,
    {
        let handle = self.load(path);
        async move {
            self.when_loaded_by_id(&handle).await;
            handle
        }
    }
}
