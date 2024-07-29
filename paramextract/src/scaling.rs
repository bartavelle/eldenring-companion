use crate::structs::calc_correct_graph::CACL_CORRECT_GRAPH_ST;

#[derive(Debug)]
pub struct Scaling {
    // starts at stat point 1
    inner: Vec<f32>,
}

/*

stage_max_val0: 1.0,
stage_max_val1: 18.0,
stage_max_val2: 60.0,
stage_max_val3: 80.0,
stage_max_val4: 150.0,

stage_max_grow_val0: 0.0,
stage_max_grow_val1: 25.0,
stage_max_grow_val2: 75.0,
stage_max_grow_val3: 90.0,
stage_max_grow_val4: 110.0,

adj_pt_max_grow_val0: 1.2,
adj_pt_max_grow_val1: -1.2,
adj_pt_max_grow_val2: 1.0,
adj_pt_max_grow_val3: 1.0,
adj_pt_max_grow_val4: 1.0,

*/

fn add_scale(v: &mut Vec<f32>, max_val_a: f32, max_val_b: f32, max_grow_a: f32, max_grow_b: f32, adj: f32) {
    let mut cur = max_val_a;

    while cur < max_val_b {
        let ratio = (cur - max_val_a) / (max_val_b - max_val_a);
        let gval = if adj > 0.0 {
            ratio.powf(adj)
        } else {
            1.0 - (1.0 - ratio).powf(-adj)
        };
        v.push((max_grow_b - max_grow_a) * gval + max_grow_a);
        cur += 1.0;
    }
}

impl Scaling {
    pub fn new(params: &CACL_CORRECT_GRAPH_ST) -> Self {
        let mut inner = Vec::new();
        add_scale(
            &mut inner,
            params.stage_max_val0,
            params.stage_max_val1,
            params.stage_max_grow_val0,
            params.stage_max_grow_val1,
            params.adj_pt_max_grow_val0,
        );
        add_scale(
            &mut inner,
            params.stage_max_val1,
            params.stage_max_val2,
            params.stage_max_grow_val1,
            params.stage_max_grow_val2,
            params.adj_pt_max_grow_val1,
        );
        add_scale(
            &mut inner,
            params.stage_max_val2,
            params.stage_max_val3,
            params.stage_max_grow_val2,
            params.stage_max_grow_val3,
            params.adj_pt_max_grow_val2,
        );
        add_scale(
            &mut inner,
            params.stage_max_val3,
            params.stage_max_val4,
            params.stage_max_grow_val3,
            params.stage_max_grow_val4,
            params.adj_pt_max_grow_val3,
        );
        inner.push(params.stage_max_grow_val4);
        Self { inner }
    }

    pub(crate) fn power(&self, str: u8) -> f32 {
        let idx = str - 1;
        if idx as usize >= self.inner.len() {
            *self.inner.last().unwrap()
        } else {
            self.inner[idx as usize]
        }
    }
}
