
// script to map out the xyz transforms as direct
// x,y,z -> dx, dy, dz transforms
let x = 4, y = 5, z = 6;

let ops = ['r', 'p', 'y'];
let apply_op = (o, coord) => {
  let [x,y,z] = coord;
  if (o == 'r') {
    return [x, z, -y]
  }
  if (o == 'p') {
    return [-z, y, x]
  }
  if (o == 'y') {
    return [y, -x, z]
  }
}
let apply_transform = (ops, coord) => {
  let v = [...coord];
  for (o of ops) {
    v = apply_op(o, v)
  }
  return v;
}
let find_all_transforms = () => {
  let observed_results = new Set();
  let q = [{transform: [], vec: [x, y, z]}];
  while (q.length > 0) {
    let cur = q.pop();
    for (o of ops) {
      let new_transform = [...cur.transform, o];
      
      let new_value = apply_transform(o, cur.vec);
      let is_new_value = !observed_results.has(new_value.join(','));
      if (is_new_value) {
        observed_results.add(new_value.join(','));
        q.push({
          transform: new_transform,
          vec: new_value
        })
      }
    }
  }
  let detect_source = (v) => {
    let sign = v > 0 ? '' : '-';
    let vabs = Math.abs(v);
    return sign + (vabs === x ? 'x' : vabs === y ? 'y' : 'z')
  }
  let r = Array.from(observed_results).map(t => t.split(',').map(v => detect_source(Number(v))))
  let i = 0;
  for (let el of r) {
    i++;
    console.log(`O::_${i} => (${el.join(',')}),`)
  }
}

find_all_transforms();