use stdweb::{Reference, Value};
use stdweb::unstable::{TryFrom, TryInto};

use api::{Direction, Part, ReturnCode};

macro_rules! reference_wrappers {
    ($name:ident) => {
        pub struct $name(Reference);

        impl AsRef<Reference> for $name {
            fn as_ref(&self) -> &Reference {
                &self.0
            }
        }
        impl TryFrom<Value> for $name {
            type Error = <Value as TryInto<Reference>>::Error;

            fn try_from(v: Value) -> Result<$name, Self::Error> {
                Ok($name(v.try_into()?))
            }
        }
    };
    ($($name:ident),* $(,)*) => {
        $(
            reference_wrappers!($name);
        )*
    };
}

reference_wrappers!(
    ConstructionSite,
    Creep,
    Flag,
    Mineral,
    Nuke,
    Resource,
    Room,
    RoomPosition,
    Source,
    StructureContainer,
    StructureController,
    StructureExtension,
    StructureExtractor,
    StructureKeeperLair,
    StructureLab,
    StructureLink,
    StructureNuker,
    StructureObserver,
    StructurePowerBank,
    StructurePowerSpawn,
    StructurePortal,
    StructureRampart,
    StructureRoad,
    StructureSpawn,
    StructureStorage,
    StructureTerminal,
    StructureTower,
    StructureWall,
    Structure,
);

macro_rules! js_unwrap {
    ($($code:tt)*) => ((js! { return $($code)* }).try_into().unwrap())
}

impl Creep {
    pub fn carry_total(&self) -> i32 {
        js_unwrap!(_.sum(@{&self.0}.carry))
    }

    pub fn cancel_order(&self, name: &str) -> ReturnCode {
        js_unwrap!(@{&self.0}.cancelOrder(@{name}))
    }

    pub fn move_direction(&self, dir: Direction) -> ReturnCode {
        js_unwrap!(@{&self.0}.move(@{dir as i32}))
    }

    pub fn move_to_xy(&self, x: i32, y: i32) -> ReturnCode {
        js_unwrap!(@{&self.0}.moveTo(@{x}, @{y}))
    }

    pub fn say(&self, msg: &str, public: bool) -> ReturnCode {
        js_unwrap!(@{&self.0}.say(@{msg}, @{public}))
    }

    pub fn sign_controller<T>(&self, target: &T, text: &str) -> ReturnCode
    where
        T: AsRef<Reference>,
    {
        js_unwrap!(@{&self.0}.signController(@{target.as_ref()}, @{text}))
    }

    pub fn suicide(&self) -> ReturnCode {
        js_unwrap!(@{&self.0}.suicide())
    }

    // TODO: need ResourceType before doing transfer, withdraw
}

macro_rules! simple_accessors {
    ($struct_name:ident; $method_name:ident -> $prop_name:ident -> $ret_type:ty) => (
        impl $struct_name {
            pub fn $method_name(&self) -> $ret_type {
                js_unwrap!(@{&self.0}.$prop_name)
            }
        }
    );
    ($struct_name:ident; $(($method:ident -> $prop:ident -> $ret:ty)),* $(,)*) => (
        $(
            simple_accessors!($struct_name; $method -> $prop -> $ret);
        )*
    )
}
simple_accessors! {
    Creep;
    (pos -> pos -> RoomPosition),
    (room -> room -> Room),
    (carry_capacity -> carryCapacity -> i32),
    (fatigue -> fatigue -> i32),
    (hits -> hits -> i32),
    (hits_max -> hitsMax -> i32),
    (my -> my -> bool),
    (spawning -> spawning -> bool),
    (ticks_to_live -> ticksToLive -> i32),
}

macro_rules! creep_simple_action {
    ($method_name:ident -> $js_name:ident) => (
        impl Creep {
            pub fn $method_name<T>(&self, target: &T) -> ReturnCode
            where
                T: AsRef<Reference>,
            {
                js_unwrap!(@{&self.0}.$js_name(@{target.as_ref()}))
            }
        }
    );
    ($(($method:ident -> $js_name:ident)),* $(,)*) => (
        $(
            creep_simple_action!($method -> $js_name);
        )*
    )
}

creep_simple_action! {
    (attack -> attack),
    (attack_controller -> attackController),
    (build -> build),
    (claim_controller -> claimController),
    (dismantle -> dismantle),
    (generate_safe_mode -> generateSafeMode),
    (harvest -> harvest),
    (heal -> heal),
    (move_to -> moveTo),
    (pickup -> pickup),
    (ranged_attack -> rangedAttack),
    (ranged_heal -> rangedHeal),
    (ranged_mass_attack -> rangedMassAttack),
    (repair -> repair),
    (reserve_controller -> reserveController),
    (upgrade_controller -> upgradeController),
}

// TODO:

// pub trait RoomObjectProps {
//     pub fn pos(&self) -> RoomPosition;
//     pub fn room(&self) -> Room;
// }

// pub trait StructureProps: RoomObject {
//     pub fn hits(&self) -> i32;
//     pub fn hits_max(&self) -> i32;
//     pub fn id(&self) -> String;
//     // TODO: StructureType
//     pub fn destroy(&self) -> ReturnCode;
//     pub fn is_active(&self) -> bool;
// }

macro_rules! impl_structure {
    ($name:ident) => (
        simple_accessors! {
            $name;
            (pos -> pos -> RoomPosition),
            (room -> room -> Room),
            (hits -> hits -> i32),
            (hits_max -> hitsMax -> i32),
            (id -> id -> String),
            (is_active -> isActive -> bool),
        }

        impl $name {
            pub fn destroy(&self) -> ReturnCode {
                js_unwrap!(@{&self.0}.destroy())
            }
        }
    );

    ($($name:ident),* $(,)*) => (
        $(
            impl_structure!($name);
        )*
    )
}

impl_structure!(
    StructureContainer,
    StructureController,
    StructureExtension,
    StructureExtractor,
    StructureKeeperLair,
    StructureLab,
    StructureLink,
    StructureNuker,
    StructureObserver,
    StructurePowerBank,
    StructurePowerSpawn,
    StructurePortal,
    StructureRampart,
    StructureRoad,
    StructureSpawn,
    StructureStorage,
    StructureTerminal,
    StructureTower,
    StructureWall,
);

simple_accessors! {
    StructureSpawn;
    (energy -> energy -> i32),
    (energy_capacity -> energyCapacity -> i32),
    (name -> name -> String),
}

impl StructureSpawn {
    pub fn spawn_creep(&self, body: &[Part], name: &str) -> ReturnCode {
        let ints = body.iter().map(|p| *p as i32).collect::<Vec<i32>>();
        ((js! {
            var body = (@{ints}).map((num) => {
                switch (num) {
                    case 0: return WORK;
                    case 1: return MOVE;
                    case 2: return CARRY;
                    case 3: return ATTACK;
                    case 4: return RANGED_ATTACK;
                    case 5: return HEAL;
                    case 6: return TOUGH;
                    case 7: return CLAIM;
                }
            });


            return @{&self.0}.spawnCreep(body, @{name});
        }).try_into()
            .unwrap())
    }
}

impl RoomPosition {
    pub fn is_near_to<T>(&self, target: &T) -> bool
    where
        T: AsRef<Reference>,
    {
        js_unwrap!(@{&self.0}.isNearTo(@{target.as_ref()}))
    }
}

simple_accessors! {
    Room;
    (controller -> controller -> Option<StructureController>),
    (energy_available -> energyAvailable -> i32),
    (energy_capacity_available -> energyCapacityAvailable -> i32),
    (name -> name -> String),
    (storage -> storage -> Option<StructureStorage>),
    (terminal -> terminal -> Option<StructureTerminal>),
}