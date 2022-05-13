use super::*;

#[derive(Debug, Default, Clone)]
pub struct FilterGroup {
    sets: Vec<FilterSet>,
}

impl FilterGroup {
    pub fn add(&mut self, s: FilterSet) {
        self.sets.push(s);
    }

    pub fn add_filter(&mut self, t: FilterType) {
        if self.sets.is_empty() {
            let s: FilterSet = Default::default();
            self.sets.push(s);
        }
        let l = self.sets.len();
        self.sets[l-1].add(t);
    }
}

impl Filterable for FilterGroup {
    fn get_filterables(&self) -> Vec<Box<dyn Filterable>> {
        let mut map: Vec<Box<dyn Filterable>> = Vec::new();
        for t in &self.sets {
            map.push(Box::new(t.clone()));
        }
        map
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_is_blank() {
        let s: FilterGroup = Default::default();
        assert_eq!(s.sets.len(), 0);
    }

    #[test]
    fn adds_filter_type() {
        let mut s: FilterGroup = Default::default();
        s.add_filter(FilterType::Current);
        assert_eq!(s.sets.len(), 1);
    }
}

