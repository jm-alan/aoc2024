use std::collections::{HashMap, HashSet};

use super::{PAGE_REQUIREMENTS, UPDATES};

type SetMap<'ref_life> = HashMap<&'ref_life u8, HashSet<&'ref_life u8>>;
type VecMap<'ref_life> = HashMap<&'ref_life u8, Vec<&'ref_life u8>>;

pub fn solution() -> i64 {
  let mut sum = 0;
  let mut requires_set_map: SetMap = HashMap::new();
  let mut requirements_map: VecMap = HashMap::new();
  let mut requires_restore: VecMap = HashMap::new();
  let mut requirements_restore: VecMap = HashMap::new();

  PAGE_REQUIREMENTS
    .iter()
    .for_each(|(required, required_by)| {
      if let Some(set) = requires_set_map.get_mut(required_by) {
        set.insert(required);
      } else {
        requires_set_map.insert(required_by, HashSet::new());
        requires_set_map
          .get_mut(required_by)
          .unwrap()
          .insert(required);
      }

      if let Some(vec) = requirements_map.get_mut(required) {
        vec.push(required_by);
      } else {
        requirements_map.insert(required, vec![required_by]);
      }
    });

  for i in 0..196 {
    if update_valid(
      UPDATES[i],
      &mut requires_set_map,
      &mut requirements_map,
      &mut requires_restore,
      &mut requirements_restore,
    ) {
      sum += UPDATES[i][UPDATES[i].len() / 2] as i64;
    }
  }

  sum
}

#[inline(always)]
fn update_valid(
  _update: &[u8],
  _requires_set_map: &mut SetMap,
  _requirements_map: &mut VecMap,
  _requires_restore: &mut VecMap,
  _requirements_restore: &mut VecMap,
) -> bool {
  false
}
