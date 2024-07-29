use packed_struct::prelude::*;

#[allow(non_camel_case_types)]
#[derive(PackedStruct, Debug, Clone)]
#[packed_struct(endian="lsb", bit_numbering="msb0")]
pub struct REINFORCE_PARAM_WEAPON_ST {
    pub physics_atk_rate: f32, // 物理攻撃力基本値 - 物理攻撃力の補正値
    pub magic_atk_rate: f32, // 魔法攻撃力基本値 - 魔法攻撃力の補正値
    pub fire_atk_rate: f32, // 炎攻撃力基本値 - 炎攻撃力の補正値
    pub thunder_atk_rate: f32, // 電撃攻撃力基本値 - 電撃攻撃力の補正値
    pub stamina_atk_rate: f32, // スタミナ攻撃力 - スタミナ攻撃力の補正値
    pub sa_weapon_atk_rate: f32, // SA武器攻撃力 - スーパーアーマー武器攻撃色の補正値
    pub sa_durability_rate: f32, // SA耐久値 - SA耐久力の補正値
    pub correct_strength_rate: f32, // 筋力補正 - 筋力補正の補正値
    pub correct_agility_rate: f32, // 俊敏補正 - 俊敏補正の補正値
    pub correct_magic_rate: f32, // 魔力補正 - 魔力補正の補正値
    pub correct_faith_rate: f32, // 信仰補正 - 信仰補正の補正値
    pub physics_guard_cut_rate: f32, // ガード時物理攻撃カット率 - ガード時物理攻撃カット率の補正値
    pub magic_guard_cut_rate: f32, // ガード時魔法攻撃カット率 - ガード時魔法攻撃カット率の補正値
    pub fire_guard_cut_rate: f32, // ガード時炎攻撃カット率 - ガード時炎攻撃カット率の補正値
    pub thunder_guard_cut_rate: f32, // ガード時電撃攻撃カット率 - ガード時電撃攻撃カット率の補正値
    pub poison_guard_resist_rate: f32, // ガード時毒攻撃カット率 - ガード時毒攻撃カット率の補正値
    pub disease_guard_resist_rate: f32, // ガード時疫病攻撃カット率 - ガード時疫病攻撃カット率の補正値
    pub blood_guard_resist_rate: f32, // ガード時出血攻撃カット率 - ガード時出血攻撃カット率の補正値
    pub curse_guard_resist_rate: f32, // ガード時呪攻撃カット率 - ガード時呪い攻撃カット率の補正値
    pub stamina_guard_def_rate: f32, // ガード時スタミナ防御力 - ガード時スタミナ防御力の補正値
    pub sp_effect_id1: u8, // 特殊効果ID1 - 特殊効果ID1の加算補正値
    pub sp_effect_id2: u8, // 特殊効果ID2 - 特殊効果ID2の加算補正値
    pub sp_effect_id3: u8, // 特殊効果ID3 - 特殊効果ID3の加算補正値
    pub resident_sp_effect_id1: u8, // 常駐特殊効果ID1 - 常駐特殊効果ID1の加算補正値
    pub resident_sp_effect_id2: u8, // 常駐特殊効果ID2 - 常駐特殊効果ID2の加算補正値
    pub resident_sp_effect_id3: u8, // 常駐特殊効果ID3 - 常駐特殊効果ID3の加算補正値
    pub material_set_id: u8, // 素材ID加算値 - 素材パラメータIDの加算補正値
    pub max_reinforce_level: u8, // 最大強化武器レベル用レベル値 - 最大強化武器レベル用レベル値
    pub dark_atk_rate: f32, // 闇攻撃力基本値 - 闇攻撃力の補正値
    pub dark_guard_cut_rate: f32, // ガード時闇攻撃カット率 - ガード時闇攻撃カット率の補正値
    pub correct_luck_rate: f32, // 運補正 - 運補正の補正値
    pub freeze_guard_def_rate: f32, // ガード時冷気攻撃カット率 - ガード時冷気攻撃カット率の補正値
    pub reinforce_price_rate: f32, // 強化価格補正値 - 武器パラメータの強化価格に乗算する補正値
    pub base_change_price_rate: f32, // 進化価格補正値 - 武器パラメータの進化価格に乗算する補正値
    pub enable_gem_rank: i8, // 装着可能魔石ランク - 装着可能魔石ランク
    pub pad2: [u8;3], // pad - 
    pub sleep_guard_def_rate: f32, // ガード時睡眠攻撃カット率 - ガード時睡眠攻撃カット率の補正値
    pub madness_guard_def_rate: f32, // ガード時発狂攻撃カット率 - ガード時発狂攻撃カット率の補正値
    pub base_atk_rate: f32, // 加算攻撃力倍率 - 加算攻撃力倍率
}
