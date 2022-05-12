use super::*;

#[derive(Debug, Default, Clone)]
pub struct FilterSet {
    // TODO: only pub because of parser tests, address this!
    pub types: Vec<FilterType>,
}

impl FilterSet {
    pub fn add(&mut self, t: FilterType) {
        self.types.push(t);
    }
}

impl Filterable for FilterSet {
    fn get_filterables(&self) -> Vec<Box<dyn Filterable>> {
        let mut map: Vec<Box<dyn Filterable>> = Vec::new();
        for t in &self.types {
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
        let s: FilterSet = Default::default();
        assert_eq!(s.types.len(), 0);
    }

    #[test]
    fn adds_filter_type() {
        let mut s: FilterSet = Default::default();
        s.add(FilterType::Current);
        assert_eq!(s.types.len(), 1);
        assert_eq!(s.types[0], FilterType::Current);
    }
}
