use std::fmt::Formatter;
use std::marker::PhantomData;
use serde::{Deserialize, Deserializer};
use serde::de::{Error, MapAccess, Unexpected, Visitor};

#[derive(Deserialize, Debug)]
pub struct Save {
    #[serde(rename="Inventory")]
    #[serde(deserialize_with = "deserialize_inventory")]
    inventory: Vec<Inventory>,
}

fn deserialize_inventory<'de, D>(deserializer: D) -> Result<Vec<Inventory>, D::Error>
    where D: Deserializer<'de> {

    struct InventoryVisitor;

    impl<'de> Visitor<'de> for InventoryVisitor {
        type Value = Vec<Inventory>;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            formatter.write_str("a string containing map data")
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: MapAccess<'de> {
            let mut vec = Vec::with_capacity(map.size_hint().unwrap_or(0));
            while let Some((key, value)) = map.next_entry::<String, _>()? {
                let key = key.parse().unwrap();
                if vec.len() < key {
                    vec.resize_with(key+1, || Inventory::Missing);
                }
                vec[key] = value;
            }
            Ok(vec)
        }
    }

    deserializer.deserialize_map(InventoryVisitor)
}

fn deserialize_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
    where D: Deserializer<'de> {

    struct U32Visitor;

    impl<'de> Visitor<'de> for U32Visitor {
        type Value = u32;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            formatter.write_str("a string containing a number")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
            v.parse::<u32>().map_err(|e| E::invalid_type(Unexpected::Str(&v), &self))
        }
    }

    deserializer.deserialize_str(U32Visitor)
}

fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where D: Deserializer<'de> {

    struct BoolVisitor;

    impl<'de> Visitor<'de> for BoolVisitor {
        type Value = bool;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            formatter.write_str("a string containing a number representing a boolean")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
            match v.parse() {
                Ok(num) => {
                    match num {
                        0 => Ok(false),
                        1 => Ok(true),
                        i => Err(E::invalid_value(Unexpected::Unsigned(i), &self)),
                    }
                },
                Err(e) => Err(E::invalid_type(Unexpected::Str(&v), &self))
            }
        }
    }

    deserializer.deserialize_str(BoolVisitor)
}


fn deserialize_option_u32<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
    where D: Deserializer<'de> {

    struct OptionVisitor;

    impl<'de> Visitor<'de> for OptionVisitor {
        type Value = Option<u32>;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            formatter.write_str("a string containing an Option of a number")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
            match v.parse::<i64>() {
                Ok(num) => match num {
                    -1 => Ok(None),
                    n => Ok(Some(n as u32)),
                },
                Err(e) => Err(E::invalid_type(Unexpected::Str(&v), &self))
            }
        }
    }

    deserializer.deserialize_str(OptionVisitor)
}

#[derive(Deserialize, Debug)]
#[serde(tag = "Type")]
pub enum Inventory {
    #[serde(rename="obj_material")]
    MaterialInventory(MaterialInventory),
    #[serde(rename="obj_artifact")]
    ArtifactInventory(ArtifactInventory),
    #[serde(rename="obj_spellgem")]
    SpellgemInventory(SpellgemInventory),
    #[serde(rename="obj_netherstone")]
    NetherstoneInventory(NetherstoneInventory),
    #[serde(rename="obj_dust")]
    DustInventory(DustInventory),
    #[serde(rename="obj_consumable")]
    ConsumableInventory(ConsumableInventory),
    Missing,
}

#[derive(Deserialize, Debug)]
pub struct MaterialInventory {
    #[serde(rename="MaterialQuantity")]
    #[serde(deserialize_with="deserialize_u32")]
    material_quantity: u32,
    #[serde(rename="MaterialID")]
    #[serde(deserialize_with="deserialize_u32")]
    material_id: u32,
    #[serde(rename = "Looked")]
    #[serde(deserialize_with="deserialize_bool")]
    looked: bool,
}

#[derive(Deserialize, Debug)]
pub struct ArtifactInventory {
    #[serde(rename = "NetherPtr")]
    #[serde(deserialize_with="deserialize_option_u32")]
    pub nether_ptr: Option<u32>,
    #[serde(rename = "ArtifactAwakened")]
    pub artifact_awakened: String,
    #[serde(rename = "ArtifactSpell")]
    pub artifact_spell: String,
    #[serde(rename = "ArtifactTrait")]
    pub artifact_trait: String,
    #[serde(rename = "ArtifactStatSlot1")]
    #[serde(deserialize_with="deserialize_option_u32")]
    pub artifact_stat_slot1: Option<ArtifactStatSlot>,
    #[serde(rename = "ArtifactStatSlot2")]
    #[serde(deserialize_with="deserialize_option_u32")]
    pub artifact_stat_slot2: Option<ArtifactStatSlot>,
    #[serde(rename = "ArtifactStatSlot3")]
    #[serde(deserialize_with="deserialize_option_u32")]
    pub artifact_stat_slot3: Option<ArtifactStatSlot>,
    #[serde(rename = "ArtifactStatSlot4")]
    #[serde(deserialize_with="deserialize_option_u32")]
    pub artifact_stat_slot4: Option<ArtifactStatSlot>,
    #[serde(rename = "ArtifactTrickSlot1")]
    #[serde(deserialize_with="deserialize_option_u32")]
    pub artifact_trick_slot1: Option<ArtifactTrickSlot>,
    #[serde(rename = "ArtifactTrickSlot2")]
    #[serde(deserialize_with="deserialize_option_u32")]
    pub artifact_trick_slot2: Option<ArtifactTrickSlot>,
    #[serde(rename = "ArtifactTier")]
    #[serde(deserialize_with="deserialize_u32")]
    pub artifact_tier: u32,
    #[serde(rename = "ArtifactGUID")]
    #[serde(deserialize_with="deserialize_u32")]
    pub artifact_guid: u32,
    #[serde(rename = "ArtifactLocked")]
    #[serde(deserialize_with="deserialize_bool")]
    pub artifact_locked: bool,
    #[serde(rename = "ArtifactNickname")]
    pub artifact_nickname: String,
    #[serde(rename = "ArtifactType")]
    #[serde(deserialize_with="deserialize_u32")]
    pub artifact_type: ArtifactType,
    #[serde(rename = "Looked")]
    #[serde(deserialize_with="deserialize_bool")]
    pub looked: bool,
}

pub type ArtifactType = u32;
pub type ArtifactStatSlot = u32;
pub type ArtifactTrickSlot = u32;

#[derive(Deserialize, Debug)]
pub struct SpellgemInventory {

}

#[derive(Deserialize, Debug)]
pub struct NetherstoneInventory {

}

#[derive(Deserialize, Debug)]
pub struct DustInventory {

}

#[derive(Deserialize, Debug)]
pub struct ConsumableInventory {

}