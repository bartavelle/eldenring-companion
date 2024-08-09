use packed_struct::prelude::*;

#[allow(non_camel_case_types)]
#[derive(PackedStruct, Debug, Clone)]
#[packed_struct(endian="lsb", bit_numbering="msb0")]
pub struct CACL_CORRECT_GRAPH_ST {
// only support unicode
    pub stage_max_val0: f32, // 閾値ポイント0 - 仕様書に「n次閾値[point]」と書いてあるもの
    pub stage_max_val1: f32, // 閾値ポイント1 - 仕様書に「n次閾値[point]」と書いてあるもの
    pub stage_max_val2: f32, // 閾値ポイント2 - 仕様書に「n次閾値[point]」と書いてあるもの
    pub stage_max_val3: f32, // 閾値ポイント3 - 仕様書に「n次閾値[point]」と書いてあるもの
    pub stage_max_val4: f32, // 閾値ポイント4 - 仕様書に「n次閾値[point]」と書いてあるもの
    pub stage_max_grow_val0: f32, // 閾値係数0 - 仕様書に「n次閾値[係数]」と書いてあるもの
    pub stage_max_grow_val1: f32, // 閾値係数1 - 仕様書に「n次閾値[係数]」と書いてあるもの
    pub stage_max_grow_val2: f32, // 閾値係数2 - 仕様書に「n次閾値[係数]」と書いてあるもの
    pub stage_max_grow_val3: f32, // 閾値係数3 - 仕様書に「n次閾値[係数]」と書いてあるもの
    pub stage_max_grow_val4: f32, // 閾値係数4 - 仕様書に「n次閾値[係数]」と書いてあるもの
    pub adj_pt_max_grow_val0: f32, // 調整係数0 - 調整係数
    pub adj_pt_max_grow_val1: f32, // 調整係数1 - 調整係数
    pub adj_pt_max_grow_val2: f32, // 調整係数2 - 調整係数
    pub adj_pt_max_grow_val3: f32, // 調整係数3 - 調整係数
    pub adj_pt_max_grow_val4: f32, // 調整係数4 - 調整係数
    pub init_inclination_soul: f32, // 成長ソウル 初期のグラフの傾きα1 - 成長ソウル 初期のグラフの傾きα1
    pub adjustment_value: f32, // 成長ソウル 初期のsoul調整α2 - 成長ソウル 初期のsoul調整α2
    pub boundry_inclination_soul: f32, // 成長ソウル 閾値後のグラフの傾きに影響α3 - 成長ソウル 閾値後のグラフの傾きに影響α3
    pub boundry_value: f32, // 成長ソウル 閾値 t - 成長ソウル 閾値 t
    pub pad: [u8;4], // パディング - 
}
