use packed_struct::prelude::*;

#[allow(non_camel_case_types)]
#[derive(PackedStruct, Debug, Clone)]
#[packed_struct(endian = "lsb", bit_numbering = "msb0")]
pub struct REINFORCE_PARAM_PROTECTOR_ST {
    // only support unicode
    pub physics_def_rate: f32,      // 物理防御力 - 物理防御力の補正値
    pub magic_def_rate: f32,        // 魔法防御力 - 魔法防御力の補正値
    pub fire_def_rate: f32,         // 炎防御力 - 炎防御力の補正値
    pub thunder_def_rate: f32,      // 電撃防御力 - 電撃防御力の補正値
    pub slash_def_rate: f32,        // 斬撃防御力 - 斬撃防御力の補正値
    pub blow_def_rate: f32,         // 打撃防御力 - 打撃防御力の補正値
    pub thrust_def_rate: f32,       // 刺突防御力 - 刺突防御力の補正値
    pub resist_poison_rate: f32,    // 毒耐性 - 毒耐性の補正値
    pub resist_disease_rate: f32,   // 疫病耐性 - 疫病耐性の補正値
    pub resist_blood_rate: f32,     // 出血耐性 - 出血耐性の補正値
    pub resist_curse_rate: f32,     // 呪耐性 - 呪耐性の補正値
    pub resident_sp_effect_id1: u8, // 常駐特殊効果ID1 - 常駐特殊効果ID1の加算補正値
    pub resident_sp_effect_id2: u8, // 常駐特殊効果ID2 - 常駐特殊効果ID2の加算補正値
    pub resident_sp_effect_id3: u8, // 常駐特殊効果ID3 - 常駐特殊効果ID3の加算補正値
    pub material_set_id: u8,        // 素材ID加算値 - 素材パラメータIDの加算補正値
}
