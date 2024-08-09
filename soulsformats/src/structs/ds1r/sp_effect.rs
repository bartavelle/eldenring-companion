use packed_struct::prelude::*;

// TODO: fix bools

#[allow(non_camel_case_types)]
#[derive(PackedStruct, Debug, Clone)]
#[packed_struct(endian="lsb", bit_numbering="msb0")]
pub struct SP_EFFECT_PARAM_ST {
// only support unicode
    pub icon_id: i32, // アイコンID - アイコンID(-1の時は、アイコン必要なし)
    pub condition_hp: f32, // 発動条件　残りHP比率[%] - 残りHPが、maxHPの何%になったら発動するかを設定
    pub effect_endurance: f32, // 効果持続時間　時間[s] - 変化持続時間　/-1で永続 /0で瞬間1回限り
    pub motion_interval: f32, // 発動間隔[s] - 何秒間隔で発生するのかを設定
    pub max_hp_rate: f32, // 最大HP倍率[%] - 最大HPに補正をかける
    pub max_mp_rate: f32, // 最大MP倍率[%] - 最大MPに補正をかける
    pub max_stamina_rate: f32, // 最大スタミナ倍率[%] - 最大SPに補正をかける
    pub slash_damage_cut_rate: f32, // 防御側：斬撃ダメージ倍率 - 斬撃ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
    pub blow_damage_cut_rate: f32, // 防御側：打撃ダメージ倍率 - 打撃ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
    pub thrust_damage_cut_rate: f32, // 防御側：刺突ダメージ倍率 - 刺突ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
    pub neutral_damage_cut_rate: f32, // 防御側：無属性ダメージ倍率 - 無属性ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
    pub magic_damage_cut_rate: f32, // 防御側：魔法ダメージ倍率 - 魔法ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
    pub fire_damage_cut_rate: f32, // 防御側：炎ダメージ倍率 - 炎ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
    pub thunder_damage_cut_rate: f32, // 防御側：電撃ダメージ倍率 - 炎ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
    pub physics_attack_rate: f32, // 攻撃側：物理ダメージ倍率 - 物理ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
    pub magic_attack_rate: f32, // 攻撃側：魔法ダメージ倍率 - 魔法ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
    pub fire_attack_rate: f32, // 攻撃側：炎ダメージ倍率 - 炎ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
    pub thunder_attack_rate: f32, // 攻撃側：電撃ダメージ倍率 - 電撃ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
    pub physics_attack_power_rate: f32, // 物理攻撃力倍率 - 物理攻撃力に設定した数値をかけます
    pub magic_attack_power_rate: f32, // 魔法攻撃力倍率 - 魔法攻撃力に設定した数値をかけます
    pub fire_attack_power_rate: f32, // 炎攻撃力倍率 - 炎攻撃力に設定した数値をかけます
    pub thunder_attack_power_rate: f32, // 電撃攻撃力倍率 - 電撃攻撃力に設定した数値をかけます
    pub physics_attack_power: i32, // 物理攻撃力[point] - 物理攻撃力に設定した数値を加減算する
    pub magic_attack_power: i32, // 魔法攻撃力[point] - 魔法攻撃力に設定した数値を加減算する
    pub fire_attack_power: i32, // 炎攻撃力[point] - 炎攻撃力に設定した数値を加減算する
    pub thunder_attack_power: i32, // 電撃攻撃力[point] - 電撃攻撃力に設定した数値を加減算する
    pub physics_diffence_rate: f32, // 物理防御力倍率 - 物理防御力に設定した数値をかけます
    pub magic_diffence_rate: f32, // 魔法防御力倍率 - 魔法防御力に設定した数値をかけます
    pub fire_diffence_rate: f32, // 炎防御力倍率 - 炎防御力に設定した数値をかけます
    pub thunder_diffence_rate: f32, // 電撃防御力倍率 - 電撃防御力に設定した数値をかけます
    pub physics_diffence: i32, // 物理防御力[point] - 物理防御力に設定した数値を加減算する
    pub magic_diffence: i32, // 魔法防御力[point] - 魔法防御力に設定した数値を加減算する
    pub fire_diffence: i32, // 炎防御力[point] - 炎防御力に設定した数値を加減算する
    pub thunder_diffence: i32, // 電撃防御力[point] - 電撃防御力に設定した数値を加減算する
    pub no_guard_damage_rate: f32, // 隙ダメージ倍率 - 隙のときのダメージ倍率を、設定した数値に置き換える（ダメージ側に設定）
    pub vital_spot_change_rate: f32, // スィートスポット倍率 - スィートスポットのダメージ計算を指定した数値に差し替える(急所ダメージ補正) -1.0で無効
    pub normal_spot_change_rate: f32, // ノーマルヒット倍率 - ノーマルヒットのダメージ計算を指定した数値に差し替える  -1.0で無効
    pub max_hp_change_rate: f32, // 最大HP変化倍率 - 最大HPに対して、設定された倍率をかけて増減させる
    pub behavior_id: i32, // 行動ID指定枠 - 特殊効果から行動IDを使ってダメージを与える場合に指定-1で無効
    pub change_hp_rate: f32, // HPダメージ量[%] - 最大HPの何%分を毎秒加算（または減算）するかを設定
    pub change_hp_point: i32, // HPダメージ[point] - 1秒間に何ポイント加算（または減算）するかを設定
    pub change_mp_rate: f32, // MPダメージ量[%] - 最大MPの何%分を毎秒加算（または減算）するかを設定
    pub change_mp_point: i32, // MPダメージ[point] - 1秒間に何ポイント加算（または減算）するかを設定
    pub mp_recover_change_speed: i32, // MP回復速度変化[point] - 回復速度を変化させる。回復計算式の基準回復速度、初期回復速度に加減算する。
    pub change_stamina_rate: f32, // スタミナダメージ量[%] - 最大スタミナの何%分を毎秒加算（または減算）するかを設定
    pub change_stamina_point: i32, // スタミナダメージ[point] - 1秒間に何ポイント加算（または減算）するかを設定
    pub stamina_recover_change_speed: i32, // スタミナ回復速度変化[point] - 回復速度を変化させる。回復計算式の基準回復速度、初期回復速度に加減算する。
    pub magic_effect_time_change: f32, // 魔法効果時間変化 - 効果持続時間に0.1秒以上設定されている魔法のみ、効果持続時間に設定されている時間を加減算する
    pub inside_durability: i32, // 耐久度変化：内部損耗度[point] - 内部損耗度に数値分を加減算する
    pub max_durability: i32, // 耐久度変化：最大損耗度変化[point] - 耐久度の内部損耗度の最大値に、設定された数値を加算する
    pub stamina_attack_rate: f32, // スタミナ攻撃力倍率 - スタミナ攻撃力に、倍率をかける(1.0 1倍 0.5 半分）
    pub poizon_attack_power: i32, // 毒耐性攻撃力[point] - ヒットした時に、対象の【毒耐性値】に加算する数値
    pub regist_illness: i32, // 疫病耐性攻撃力[point] - ヒットした時に、対象の【疫病耐性値】に加算する数値
    pub regist_blood: i32, // 出血耐性攻撃力[point] - ヒットした時に、対象の【出血耐性値】に加算する数値
    pub regist_curse: i32, // 呪耐性攻撃力[point] - ヒットした時に、対象の【呪耐性値】に加算する数値
    pub fall_damage_rate: f32, // 落下ダメージ倍率 - 落下時のダメージ計算に倍率をかける
    pub soul_rate: f32, // 取得ソウル倍率 - 敵を倒した時の取得ソウル量が、指定倍数分上乗せされる
    pub equip_weight_change_rate: f32, // 装備重量変化倍率 - 最大装備重量に、設定された倍率をかける
    pub all_item_weight_change_rate: f32, // 所持重量変化倍率 - 最大所持重量に、設定された倍率をかける
    pub soul: i32, // ソウル加算 - 所持ソウルに、設定値を加算する
    pub anim_id_offset: i32, // アニメIDオフセット - アニメIDオフセット
    pub have_soul_rate: f32, // 所持ソウル率 - 敵周回効果用。設定されているキャラから外にソウルが出て行く時に適用されます。
    pub target_priority: f32, // ターゲット優先度加算分 - マルチプレイ時、敵から優先的にターゲットとして狙われるようになる。プライオリティの加算。０がデフォルト。プラス値でよく狙われるようになる。マイナスは、－１まで。
    pub sight_search_enemy_cut: i32, // 視覚索敵カット - AIの視覚索敵対象から外れやすくなる。０がデフォルト。
    pub hearing_search_enemy_cut: i32, // 聴覚索敵カット - AIの聴覚索敵対象から外れやすくなる。０がデフォルト。
    pub grabity_rate: f32, // グラビティ率 - グラビティ率
    pub regist_poizon_change_rate: f32, // 毒耐性変化倍率 - 毒耐性値に設定された倍率をかける
    pub regist_illness_change_rate: f32, // 疫病耐性変化倍率 - 疫病耐性値に設定された倍率をかける
    pub regist_blood_change_rate: f32, // 出血耐性変化倍率 - 出血耐性値に設定された倍率をかける
    pub regist_curse_change_rate: f32, // 呪耐性変化倍率 - 呪耐性値に設定された倍率をかける
    pub soul_steal_rate: f32, // ソウルスティール係数 - NPCがソウルスティールで奪われるHPに対する防御力
    pub life_reduction_rate: f32, // 防御：寿命係数 - 
    pub hp_recover_rate: f32, // HP回復量係数 - HPが減るときは、効かない。
    pub replace_sp_effect_id: i32, // 差し替える特殊効果 - 寿命が尽きた時に追加される特殊効果ID(-1は無視)
    pub cycle_occurrence_sp_effect_id: i32, // 周期発生特殊効果 - 発動周期毎に発生する特殊効果ID(-1は無視)
    pub atk_occurrence_sp_effect_id: i32, // 攻撃発生特殊効果 - 攻撃Hit時に発生する特殊効果ID(-1は無視)
    pub guard_def_flick_power_rate: f32, // ガード時はじき防御力アップ倍率 - ガード時のはじき防御力補正値
    pub guard_stamina_cut_rate: f32, // ガード時スタミナカット倍率 - ガード時のスタミナカット率補正値
    pub ray_cast_passed_time: i16, // 視線通過：発動時間[ms] - 視線通過：発動時間[ms]（邪眼用）
    pub change_super_armor_point: i16, // SA値[point] - スーパーアーマー値に加算する値
    pub bow_dist_rate: i16, // 弓飛距離補正[％] - 武器の飛距離補正に加算される補正値
    pub sp_category: u16, // 特殊効果カテゴリ - 特殊効果の上書きなどの挙動を決めるカテゴリ
    pub category_priority: u8, // カテゴリ内優先度 - 同一カテゴリ内での優先度（低い方が優先）
    pub save_category: i8, // 保存カテゴリ - 特殊効果を保存するカテゴリ
    pub change_magic_slot: u8, // 魔法登録枠変化　魔法スロット - 魔法登録枠を指定数増やすことが出来る
    pub change_miracle_slot: u8, // 奇跡登録枠変化　奇跡スロット - 軌跡登録枠を指定数増やすことが出来る
    pub hero_point_damage: i8, // 人間性ダメージ値 - 人間性値に与えるダメージ値
    pub def_flick_power: u8, // はじき防御力_上書き - はじき防御力を上書きする値を設定
    pub flick_damage_cut_rate: u8, // はじき時ダメージ減衰率[%]_上書き - はじき時のダメージ減衰率を上書きする値を設定
    pub blood_damage_rate: u8, // 出血ダメージ補正倍率 - 
    pub dmg_lvnone: i8, // DL_ダメージなし（0） - ダメージLv0を差し替えるタイプを指定
    pub dmg_lvs: i8, // DL_小（1） - ダメージLv1を差し替えるタイプを指定
    pub dmg_lvm: i8, // DL_中（2） - ダメージLv2を差し替えるタイプを指定
    pub dmg_lvl: i8, // DL_大（3） - ダメージLv3を差し替えるタイプを指定
    pub dmg_lvblow_m: i8, // DL_吹っ飛び（4） - ダメージLv4を差し替えるタイプを指定
    pub dmg_lvpush: i8, // DL_プッシュ（5） - ダメージLv5を差し替えるタイプを指定
    pub dmg_lvstrike: i8, // DL_叩きつけ（6） - ダメージLv6を差し替えるタイプを指定
    pub dmg_lvblow_s: i8, // DL_小吹っ飛び（7） - ダメージLv7を差し替えるタイプを指定
    pub dmg_lvmin: i8, // DL_極小（8） - ダメージLv8を差し替えるタイプを指定
    pub dmg_lvuppercut: i8, // DL_打ち上げ（9） - ダメージLv9を差し替えるタイプを指定
    pub dmg_lvblow_l_l: i8, // DL_特大吹っ飛び(10) - ダメージLv10を差し替えるタイプを指定
    pub dmg_lvbreath: i8, // DL_ブレス(11) - ダメージLv11を差し替えるタイプを指定
    pub atk_attribute: u8, // 物理属性 - 特殊効果に設定する物理属性
    pub sp_attribute: u8, // 特殊属性 - 特殊効果に設定する特殊属性
    pub state_info: u8, // 状態変化タイプ - 状態変化の判定フラグ
    pub wep_param_change: u8, // 対武器パラメータ変化 - どの武器に対して効果を発揮するかを指定する。制限無しの場合は敵も含めた全ての攻撃・防御が対象
    pub move_type: u8, // 移動タイプ - 移動タイプ。移動速度を変更する。
    pub life_reduction_type: u8, // 防御：寿命減少タイプ - 
    pub throw_condition: u8, // 投げ条件 - 投げ条件。投げマスクに影響する。
    pub add_behavior_judge_id_condition: i8, // 行動判定IDに加算する条件値 - 行動判定ＩＤに値を加算する条件値(Def:-1)
    pub add_behavior_judge_id_add: u8, // 行動判定IDに加算する加算値 - 行動判定IDの加算値 ０の場合は行動を切り替えるのではなく、行動しなくなります。

    pub effect_target_self: bool, // 効果対象：所属　自分 - この判定にチェックが入っている対象のみ効果を発揮する、デフォルトは×
    pub effect_target_friend: bool, // 効果対象：所属　味方 - この判定にチェックが入っている対象のみ効果を発揮する、デフォルトは×
    pub effect_target_enemy: bool, // 効果対象：所属　敵 - この判定にチェックが入っている対象のみ効果を発揮する、デフォルトは×
    pub effect_target_player: bool, // 効果対象：操作　PC - この判定にチェックが入っている対象のみ効果を発揮する、デフォルトは×
    pub effect_target_a_i: bool, // 効果対象：操作　AI - この判定にチェックが入っている対象のみ効果を発揮する、デフォルトは×
    pub effect_target_live: bool, // 効果対象：状態　生存 - この判定にチェックが入っている対象のみ効果を発揮する、デフォルトは×
    pub effect_target_ghost: bool, // 効果対象：状態　全ゴースト - この判定にチェックが入っている対象のみ効果を発揮する、デフォルトは×
    pub effect_target_white_ghost: bool, // 効果対象：状態　白ゴースト - この判定にチェックが入っている対象のみ効果を発揮する、デフォルトは×

    pub effect_target_black_ghost: bool, // 効果対象：状態　黒ゴースト - この判定にチェックが入っている対象のみ効果を発揮する、デフォルトは×
    pub effect_target_attacker: bool, // 効果対象：攻撃者に発動 - ダメージ後に攻撃者に特殊効果を適用（防御側には入れない）
    pub disp_icon_nonactive: bool, // 発動してなくてもアイコン表示 - 発動待ちの状態でもアイコンを表示する。
    pub use_sp_effect_effect: bool, // 特殊効果エフェクトを使用するか - 特殊効果エフェクトを使用するか
    pub b_adjust_magic_ablity: bool, // 魔力補正するか？ - 魔力補正するか？
    pub b_adjust_faith_ablity: bool, // 信仰補正するか？ - 信仰補正するか？
    pub b_game_clear_bonus: bool, // 周回ボーナス用か？ - ゲームクリア周回ボーナス用かどうか。
    pub mag_param_change: bool, // 対魔法パラメータ変化 - 魔法に対して効果を発揮するかしないかを設定する

    pub miracle_param_change: bool, // 対奇跡パラメータ変化 - 奇跡に対して効果を発揮するかしないかを設定する
    pub clear_soul: bool, // 所持ソウルクリアするか - 所持ソウルが0になります。
    pub request_s_o_s: bool, // SOSサイン　判定フラグ - チェックが付いている場合、発動時にSOSサイン要求を発行
    pub request_black_s_o_s: bool, // ブラックSOSサイン　判定フラグ - チェックが付いている場合、発動時にブラックSOSサイン要求を発行
    pub request_force_join_black_s_o_s: bool, // ブラック強制参加SOSサイン　判定フラグ - チェックが付いている場合、発動時にブラック強制参加SOSサイン要求を発行
    pub request_kick_session: bool, // キック　判定フラグ - チェックが付いている場合、発動時にキック要求を発行
    pub request_leave_session: bool, // 退出　判定フラグ - チェックが付いている場合、発動時に退出要求を発行
    pub request_npc_inveda: bool, // NPCへの侵入　判定フラグ - チェックが付いている場合、発動時にNPCへの侵入要求を発行

    pub no_dead: bool, // 成仏不可　判定フラグ - 死体状態になれるかどうか。このチェックが付いていると、死亡状態にならない
    pub b_curr_h_p_independe_max_h_p: bool, // 最大HPが増減しても、現在HPは影響しないか？ - 最大HPが増減しても、現在HPは影響しないか？
    pub corrosion_ignore: bool, // 腐食無視 - 【状態変化タイプ】が【腐食】による【耐久度】減少を無視する
    pub sight_search_cut_ignore: bool, // 視覚索敵カット無視 - 視覚索敵無効を無視する
    pub hearing_search_cut_ignore: bool, // 聴覚索敵カット無視 - 聴覚索敵無効を無視する
    pub anti_magic_ignore: bool, // アンチマジック無効 - アンチマジック範囲でも魔法を使用できる
    pub fake_target_ignore: bool, // 偽ターゲット無効 - 発生した偽ターゲットに引っかからなくなる
    pub fake_target_ignore_undead: bool, // 偽ターゲット無効_不死系 - 発生した不死系の偽ターゲットに引っかからなくなる

    pub fake_target_ignore_animal: bool, // 偽ターゲット無効_獣系 - 発生した獣系の偽ターゲットに引っかからなくなる
    pub grabity_ignore: bool, // グラビティ無効 - グラビティ効果無効
    pub disable_poison: bool, // 毒無効 - この効果がかかっていると毒にかからなくなる
    pub disable_disease: bool, // 疫病無効 - この効果がかかっていると疫病にかからなくなる
    pub disable_blood: bool, // 出血無効 - この効果がかかっていると出血にかからなくなる
    pub disable_curse: bool, // 呪無効 - この効果がかかっていると呪いにかからなくなる
    pub enable_charm: bool, // 魅了有効 - この効果がかかっていると魅了にかかるようになる
    pub enable_life_time: bool, // 寿命延長するか？ - TAEによるフラグ設定時に寿命延長するか？

    pub has_target: bool, // 敵を把握しているか？ - 敵を把握しているか？：[発動条件](邪眼使用者用)
    pub is_fire_damage_cancel: bool, // 解除条件:炎ダメージ - 炎ダメージによる特殊効果の解除を行うか？
    pub is_extend_sp_effect_life: bool, // 寿命延長効果で延長するか？ - 寿命延長効果が掛かっている時に延長対象になるかどうか
    pub request_leave_coliseum_session: bool, // 闘技場退出　判定フラグ - チェックが付いている場合、発動時に闘技場退出要求を発行
    pub pad_2: Integer<u8, packed_bits::Bits::<4>>, // pad - pad

    pub vow_type0: bool, // 誓約0 - 誓約0
    pub vow_type1: bool, // 誓約1 - 誓約1
    pub vow_type2: bool, // 誓約2 - 誓約2
    pub vow_type3: bool, // 誓約3 - 誓約3
    pub vow_type4: bool, // 誓約4 - 誓約4
    pub vow_type5: bool, // 誓約5 - 誓約5
    pub vow_type6: bool, // 誓約6 - 誓約6
    pub vow_type7: bool, // 誓約7 - 誓約7

    pub vow_type8: bool, // 誓約8 - 誓約8
    pub vow_type9: bool, // 誓約9 - 誓約9
    pub vow_type10: bool, // 誓約10 - 誓約10
    pub vow_type11: bool, // 誓約11 - 誓約11
    pub vow_type12: bool, // 誓約12 - 誓約12
    pub vow_type13: bool, // 誓約13 - 誓約13
    pub vow_type14: bool, // 誓約14 - 誓約14
    pub vow_type15: bool, // 誓約15 - 誓約15

    pub pad1: [u8;11], // pad - pad
}
