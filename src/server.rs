use futures::Stream;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::hash::Hash;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use tokio::sync::{mpsc, Mutex};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::{Request, Response, Status};

use crate::store::inventory_server::Inventory;
use crate::store::{
    InventoryChangeResponse, InventoryUpdateResponse, Item, ItemIdentifier, PriceChangeRequest,
    QuantityChangeRequest,
};

pub struct StoreInventory{
    inventory : Arc<Mutex<HashMap<ItemIdentifier, Item>>>,
    traveller_map : Arc<Mutex<HashMap<String, Vector<ItemIdentifier>>>>,
}

impl Default for StoreInventory{
    fn default() -> Self {
        StoreInventory {
            inventory: Arc::new(Mutex::new(HashMap::<ItemIdentifier, Item>::new())),
            traveller_map: Arc::new(Mutex::new(HashMap::<String, Vector<ItemIdentifier>::new()>::new())),
        }
    }
}

#[tonic::async_trait]
impl Inventory for StoreInventory {
    
}

