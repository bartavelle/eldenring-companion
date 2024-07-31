use packed_struct::prelude::*;

#[allow(non_camel_case_types)]
#[derive(PackedStruct, Debug, Clone)]
#[packed_struct(endian = "lsb", bit_numbering = "msb0")]
pub struct SP_EFFECT_PARAM_ST {
    pub icon_id: i32,                        // アイコンID - アイコンID(-1の時は、アイコン必要なし)
    pub condition_hp: f32,                   // 発動条件　残りHP比率[%] - 残りHPが、maxHPの何%になったら発動するかを設定
    pub effect_endurance: f32,               // 効果持続時間　時間[s] - 変化持続時間　/-1で永続 /0で瞬間1回限り
    pub motion_interval: f32,                // 発動間隔[s] - 何秒間隔で発生するのかを設定
    pub max_hp_rate: f32,                    // 最大HP倍率[%] - 最大HPに補正をかける
    pub max_mp_rate: f32,                    // 最大MP倍率[%] - 最大MPに補正をかける
    pub max_stamina_rate: f32,               // 最大スタミナ倍率[%] - 最大SPに補正をかける
    pub slash_damage_cut_rate: f32, // 防御側：斬撃ダメージ倍率 - 斬撃ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
    pub blow_damage_cut_rate: f32, // 防御側：打撃ダメージ倍率 - 打撃ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
    pub thrust_damage_cut_rate: f32, // 防御側：刺突ダメージ倍率 - 刺突ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
    pub neutral_damage_cut_rate: f32, // 防御側：無属性ダメージ倍率 - 無属性ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
    pub magic_damage_cut_rate: f32, // 防御側：魔法ダメージ倍率 - 魔法ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
    pub fire_damage_cut_rate: f32, // 防御側：炎ダメージ倍率 - 炎ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
    pub thunder_damage_cut_rate: f32, // 防御側：電撃ダメージ倍率 - 電撃ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
    pub physics_attack_rate: f32, // 攻撃側：物理ダメージ倍率 - 物理ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
    pub magic_attack_rate: f32, // 攻撃側：魔法ダメージ倍率 - 魔法ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
    pub fire_attack_rate: f32, // 攻撃側：炎ダメージ倍率 - 炎ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
    pub thunder_attack_rate: f32, // 攻撃側：電撃ダメージ倍率 - 電撃ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
    pub physics_attack_power_rate: f32, // 物理攻撃力倍率 - 物理攻撃力に設定した数値をかけます
    pub magic_attack_power_rate: f32, // 魔法攻撃力倍率 - 魔法攻撃力に設定した数値をかけます
    pub fire_attack_power_rate: f32, // 炎攻撃力倍率 - 炎攻撃力に設定した数値をかけます
    pub thunder_attack_power_rate: f32, // 電撃攻撃力倍率 - 電撃攻撃力に設定した数値をかけます
    pub physics_attack_power: i32, // 物理攻撃力[point] - 物理攻撃力に設定した数値を加減算する
    pub magic_attack_power: i32,  // 魔法攻撃力[point] - 魔法攻撃力に設定した数値を加減算する
    pub fire_attack_power: i32,   // 炎攻撃力[point] - 炎攻撃力に設定した数値を加減算する
    pub thunder_attack_power: i32, // 電撃攻撃力[point] - 電撃攻撃力に設定した数値を加減算する
    pub physics_diffence_rate: f32, // 物理防御力倍率 - 物理防御力に設定した数値をかけます
    pub magic_diffence_rate: f32, // 魔法防御力倍率 - 魔法防御力に設定した数値をかけます
    pub fire_diffence_rate: f32,  // 炎防御力倍率 - 炎防御力に設定した数値をかけます
    pub thunder_diffence_rate: f32, // 電撃防御力倍率 - 電撃防御力に設定した数値をかけます
    pub physics_diffence: i32,    // 物理防御力[point] - 物理防御力に設定した数値を加減算する
    pub magic_diffence: i32,      // 魔法防御力[point] - 魔法防御力に設定した数値を加減算する
    pub fire_diffence: i32,       // 炎防御力[point] - 炎防御力に設定した数値を加減算する
    pub thunder_diffence: i32,    // 電撃防御力[point] - 電撃防御力に設定した数値を加減算する
    pub no_guard_damage_rate: f32, // 隙ダメージ倍率 - 隙のときのダメージ倍率を、設定した数値に置き換える（ダメージ側に設定）
    pub vital_spot_change_rate: f32, // スィートスポット倍率 - スィートスポットのダメージ計算を指定した数値に差し替える(急所ダメージ補正) -1.0で無効
    pub normal_spot_change_rate: f32, // ノーマルヒット倍率 - ノーマルヒットのダメージ計算を指定した数値に差し替える  -1.0で無効
    pub look_at_target_pos_offset: f32, // LookAt位置オフセット[m] - 敵がLookAtする際に目標位置をオフセットする。見られる側のしゃがみや騎乗に設定する
    pub behavior_id: i32,               // 行動ID指定枠 - 特殊効果から行動IDを使ってダメージを与える場合に指定-1で無効
    pub change_hp_rate: f32,            // HPダメージ量[%] - 一度の発動で最大HPの何%分を減算（または加算）するかを設定
    pub change_hp_point: i32,           // HPダメージ[point] - 一度の発動で何ポイント減算（または加算）するかを設定
    pub change_mp_rate: f32,            // MPダメージ量[%] - 一度の発動で最大MPの何%分を減算（または加算）するかを設定
    pub change_mp_point: i32,           // MPダメージ[point] - 一度の発動で何ポイント減算（または加算）するかを設定
    pub mp_recover_change_speed: i32, // MP回復速度変化[point] - 回復速度を変化させる。回復計算式の基準回復速度、初期回復速度に加減算する。
    pub change_stamina_rate: f32, // スタミナダメージ量[%] - 一度の発動で最大スタミナの何%分を減算（または加算）するかを設定
    pub change_stamina_point: i32, // スタミナダメージ[point] - 一度の発動で何ポイント減算（または加算）するかを設定
    pub stamina_recover_change_speed: i32, // スタミナ回復速度変化[point] - 回復速度を変化させる。回復計算式の基準回復速度、初期回復速度に加減算する。
    pub magic_effect_time_change: f32, // 魔法効果時間変化 - 効果持続時間に0.1秒以上設定されている魔法のみ、効果持続時間に設定されている時間を加減算する
    pub inside_durability: i32,        // 耐久度変化：内部損耗度[point] - 内部損耗度に数値分を加減算する
    pub max_durability: i32, // 耐久度変化：最大損耗度変化[point] - 耐久度の内部損耗度の最大値に、設定された数値を加算する
    pub stamina_attack_rate: f32, // スタミナ攻撃力倍率 - スタミナ攻撃力に、倍率をかける(1.0 1倍 0.5 半分）
    pub poizon_attack_power: i32, // 毒耐性攻撃力[point] - ヒットした時に、対象の【毒耐性値】に加算する数値
    pub disease_attack_power: i32, // 疫病耐性攻撃力[point] - ヒットした時に、対象の【疫病耐性値】に加算する数値
    pub blood_attack_power: i32, // 出血耐性攻撃力[point] - ヒットした時に、対象の【出血耐性値】に加算する数値
    pub curse_attack_power: i32, // 呪耐性攻撃力[point] - ヒットした時に、対象の【呪耐性値】に加算する数値
    pub fall_damage_rate: f32, // 落下ダメージ倍率 - 落下時のダメージ計算に倍率をかける
    pub soul_rate: f32,      // 取得ソウル倍率 - 敵を倒した時の取得ソウル量が、指定倍数分上乗せされる
    pub equip_weight_change_rate: f32, // 装備重量変化倍率 - 最大装備重量に、設定された倍率をかける
    pub all_item_weight_change_rate: f32, // 所持重量変化倍率 - 最大所持重量に、設定された倍率をかける
    pub soul: i32,           // ソウル加算 - 所持ソウルに、設定値を加算する
    pub anim_id_offset: i32, // アニメIDオフセット(無効-1) - アニメIDオフセット(無効-1)
    pub have_soul_rate: f32, // 所持ソウル率 - 敵周回効果用。設定されているキャラから外にソウルが出て行く時に適用されます。
    pub target_priority: f32, // ターゲット優先度加算分 - マルチプレイ時、敵から優先的にターゲットとして狙われるようになる。プライオリティの加算。０がデフォルト。プラス値でよく狙われるようになる。マイナスは、－１まで。
    pub sight_search_enemy_rate: f32, // 見られる方：視覚倍率 - 見つかりやすさを倍率で補正する
    pub hearing_search_enemy_rate: f32, // 聞かせる方：AI音半径倍率 - 発するAI音の大きさを倍率で補正する
    pub grabity_rate: f32,    // グラビティ率 - グラビティ率
    pub regist_poizon_change_rate: f32, // 毒耐性変化倍率 - 毒耐性値に設定された倍率をかける
    pub regist_disease_change_rate: f32, // 疫病耐性変化倍率 - 疫病耐性値に設定された倍率をかける
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
    pub magic_sub_category_change1: u8, // 対サブカテゴリパラメータ変化1 - 対サブカテゴリパラメータ変化1
    pub magic_sub_category_change2: u8, // 対サブカテゴリパラメータ変化2 - 対サブカテゴリパラメータ変化2
    pub bow_dist_rate: i16,   // 弓飛距離補正[％] - 武器の飛距離補正に加算される補正値
    pub sp_category: u16,     // 特殊効果カテゴリ - 特殊効果の上書きなどの挙動を決めるカテゴリ
    pub category_priority: u8, // カテゴリ内優先度 - 同一カテゴリ内での優先度（低い方が優先）
    pub save_category: i8,    // 保存カテゴリ - 特殊効果を保存するカテゴリ
    pub change_magic_slot: u8, // 魔法登録枠変化　魔法スロット - 魔法登録枠を指定数増やすことが出来る
    pub change_miracle_slot: u8, // 奇跡登録枠変化　奇跡スロット - 軌跡登録枠を指定数増やすことが出来る
    pub hero_point_damage: i8, // 人間性ダメージ値 - 人間性値に与えるダメージ値
    pub def_flick_power: u8,  // はじき防御力_上書き - はじき防御力を上書きする値を設定
    pub flick_damage_cut_rate: u8, // はじき時ダメージ減衰率[%]_上書き - はじき時のダメージ減衰率を上書きする値を設定
    pub blood_damage_rate: u8, // 出血ダメージ補正倍率 - 状態変化タイプ[出血]のPointダメージ、％ダメージの時のみ使用される補正値
    pub dmg_lvnone: i8,        // DL_ダメージなし（0） - ダメージLv0を差し替えるタイプを指定
    pub dmg_lvs: i8,           // DL_小（1） - ダメージLv1を差し替えるタイプを指定
    pub dmg_lvm: i8,           // DL_中（2） - ダメージLv2を差し替えるタイプを指定
    pub dmg_lvl: i8,           // DL_大（3） - ダメージLv3を差し替えるタイプを指定
    pub dmg_lvblow_m: i8,      // DL_吹っ飛び（4） - ダメージLv4を差し替えるタイプを指定
    pub dmg_lvpush: i8,        // DL_プッシュ（5） - ダメージLv5を差し替えるタイプを指定
    pub dmg_lvstrike: i8,      // DL_叩きつけ（6） - ダメージLv6を差し替えるタイプを指定
    pub dmg_lvblow_s: i8,      // DL_小吹っ飛び（7） - ダメージLv7を差し替えるタイプを指定
    pub dmg_lvmin: i8,         // DL_極小（8） - ダメージLv8を差し替えるタイプを指定
    pub dmg_lvuppercut: i8,    // DL_打ち上げ（9） - ダメージLv9を差し替えるタイプを指定
    pub dmg_lvblow_l_l: i8,    // DL_特大吹っ飛び(10) - ダメージLv10を差し替えるタイプを指定
    pub dmg_lvbreath: i8,      // DL_ブレス(11) - ダメージLv11を差し替えるタイプを指定
    pub atk_attribute: u8,     // 物理属性 - 特殊効果に設定する物理属性
    pub sp_attribute: u8,      // 特殊属性 - 特殊効果に設定する特殊属性
    pub state_info: u16,       // 状態変化タイプ - 状態変化の判定フラグ
    pub wep_param_change: u8, // 対武器パラメータ変化 - どの武器に対して効果を発揮するかを指定する。制限無しの場合は敵も含めた全ての攻撃・防御が対象
    pub move_type: u8,        // 移動タイプ - 移動タイプ。移動速度を変更する。
    pub life_reduction_type: u16, // 防御：寿命減少タイプ -
    pub throw_condition: u8,  // 投げ条件 - 投げ条件。投げマスクに影響する。
    pub add_behavior_judge_id_condition: i8, // 行動判定IDに加算する条件値 - 行動判定ＩＤに値を加算する条件値(Def:-1)
    pub freeze_damage_rate: u8, // 冷気ダメージ補正倍率 - 状態変化タイプ[冷気]のPointダメージ、％ダメージの時のみ使用される補正値

    pub effect_target_self: bool, // 効果対象：所属　自分 - この判定にチェックが入っている対象のみ効果を発揮する、デフォルトは×
    pub effect_target_friend: bool, // 効果対象：所属　味方 - この判定にチェックが入っている対象のみ効果を発揮する、デフォルトは×
    pub effect_target_enemy: bool, // 効果対象：所属　敵 - この判定にチェックが入っている対象のみ効果を発揮する、デフォルトは×
    pub effect_target_player: bool, // 効果対象：操作　PC - この判定にチェックが入っている対象のみ効果を発揮する、デフォルトは×
    pub effect_target_a_i: bool, // 効果対象：操作　AI - この判定にチェックが入っている対象のみ効果を発揮する、デフォルトは×
    pub effect_target_live: bool, // 効果対象：状態　生存 - この判定にチェックが入っている対象のみ効果を発揮する、デフォルトは×
    pub effect_target_ghost: bool, // 効果対象：状態　全ゴースト - この判定にチェックが入っている対象のみ効果を発揮する、デフォルトは×
    pub disable_sleep: bool,       // 睡眠無効 - この効果がかかっていると睡眠にかからなくなる

    pub disable_madness: bool,        // 発狂無効 - この効果がかかっていると発狂にかからなくなる
    pub effect_target_attacker: bool, // 効果対象：攻撃者に発動 - ダメージ後に攻撃者に特殊効果を適用（防御側には入れない）
    pub disp_icon_nonactive: bool,    // 発動してなくてもアイコン表示 - 発動待ちの状態でもアイコンを表示する。
    pub regain_gauge_damage: bool,    // リゲインゲージを発生させるか - リゲインゲージを発生させるか
    pub b_adjust_magic_ablity: bool,  // 魔力補正するか？ - 魔力補正するか？
    pub b_adjust_faith_ablity: bool,  // 信仰補正するか？ - 信仰補正するか？
    pub b_game_clear_bonus: bool,     // 周回ボーナス用か？ - ゲームクリア周回ボーナス用かどうか。
    pub mag_param_change: bool,       // 対魔法パラメータ変化 - 魔法に対して効果を発揮するかしないかを設定する

    pub miracle_param_change: bool, // 対奇跡パラメータ変化 - 奇跡に対して効果を発揮するかしないかを設定する
    pub clear_soul: bool,           // 所持ソウルクリアするか - 所持ソウルが0になります。
    pub request_s_o_s: bool,        // SOSサイン　判定フラグ - チェックが付いている場合、発動時にSOSサイン要求を発行
    pub request_black_s_o_s: bool, // ブラックSOSサイン　判定フラグ - チェックが付いている場合、発動時にブラックSOSサイン要求を発行
    pub request_force_join_black_s_o_s: bool, // 侵入_Aリクエスト　判定フラグ - チェックが付いている場合、発動時に侵入_Aリクエストを発行
    pub request_kick_session: bool,           // キック　判定フラグ - チェックが付いている場合、発動時にキック要求を発行
    pub request_leave_session: bool,          // 退出　判定フラグ - チェックが付いている場合、発動時に退出要求を発行
    pub request_npc_inveda: bool, // NPCへの侵入　判定フラグ - チェックが付いている場合、発動時にNPCへの侵入要求を発行

    pub no_dead: bool, // 成仏不可　判定フラグ - 死体状態になれるかどうか。このチェックが付いていると、死亡状態にならない
    pub b_curr_h_p_independe_max_h_p: bool, // 最大HPが増減しても、現在HPは影響しないか？ - 最大HPが増減しても、現在HPは影響しないか？
    pub corrosion_ignore: bool,             // 腐食無視 - 【状態変化タイプ】が【腐食】による【耐久度】減少を無視する
    pub sight_search_cut_ignore: bool,      // 視覚索敵カット無視 - 視覚索敵無効を無視する
    pub hearing_search_cut_ignore: bool,    // 聴覚索敵カット無視 - 聴覚索敵無効を無視する
    pub anti_magic_ignore: bool,            // アンチマジック無効 - アンチマジック範囲でも魔法を使用できる
    pub fake_target_ignore: bool,           // 偽ターゲット無効_幻聴系 - 発生した偽ターゲットに引っかからなくなる
    pub fake_target_ignore_undead: bool,    // 偽ターゲット無効_人系 - 発生した人系の偽ターゲットに引っかからなくなる

    pub fake_target_ignore_animal: bool, // 偽ターゲット無効_獣系 - 発生した獣系の偽ターゲットに引っかからなくなる
    pub grabity_ignore: bool,            // グラビティ無効 - グラビティ効果無効
    pub disable_poison: bool,            // 毒無効 - この効果がかかっていると毒にかからなくなる
    pub disable_disease: bool,           // 疫病無効 - この効果がかかっていると疫病にかからなくなる
    pub disable_blood: bool,             // 出血無効 - この効果がかかっていると出血にかからなくなる
    pub disable_curse: bool,             // 呪無効 - この効果がかかっていると呪いにかからなくなる
    pub enable_charm: bool,              // 魅了有効 - この効果がかかっていると魅了にかかるようになる
    pub enable_life_time: bool,          // 寿命延長するか？ - TAEによるフラグ設定時に寿命延長するか？

    pub b_adjust_strength_ablity: bool,       // 筋力補正するか？ - 筋力補正するか？
    pub b_adjust_agility_ablity: bool,        // 技量補正するか？ - 技量補正するか？
    pub erase_on_bonfire_recover: bool,       // 篝火回復で消えるか - 篝火回復で消えるか
    pub throw_attack_param_change: bool, // 対投げパラメータ変化 - 投げ攻撃に対して効果を発揮するかしないかを設定する
    pub request_leave_coliseum_session: bool, // 闘技場退出　判定フラグ - チェックが付いている場合、発動時に闘技場退出要求を発行
    pub is_extend_sp_effect_life: bool, // 寿命延長効果で延長するか？ - 寿命延長効果が掛かっている時に延長対象になるかどうか
    pub has_target: bool,               // 敵を把握しているか？ - 敵を把握しているか？：[発動条件](邪眼使用者用)
    pub replanning_on_fire: bool,       // 発動時リプランニングするか？ - 発動時リプランニングするか

    pub vow_type0: bool, // 誓約0 - 誓約0
    pub vow_type1: bool, // 誓約1 - 誓約1
    pub vow_type2: bool, // 誓約2 - 誓約2
    pub vow_type3: bool, // 誓約3 - 誓約3
    pub vow_type4: bool, // 誓約4 - 誓約4
    pub vow_type5: bool, // 誓約5 - 誓約5
    pub vow_type6: bool, // 誓約6 - 誓約6
    pub vow_type7: bool, // 誓約7 - 誓約7

    pub vow_type8: bool,  // 誓約8 - 誓約8
    pub vow_type9: bool,  // 誓約9 - 誓約9
    pub vow_type10: bool, // 誓約10 - 誓約10
    pub vow_type11: bool, // 誓約11 - 誓約11
    pub vow_type12: bool, // 誓約12 - 誓約12
    pub vow_type13: bool, // 誓約13 - 誓約13
    pub vow_type14: bool, // 誓約14 - 誓約14
    pub vow_type15: bool, // 誓約15 - 誓約15

    pub rep_atk_dmg_lv: i8, // 攻撃側ダメージレベル差し替え - 攻撃側のダメージレベルがこの値に指し換わる
    pub sight_search_rate: f32, // 見る方：視覚倍率 - 見つけやすさを倍率で補正する

    pub effect_target_oppose_target: bool, // 効果対象：●敵対 - この判定にチェックが入っている対象のみ効果を発揮する、デフォルトは×
    pub effect_target_friendly_target: bool, // 効果対象：○味方 - この判定にチェックが入っている対象のみ効果を発揮する、デフォルトは×
    pub effect_target_self_target: bool, // 効果対象：自分 - この判定にチェックが入っている対象のみ効果を発揮する、デフォルトは×
    pub effect_target_pc_horse: bool, // 効果対象：PC馬 - この判定にチェックが入っている対象のみ効果を発揮する、デフォルトは×
    pub effect_target_pc_deceased: bool, // 効果対象：PC亡者のみ - この判定にチェックが入っている対象のみ効果を発揮する、デフォルトは×
    pub is_contract_sp_effect_life: bool, // 寿命短縮効果で短縮するか？ - 寿命短縮効果が掛かっている時に短縮対象になるかどうか
    pub is_wait_mode_delete: bool,        // 待ち状態に入ると削除 - 待ち状態になった瞬間に削除するか？
    pub is_ignore_no_damage: bool, // 無敵時でも発動するか - 状態変化タイプ「無敵時でも発動機能を適応」が掛かっているときのみ、無敵状態でもこの特殊効果からのダメージを適応するか

    pub change_team_type: i8,        // チームタイプ変更 - 指定したチームタイプに上書きする
    pub dmypoly_id: i16,             // ダミポリID - ダミポリID。ダミポリID範囲は0～999.1000,10000の位はカテゴリ番号.
    pub vfx_id: i32,                 // 特殊効果VfxId_０ - 特殊効果VfxId(-1無効)
    pub accumu_over_fire_id: i32,    // 元気玉上限時発動特殊効果Id - 元気玉上限時発動特殊効果Id
    pub accumu_over_val: i32,        // 元気玉上限値 - 元気玉上限値
    pub accumu_under_fire_id: i32,   // 元気玉下限時発動特殊効果Id - 元気玉下限時発動特殊効果Id
    pub accumu_under_val: i32,       // 元気玉下限値 - 元気玉下限値
    pub accumu_val: i32,             // 元気玉蓄積値 - 元気玉蓄積値
    pub eye_ang_x: u8,               // 見る方：視覚角度（高さ）上書き[deg] - 見つけやすさの角度を上書きする
    pub eye_ang_y: u8,               // 見る方：視覚角度（幅）上書き[deg] - 見つけやすさの角度を上書きする
    pub add_deceased_lv: i16,        // 亡者度 変更 - この値分亡者度を加算する
    pub vfx_id1: i32,                // 特殊効果VfxId_１ - 特殊効果VfxId１(-1無効)
    pub vfx_id2: i32,                // 特殊効果VfxId_２ - 特殊効果VfxId２(-1無効)
    pub vfx_id3: i32,                // 特殊効果VfxId_３ - 特殊効果VfxId３(-1無効)
    pub vfx_id4: i32,                // 特殊効果VfxId_４ - 特殊効果VfxId４(-1無効)
    pub vfx_id5: i32,                // 特殊効果VfxId_５ - 特殊効果VfxId５(-1無効)
    pub vfx_id6: i32,                // 特殊効果VfxId_６ - 特殊効果VfxId６(-1無効)
    pub vfx_id7: i32,                // 特殊効果VfxId_７ - 特殊効果VfxId７(-1無効)
    pub freeze_attack_power: i32,    // 冷気耐性攻撃力[point] - ヒットした時に、対象の【冷気耐性値】に加算する数値
    pub appear_ai_sound_id: i32,     // 発生AI音ID - 設定された値のAI音パラメータを発生させる
    pub add_foot_effect_sfx_id: i16, // 追加フットエフェクト識別子 - 特殊効果時に追加で発生させるフットエフェクトの識別子。XYYZZZのZZZ
    pub dexterity_cancel_system_only_add_dexterity: i8, // 技量キャンセル用仮想ステータス - 「技量キャンセル」のTAEフラグの終了タイミングを計算する時に、この値を追加して計算する
    pub team_offense_effectivity: i8, // チーム攻撃影響力_上書き - 対象の【チーム攻撃影響力】の値を、上書きして変更する。デフォルト値（-1）のときは変更しない。
    pub toughness_damage_cut_rate: f32, // 強靭度 被ダメージ倍率 - 強靭度版カット率
    pub weak_dmg_rate_a: f32,         // 特攻Aダメージ倍率補正 - 特攻Aダメージ倍率に補正をかけます。１が通常。
    pub weak_dmg_rate_b: f32,         // 特攻Bダメージ倍率補正 - 特攻Bダメージ倍率に補正をかけます。１が通常。
    pub weak_dmg_rate_c: f32,         // 特攻Cダメージ倍率補正 - 特攻Cダメージ倍率に補正をかけます。１が通常。
    pub weak_dmg_rate_d: f32,         // 特攻Dダメージ倍率補正 - 特攻Dダメージ倍率に補正をかけます。１が通常。
    pub weak_dmg_rate_e: f32,         // 特攻Eダメージ倍率補正 - 特攻Eダメージ倍率に補正をかけます。１が通常。
    pub weak_dmg_rate_f: f32,         // 特攻Fダメージ倍率補正 - 特攻Fダメージ倍率に補正をかけます。１が通常。
    pub dark_damage_cut_rate: f32, // 防御側：闇ダメージ倍率 - 闇ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
    pub dark_diffence_rate: f32,   // 闇防御力倍率 - 闇防御力に設定した数値をかけます
    pub dark_diffence: i32,        // 闇防御力[point] - 闇防御力に設定した数値を加減算する
    pub dark_attack_rate: f32, // 攻撃側：闇ダメージ倍率 - 闇ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
    pub dark_attack_power_rate: f32, // 闇攻撃力倍率 - 闇攻撃力に設定した数値をかけます
    pub dark_attack_power: i32, // 闇攻撃力[point] - 闇攻撃力に設定した数値を加減算する
    pub anti_dark_sight_radius: f32, // 暗闇丸見え半径[m] - 暗闇丸見え半径[m]。この距離内にいる場合は暗所でも通常距離で見えるようになります
    pub anti_dark_sight_dmypoly_id: i32, // 暗闇丸見えダミポリID - 暗闇丸見えダミポリID(-1:マスター)。このダミポリを中心に丸見え領域を作成します
    pub condition_hp_rate: f32, // 発動条件　残りHP比率が一定以上[%] - 指定された値以上のHPを持っている時にしか発動しない
    pub consume_stamina_rate: f32, // 消費スタミナ倍率 - 行動パラメータの消費スタミナの値にかける倍率
    pub item_drop_rate: f32,    // アイテムドロップ補正 - 設定された値が【アイテムドロップ補正】に加算される
    pub change_poison_resist_point: i32, // 毒耐性変化[point] - 状態耐性値を増減させる
    pub change_disease_resist_point: i32, // 疫病耐性変化[point] - 状態耐性値を増減させる
    pub change_blood_resist_point: i32, // 出血耐性変化[point] - 状態耐性値を増減させる
    pub change_curse_resist_point: i32, // 呪耐性変化[point] - 状態耐性値を増減させる
    pub change_freeze_resist_point: i32, // 冷気耐性変化[point] - 状態耐性値を増減させる
    pub slash_attack_rate: f32, // 攻撃側：斬撃ダメージ倍率 - 斬撃ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
    pub blow_attack_rate: f32, // 攻撃側：打撃ダメージ倍率 - 打撃ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
    pub thrust_attack_rate: f32, // 攻撃側：刺突ダメージ倍率 - 刺突ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
    pub neutral_attack_rate: f32, // 攻撃側：無属性ダメージ倍率 - 無属性ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
    pub slash_attack_power_rate: f32, // 斬撃攻撃力倍率 - 斬撃攻撃力に設定した数値をかけます
    pub blow_attack_power_rate: f32, // 打撃攻撃力倍率 - 打撃攻撃力に設定した数値をかけます
    pub thrust_attack_power_rate: f32, // 刺突攻撃力倍率 - 刺突攻撃力に設定した数値をかけます
    pub neutral_attack_power_rate: f32, // 無属性攻撃力倍率 - 無属性攻撃力に設定した数値をかけます
    pub slash_attack_power: i32,  // 斬撃攻撃力[point] - 斬撃攻撃力に設定した数値を加減算する
    pub blow_attack_power: i32,   // 打撃攻撃力[point] - 打撃攻撃力に設定した数値を加減算する
    pub thrust_attack_power: i32, // 刺突攻撃力[point] - 刺突攻撃力に設定した数値を加減算する
    pub neutral_attack_power: i32, // 無属性攻撃力[point] - 無属性攻撃力に設定した数値を加減算する
    pub change_strength_point: i32, // 筋力補正変化[point] - 武器の補正値を加減算する
    pub change_agility_point: i32, // 俊敏補正変化[point] - 武器の補正値を加減算する
    pub change_magic_point: i32,  // 魔力補正変化[point] - 武器の補正値を加減算する
    pub change_faith_point: i32,  // 信仰補正変化[point] - 武器の補正値を加減算する
    pub change_luck_point: i32,   // 運補正変化[point] - 武器の補正値を加減算する
    pub recover_arts_pointstr: i8, // アーツポイント回復 筋力系 - アーツポイント筋力を回復させる
    pub recover_arts_pointdex: i8, // アーツポイント回復 技量系 - アーツポイント技量を回復させる
    pub recover_arts_pointmagic: i8, // アーツポイント回復 魔法系 - アーツポイント魔法を回復させる
    pub recover_arts_pointmiracle: i8, // アーツポイント回復 奇跡系 - アーツポイント奇跡を回復させる
    pub madness_damage_rate: u8, // 発狂ダメージ補正倍率 - 状態変化タイプ[発狂]のPointダメージ、％ダメージの時のみ使用される補正値

    pub is_use_status_ailment_atk_power_correct: bool, // 状態異常攻撃力倍率補正を適応するか - ○なら攻撃パラの状態異常攻撃力倍率補正を適応します。
    pub is_use_atk_param_atk_power_correct: bool, // 攻撃パラメータの攻撃力倍率補正を適応するか - ○なら攻撃パラの攻撃力倍率補正を適応します。
    pub dont_delete_on_dead: bool, // 死亡時に削除しない - ○ならキャラが死亡しても削除しません。主に死亡エフェクトに使います。
    pub disable_freeze: bool,      // 冷気無効 - この効果がかかっていると冷気にかからなくなる
    pub is_disable_net_sync: bool, // ネット同期しない - ネット同期しない。ローカルに掛けるようになる、という意味ではなく、単にネット送信しない。例えばリモートキャラはローカル発動しないので、その場合何も起こらない。
    pub shaman_param_change: bool, // 対呪術パラメータ変化 - 呪術に対して効果を発揮するかしないかを設定する
    pub is_stop_searched_notify: bool, // 被索敵状態の通知停止 - 自軍をターゲットしている通知を停止するかどうか(EventMakerでの判定やバディ小隊で使用)
    pub is_check_above_shadow_test: bool, // 雨遮蔽外の時のみかかる - ○なら遮蔽判定されているときは掛からない（×は常に掛かる）

    pub add_behavior_judge_id_add: u16, // 行動判定IDに加算する加算値 - 行動判定IDの加算値 ０の場合は行動を切り替えるのではなく、行動しなくなります。
    pub sa_receive_damage_rate: f32,    // SA値_被ダメージ倍率 - SAダメージかかる倍率
    pub def_player_dmg_correct_ratephysics: f32, // 防御側 プレイヤー 物理ダメージ補正倍率 - プレイヤーから受けるダメージ値に対するダメージ補正。
    pub def_player_dmg_correct_ratemagic: f32, // 防御側 プレイヤー 魔法ダメージ補正倍率 - プレイヤーから受けるダメージ値に対するダメージ補正。
    pub def_player_dmg_correct_ratefire: f32, // 防御側 プレイヤー 炎ダメージ補正倍率 - プレイヤーから受けるダメージ値に対するダメージ補正。
    pub def_player_dmg_correct_ratethunder: f32, // 防御側 プレイヤー 雷ダメージ補正倍率 - プレイヤーから受けるダメージ値に対するダメージ補正。
    pub def_player_dmg_correct_ratedark: f32, // 防御側 プレイヤー 闇ダメージ補正倍率 - プレイヤーから受けるダメージ値に対するダメージ補正。
    pub def_enemy_dmg_correct_ratephysics: f32, // 防御側 敵 物理ダメージ補正倍率 - 敵から受けるダメージ値に対するダメージ補正。
    pub def_enemy_dmg_correct_ratemagic: f32, // 防御側 敵 魔法ダメージ補正倍率 - 敵から受けるダメージ値に対するダメージ補正。
    pub def_enemy_dmg_correct_ratefire: f32, // 防御側 敵 炎ダメージ補正倍率 - 敵から受けるダメージ値に対するダメージ補正。
    pub def_enemy_dmg_correct_ratethunder: f32, // 防御側 敵 雷ダメージ補正倍率 - 敵から受けるダメージ値に対するダメージ補正。
    pub def_enemy_dmg_correct_ratedark: f32, // 防御側 敵 闇ダメージ補正倍率 - 敵から受けるダメージ値に対するダメージ補正。
    pub def_obj_dmg_correct_rate: f32, // 防御側 オブジェクトダメージ補正倍率 - OBJから受けるダメージ値に対するダメージ補正。
    pub atk_player_dmg_correct_ratephysics: f32, // 攻撃側 プレイヤー 物理ダメージ補正倍率 - プレイヤーに与えるダメージ値に対するダメージ補正。
    pub atk_player_dmg_correct_ratemagic: f32, // 攻撃側 プレイヤー 魔法ダメージ補正倍率 - プレイヤーに与えるダメージ値に対するダメージ補正。
    pub atk_player_dmg_correct_ratefire: f32, // 攻撃側 プレイヤー 炎ダメージ補正倍率 - プレイヤーに与えるダメージ値に対するダメージ補正。
    pub atk_player_dmg_correct_ratethunder: f32, // 攻撃側 プレイヤー 雷ダメージ補正倍率 - プレイヤーに与えるダメージ値に対するダメージ補正。
    pub atk_player_dmg_correct_ratedark: f32, // 攻撃側 プレイヤー 闇ダメージ補正倍率 - プレイヤーに与えるダメージ値に対するダメージ補正。
    pub atk_enemy_dmg_correct_ratephysics: f32, // 攻撃側 敵 物理ダメージ補正倍率 - 敵に与えるダメージ値に対するダメージ補正。
    pub atk_enemy_dmg_correct_ratemagic: f32, // 攻撃側 敵 魔法ダメージ補正倍率 - 敵に与えるダメージ値に対するダメージ補正。
    pub atk_enemy_dmg_correct_ratefire: f32, // 攻撃側 敵 炎ダメージ補正倍率 - 敵に与えるダメージ値に対するダメージ補正。
    pub atk_enemy_dmg_correct_ratethunder: f32, // 攻撃側 敵 雷ダメージ補正倍率 - 敵に与えるダメージ値に対するダメージ補正。
    pub atk_enemy_dmg_correct_ratedark: f32, // 攻撃側 敵 闇ダメージ補正倍率 - 敵に与えるダメージ値に対するダメージ補正。
    pub regist_freeze_change_rate: f32,      // 冷気耐性変化倍率 - 冷気耐性値に設定された倍率をかける
    pub invocation_conditions_state_change1: u16, // 発動条件状態変化タイプ1 - 発動条件状態変化タイプ1
    pub invocation_conditions_state_change2: u16, // 発動条件状態変化タイプ2 - 発動条件状態変化タイプ2
    pub invocation_conditions_state_change3: u16, // 発動条件状態変化タイプ3 - 発動条件状態変化タイプ3
    pub hearing_ai_sound_level: i16,         // 聞く方：可聴AI音レベル上書き - どれくらい耳が良いのかを上書きする
    pub chr_proxy_height_rate: f32,          // カプセルサイズ倍率 - キャラカプセルの高さに掛かる倍率
    pub add_aware_point_correct_value_for_me: f32, // 索敵度加算補正_見る側 - 索敵度加算補正_見る側
    pub add_aware_point_correct_value_for_target: f32, // 索敵度加算補正_見られる側 - 索敵度加算補正_見られる側
    pub sight_search_enemy_add: f32,         // 見られる方：視覚加算 - 見つかりやすさを実数で補正する
    pub sight_search_add: f32,               // 見る方：視覚加算 - 見つけやすさを実数で補正する
    pub hearing_search_add: f32,             // 聞く方：AI音半径加算 - AI音の聞こえ具合を実数で補正する
    pub hearing_search_rate: f32,            // 聞く方：AI音半径倍率 - AI音の聞こえ具合を倍率で補正する
    pub hearing_search_enemy_add: f32,       // 聞かせる方：AI音半径加算 - 発するAI音の大きさを実数で補正する
    pub valuemagnification: f32,             // 販売価格補正：倍率 - 販売価格補正：倍率
    pub arts_consumption_rate: f32,          // アーツ消費MP倍率 - アーツ消費MP倍率[%]
    pub magic_consumption_rate: f32,         // 魔法消費MP倍率 - 魔法消費MP倍率[%]
    pub shaman_consumption_rate: f32,        // 呪術消費MP倍率 - 呪術消費MP倍率[%]
    pub miracle_consumption_rate: f32,       // 奇跡消費MP倍率 - 奇跡消費MP倍率[%]
    pub change_hp_estus_flask_rate: i32, // エスト瓶HPダメージ量[%] - 一度の発動で最大HPの何%分を加算（または減算）するかを設定
    pub change_hp_estus_flask_point: i32, // エスト瓶HPダメージ量[point] - 一度の発動で何ポイント加算（または減算）するかを設定
    pub change_mp_estus_flask_rate: i32, // エスト瓶MPダメージ量[%]  - 一度の発動で最大MPの何%分を加算（または減算）するかを設定
    pub change_mp_estus_flask_point: i32, // エスト瓶MPダメージ量[point]  - 一度の発動で何ポイント加算（または減算）するかを設定
    pub change_hp_estus_flask_correct_rate: f32, // エスト瓶HPダメージ倍率  - HPエスト瓶のダメージ量に対して補正をかける
    pub change_mp_estus_flask_correct_rate: f32, // エスト瓶MPダメージ倍率  - MPエスト瓶のダメージ量に対して補正をかける
    pub apply_id_on_get_soul: i32, // HPドレイン発動特殊効果 - 状態変化タイプ「HPドレイン」の特殊効果が有効の時に、敵を倒した際に同じ特殊効果の「HPドレイン発動特殊効果」に設定されている特殊効果IDを呼び出す(0：無視)
    pub extend_life_rate: f32,     // 寿命延長倍率 - 状態変化タイプ「寿命延長」の延長係数
    pub contract_life_rate: f32,   // 寿命短縮倍率 - 状態変化タイプ「寿命短縮」の短縮係数
    pub def_object_attack_power_rate: f32, // 被ダメージ オブジェクト攻撃力倍率 - OBJから受けるダメージに対して攻撃力を補正する。（ダメージ補正ではない）
    pub effect_end_delete_decal_group_id: i16, // 特殊効果消失時にキャラのペイントデカールを削除するグループID - 特殊効果が消失した時（寿命/何かに上書きされる/消される…など）に、同じグループIDの特殊効果がかかっていなければペイントデカールを削除する。
    pub add_life_force_status: i8,             // 生命力追加値 - 成長ステータスに値を加える
    pub add_willpower_status: i8,              // 精神力追加値 - 成長ステータスに値を加える
    pub add_endure_status: i8,                 // 持久力追加値 - 成長ステータスに値を加える
    pub add_vitality_status: i8,               // 体力追加値 - 成長ステータスに値を加える
    pub add_strength_status: i8,               // 筋力追加値 - 成長ステータスに値を加える
    pub add_dexterity_status: i8,              // 技量追加値 - 成長ステータスに値を加える
    pub add_magic_status: i8,                  // 理力追加値 - 成長ステータスに値を加える
    pub add_faith_status: i8,                  // 信仰追加値 - 成長ステータスに値を加える
    pub add_luck_status: i8,                   // 運追加値 - 成長ステータスに値を加える
    pub delete_criteria_damage: u8,            // 削除条件ダメージ - 特殊効果を削除する条件のダメージ理由
    pub magic_sub_category_change3: u8,        // 対サブカテゴリパラメータ変化3 - 対サブカテゴリパラメータ変化3
    pub sp_attribute_variation_value: u8, // 特殊属性バリエーション値 - 特殊効果に設定する特殊属性と組み合わせて状態異常SFX,SEなどにバリエーションを持たせるために使用する値です。SEQ16473
    pub atk_flick_power: u8,              // はじき攻撃力_上書き - はじき攻撃力を上書きする値を設定
    pub wet_condition_depth: u8, // 濡れる条件の水位設定 - TimeAct「どの水位で濡れるか」と組み合わせて特殊効果に掛かるかどうかを判定する
    pub change_sa_recovery_velocity: f32, // SA回復速度変化 - SA耐久度の回復速度を変化させる
    pub regain_rate: f32,        // リゲイン倍率 - リゲイン倍率
    pub sa_attack_power_rate: f32, // SA攻撃力倍率 - SA攻撃力倍率
    pub sleep_attack_power: i32, // 睡眠耐性攻撃力[point] - ヒットした時に、対象の【睡眠耐性値】に加算する数値
    pub madness_attack_power: i32, // 発狂耐性攻撃力[point] - ヒットした時に、対象の【発狂耐性値】に加算する数値
    pub regist_sleep_change_rate: f32, // 睡眠耐性変化倍率 - 睡眠耐性値に設定された倍率をかける
    pub regist_madness_change_rate: f32, // 発狂耐性変化倍率 - 発狂耐性値に設定された倍率をかける
    pub change_sleep_resist_point: i32, // 睡眠耐性変化[point] - 状態耐性値を増減させる
    pub change_madness_resist_point: i32, // 発狂耐性変化[point] - 状態耐性値を増減させる
    pub sleep_damage_rate: u8, // 睡眠ダメージ補正倍率 - 状態変化タイプ[睡眠]のPointダメージ、％ダメージの時のみ使用される補正値
    pub apply_parts_group: u8, // 対部位パラメータ変化 - 攻撃がヒットした部位によって効果を制限する。ダメージ計算の防御系の項目のみ制限対象となる

    pub clear_target: bool, // ターゲットクリア - 特殊効果が掛かっている間ターゲットを認識しない（騎乗ターゲット除く
    pub fake_target_ignore_ajin: bool, // 偽ターゲット無効_亜人系 - 発生した亜人系の偽ターゲットに引っかからなくなる
    pub fake_target_ignore_mirage_arts: bool, // 偽ターゲット無効_幻影アーツ系 - 発生した幻影アーツ系の偽ターゲットに引っかからなくなる
    pub request_force_join_black_s_o_sb: bool, // 侵入_Bリクエスト　判定フラグ - チェックが付いている場合、発動時に侵入_Bリクエストを発行
    pub unk353_4: bool,                        //  -
    pub padb: Integer<u8, packed_bits::Bits<3>>, // パディング -

    pub pad2: [u8; 1],                           // pad -
    pub change_super_armor_point: f32,           // 最大SA加算値[point] - スーパーアーマー値に加算する値
    pub change_sa_point: f32, // SAダメージ量[point] - 一度の発動で何ポイント減算（または加算）するかを設定
    pub huge_enemy_pickup_height_overwrite: f32, // 巨大敵持ち上げ高さ上書き[m] - 巨大敵持ち上げ高さ上書き[m]
    pub poison_def_damage_rate: f32, // 防御側：毒耐性ダメージ倍率 - 毒耐性ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
    pub disease_def_damage_rate: f32, // 防御側：疫病耐性ダメージ倍率 - 疫病耐性ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
    pub blood_def_damage_rate: f32, // 防御側：出血耐性ダメージ倍率 - 出血耐性ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
    pub curse_def_damage_rate: f32, // 防御側：呪耐性ダメージ倍率 - 呪耐性ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
    pub freeze_def_damage_rate: f32, // 防御側：冷気耐性ダメージ倍率 - 冷気耐性ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
    pub sleep_def_damage_rate: f32, // 防御側：睡眠耐性ダメージ倍率 - 睡眠耐性ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
    pub madness_def_damage_rate: f32, // 防御側：発狂耐性ダメージ倍率 - 発狂耐性ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
    pub overwrite_max_backhome_dist: u16, // 何があっても帰宅する距離[m]_上書き - 何があっても帰宅する距離[m]_上書き
    pub overwrite_backhome_dist: u16, // 戦闘しつつ帰宅する距離[m]_上書き - 戦闘しつつ帰宅する距離[m]_上書き
    pub overwrite_backhome_battle_dist: u16, // 巣に帰るのをあきらめて戦闘する距離[m]_上書き - 巣に帰るのをあきらめて戦闘する距離[m]_上書き
    pub overwriteback_homelook_target_dist: u16, // 帰宅時：ターゲットを見ている距離[m]_上書き - 帰宅時：ターゲットを見ている距離[m]_上書き
    pub goods_consumption_rate: f32,             // アイテム消費MP倍率 - アイテム消費MP倍率
    pub pad3: [u8; 8],                           // pad -
}
