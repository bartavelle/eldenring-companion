use packed_struct::prelude::*;

#[allow(non_camel_case_types)]
#[derive(PackedStruct, Debug, Clone)]
#[packed_struct(endian = "lsb", bit_numbering = "msb0")]
// field positions are reverted, this is actually lsb0
pub struct ATTACK_ELEMENT_CORRECT_PARAM_ST {
    pub is_magic_correct_by_magic: bool,       // 理力補正するか（魔法） -
    pub is_dexterity_correct_by_magic: bool,   // 技量補正するか（魔法） -
    pub is_strength_correct_by_magic: bool,    // 筋力補正するか（魔法） -
    pub is_luck_correct_by_physics: bool,      // 運補正するか（物理） -
    pub is_faith_correct_by_physics: bool,     // 信仰補正するか（物理） -
    pub is_magic_correct_by_physics: bool,     // 理力補正するか（物理） -
    pub is_dexterity_correct_by_physics: bool, // 技量補正するか（物理） -
    pub is_strength_correct_by_physics: bool,  // 筋力補正するか（物理） -

    pub is_strength_correct_by_thunder: bool, // 筋力補正するか（雷） -
    pub is_luck_correct_by_fire: bool,        // 運補正するか（炎） -
    pub is_faith_correct_by_fire: bool,       // 信仰補正するか（炎） -
    pub is_magic_correct_by_fire: bool,       // 理力補正するか（炎） -
    pub is_dexterity_correct_by_fire: bool,   // 技量補正するか（炎） -
    pub is_strength_correct_by_fire: bool,    // 筋力補正するか（炎） -
    pub is_luck_correct_by_magic: bool,       // 運補正するか（魔法） -
    pub is_faith_correct_by_magic: bool,      // 信仰補正するか（魔法） -

    pub is_faith_correct_by_dark: bool,        // 信仰補正するか（闇） -
    pub is_magic_correct_by_dark: bool,        // 理力補正するか（闇） -
    pub is_dexterity_correct_by_dark: bool,    // 技量補正するか（闇） -
    pub is_strength_correct_by_dark: bool,     // 筋力補正するか（闇） -
    pub is_luck_correct_by_thunder: bool,      // 運補正するか（雷） -
    pub is_faith_correct_by_thunder: bool,     // 信仰補正するか（雷） -
    pub is_magic_correct_by_thunder: bool,     // 理力補正するか（雷） -
    pub is_dexterity_correct_by_thunder: bool, // 技量補正するか（雷） -

    pub pad1: Integer<u8, packed_bits::Bits<7>>, // パディング -
    pub is_luck_correct_by_dark: bool,           // 運補正するか（闇） -

    pub overwrite_strength_correct_rate_by_physics: i16, // 筋力補正値上書き（物理） -
    pub overwrite_dexterity_correct_rate_by_physics: i16, // 技量補正値上書き（物理） -
    pub overwrite_magic_correct_rate_by_physics: i16,    // 理力補正値上書き（物理） -
    pub overwrite_faith_correct_rate_by_physics: i16,    // 信仰補正値上書き（物理） -
    pub overwrite_luck_correct_rate_by_physics: i16,     // 運補正値上書き（物理） -
    pub overwrite_strength_correct_rate_by_magic: i16,   // 筋力補正値上書き（魔法） -
    pub overwrite_dexterity_correct_rate_by_magic: i16,  // 技量補正値上書き（魔法） -
    pub overwrite_magic_correct_rate_by_magic: i16,      // 理力補正値上書き（魔法） -
    pub overwrite_faith_correct_rate_by_magic: i16,      // 信仰補正値上書き（魔法） -
    pub overwrite_luck_correct_rate_by_magic: i16,       // 運補正値上書き（魔法） -
    pub overwrite_strength_correct_rate_by_fire: i16,    // 筋力補正値上書き（炎） -
    pub overwrite_dexterity_correct_rate_by_fire: i16,   // 技量補正値上書き（炎） -
    pub overwrite_magic_correct_rate_by_fire: i16,       // 理力補正値上書き（炎） -
    pub overwrite_faith_correct_rate_by_fire: i16,       // 信仰補正値上書き（炎） -
    pub overwrite_luck_correct_rate_by_fire: i16,        // 運補正値上書き（炎） -
    pub overwrite_strength_correct_rate_by_thunder: i16, // 筋力補正値上書き（雷） -
    pub overwrite_dexterity_correct_rate_by_thunder: i16, // 技量補正値上書き（雷） -
    pub overwrite_magic_correct_rate_by_thunder: i16,    // 理力補正値上書き（雷） -
    pub overwrite_faith_correct_rate_by_thunder: i16,    // 信仰補正値上書き（雷） -
    pub overwrite_luck_correct_rate_by_thunder: i16,     // 運補正値上書き（雷） -
    pub overwrite_strength_correct_rate_by_dark: i16,    // 筋力補正値上書き（闇） -
    pub overwrite_dexterity_correct_rate_by_dark: i16,   // 技量補正値上書き（闇） -
    pub overwrite_magic_correct_rate_by_dark: i16,       // 理力補正値上書き（闇） -
    pub overwrite_faith_correct_rate_by_dark: i16,       // 信仰補正値上書き（闇） -
    pub overwrite_luck_correct_rate_by_dark: i16,        // 運補正値上書き（闇） -
    pub influence_strength_correct_rate_by_physics: i16, // 筋力補正値影響率（物理） - 補正率の影響割合。
    pub influence_dexterity_correct_rate_by_physics: i16, // 技量補正値影響率（物理） - 補正率の影響割合。
    pub influence_magic_correct_rate_by_physics: i16,    // 理力補正値影響率（物理） - 補正率の影響割合。
    pub influence_faith_correct_rate_by_physics: i16,    // 信仰補正値影響率（物理） - 補正率の影響割合。
    pub influence_luck_correct_rate_by_physics: i16,     // 運補正値影響率（物理） - 補正率の影響割合。
    pub influence_strength_correct_rate_by_magic: i16,   // 筋力補正値影響率（魔法） - 補正率の影響割合。
    pub influence_dexterity_correct_rate_by_magic: i16,  // 技量補正値影響率（魔法） - 補正率の影響割合。
    pub influence_magic_correct_rate_by_magic: i16,      // 理力補正値影響率（魔法） - 補正率の影響割合。
    pub influence_faith_correct_rate_by_magic: i16,      // 信仰補正値影響率（魔法） - 補正率の影響割合。
    pub influence_luck_correct_rate_by_magic: i16,       // 運補正値影響率（魔法） - 補正率の影響割合。
    pub influence_strength_correct_rate_by_fire: i16,    // 筋力補正値影響率（炎） - 補正率の影響割合。
    pub influence_dexterity_correct_rate_by_fire: i16,   // 技量補正値影響率（炎） - 補正率の影響割合。
    pub influence_magic_correct_rate_by_fire: i16,       // 理力補正値影響率（炎） - 補正率の影響割合。
    pub influence_faith_correct_rate_by_fire: i16,       // 信仰補正値影響率（炎） - 補正率の影響割合。
    pub influence_luck_correct_rate_by_fire: i16,        // 運補正値影響率（炎） - 補正率の影響割合。
    pub influence_strength_correct_rate_by_thunder: i16, // 筋力補正値影響率（雷） - 補正率の影響割合。
    pub influence_dexterity_correct_rate_by_thunder: i16, // 技量補正値影響率（雷） - 補正率の影響割合。
    pub influence_magic_correct_rate_by_thunder: i16,    // 理力補正値影響率（雷） - 補正率の影響割合。
    pub influence_faith_correct_rate_by_thunder: i16,    // 信仰補正値影響率（雷） - 補正率の影響割合。
    pub influence_luck_correct_rate_by_thunder: i16,     // 運補正値影響率（雷） - 補正率の影響割合。
    pub influence_strength_correct_rate_by_dark: i16,    // 筋力補正値影響率（闇） - 補正率の影響割合。
    pub influence_dexterity_correct_rate_by_dark: i16,   // 技量補正値影響率（闇） - 補正率の影響割合。
    pub influence_magic_correct_rate_by_dark: i16,       // 理力補正値影響率（闇） - 補正率の影響割合。
    pub influence_faith_correct_rate_by_dark: i16,       // 信仰補正値影響率（闇） - 補正率の影響割合。
    pub influence_luck_correct_rate_by_dark: i16,        // 運補正値影響率（闇） - 補正率の影響割合。
    pub pad2: [u8; 24],                                  // パディング -
}
