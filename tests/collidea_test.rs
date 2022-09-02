use collidea::collidea::{CollideMap, CollideSet};

#[test]
fn collide_map_test() {
    let size = 100;
    let mut map: CollideMap<String, i32> = CollideMap::new();

    for i in 0..size {
        let key = i.to_string();
        map.insert(&key, i);
    }

    for i in 0..size {
        let key = i.to_string();
        assert_eq!(map.get(&key).unwrap(), &i);
    }
    assert_eq!(map.len(), size as usize);
}

#[test]
fn collide_set_test() {
    let size = 100;
    let mut set: CollideSet<String> = CollideSet::new();

    for i in 0..size {
        let element = i.to_string();
        set.insert(element);
    }
    assert_eq!(set.len(), size as usize);

    for i in 0..size {
        let element = i.to_string();
        assert_eq!(set.contains(&element), true);
        assert_eq!(set.contains(&(i + size).to_string()), false);
        assert_eq!(set.remove(&element), true);
    }
    assert_eq!(set.len(), 0);
}

#[test]
fn collide_set_operator_test() {
    let size = 30;
    let a_start = 0;
    let b_start = size * 1 / 3;
    let a_end = size * 2 / 3;
    let b_end = size;

    let mut set_a: CollideSet<String> = CollideSet::new();
    let mut set_b: CollideSet<String> = CollideSet::new();

    for i in a_start..a_end {
        let t = i.to_string();
        set_a.insert(t);
    }
    for i in b_start..b_end {
        let t = i.to_string();
        set_b.insert(t);
    }

    let intersection_a_b = set_a.intersection(&set_b);
    let intersection_b_a = set_b.intersection(&set_a);
    let union_a_b = set_a.union(&set_b);
    let union_b_a = set_b.union(&set_a);
    let difference_a_b = set_a.difference(&set_b);
    let difference_b_a = set_b.difference(&set_a);
    let symmetric_difference_a_b = set_a.symmetric_difference(&set_b);
    let symmetric_difference_b_a = set_b.symmetric_difference(&set_a);

    assert_eq!(set_a.is_empty(), false);
    assert_eq!(set_b.is_empty(), false);
    assert_eq!(union_a_b.len(), size);
    assert_eq!(union_b_a.len(), size);
    assert_eq!(intersection_a_b.len(), a_end - b_start);
    assert_eq!(intersection_b_a.len(), a_end - b_start);

    for i in a_start..b_end {
        let element = i.to_string();
        assert_eq!(set_a.contains(&element), i < a_end);
        assert_eq!(set_b.contains(&element), i >= b_start);
        assert_eq!(union_a_b.contains(&element), true);
        assert_eq!(union_b_a.contains(&element), true);
        assert_eq!(
            intersection_a_b.contains(&element),
            i >= b_start && i < a_end
        );
        assert_eq!(
            intersection_b_a.contains(&element),
            i >= b_start && i < a_end
        );
        assert_eq!(difference_a_b.contains(&element), i < b_start);
        assert_eq!(difference_b_a.contains(&element), i >= a_end);
        assert_eq!(
            difference_a_b.contains(&element) && difference_b_a.contains(&element),
            false
        );
        assert_eq!(
            symmetric_difference_a_b.contains(&element),
            i < b_start || i >= a_end
        );
        assert_eq!(
            symmetric_difference_a_b.contains(&element),
            symmetric_difference_b_a.contains(&element)
        );
        assert_eq!(
            intersection_a_b.contains(&element) ^ symmetric_difference_a_b.contains(&element),
            true
        );
    }

    assert_eq!(
        set_a.is_subset(&union_a_b) && set_b.is_subset(&union_a_b),
        true
    );
    assert_eq!(
        union_a_b.is_subset(&union_b_a) && union_b_a.is_subset(&union_a_b),
        true
    );
    assert_eq!(
        intersection_a_b.is_subset(&set_a) && intersection_a_b.is_subset(&set_b),
        true
    );
    assert_eq!(
        intersection_a_b.is_subset(&intersection_b_a)
            && intersection_b_a.is_subset(&intersection_a_b),
        true
    );
    assert_eq!(difference_a_b.is_subset(&set_a), true);
    assert_eq!(difference_b_a.is_subset(&set_b), true);
    assert_eq!(difference_a_b.is_subset(&symmetric_difference_a_b), true);
    assert_eq!(difference_b_a.is_subset(&symmetric_difference_b_a), true);
    assert_eq!(intersection_a_b.is_subset(&union_a_b), true);
    assert_eq!(symmetric_difference_a_b.is_subset(&union_a_b), true);
    assert_eq!(
        symmetric_difference_a_b.is_subset(&symmetric_difference_b_a)
            && symmetric_difference_b_a.is_subset(&symmetric_difference_a_b),
        true
    );
    assert_eq!(
        set_a.is_disjoint(&difference_b_a) && set_b.is_disjoint(&difference_a_b),
        true
    );
    assert_eq!(
        intersection_a_b.is_disjoint(&symmetric_difference_a_b),
        true
    );
    assert_eq!(difference_a_b.is_disjoint(&difference_b_a), true);
    assert_eq!(difference_a_b.is_disjoint(&intersection_a_b), true);
    assert_eq!(difference_b_a.is_disjoint(&intersection_b_a), true);
}
