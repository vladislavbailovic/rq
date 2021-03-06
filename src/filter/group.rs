use super::*;

#[derive(Debug, Clone)]
pub enum DataStrategy {
    Serial,
    Concat,
}

impl std::fmt::Display for DataStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match &self {
            DataStrategy::Serial => write!(f, "|"),
            DataStrategy::Concat => write!(f, ","),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FilterGroup {
    pub strategy: DataStrategy,
    sets: Vec<FilterSet>,
}

impl Default for FilterGroup {
    fn default() -> Self {
        Self {
            sets: Vec::new(),
            strategy: DataStrategy::Serial,
        }
    }
}

impl std::fmt::Display for FilterGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for (idx, set) in self.sets.iter().enumerate() {
            if idx > 0 {
                write!(f, "{}", self.strategy)?;
            }
            write!(f, "{}", &set)?;
        }
        Ok(())
    }
}

impl FilterGroup {
    pub fn add_set(&mut self, s: FilterSet) {
        self.sets.push(s);
    }

    pub fn add_filter(&mut self, t: FilterType) {
        if self.sets.is_empty() {
            let s: FilterSet = Default::default();
            self.sets.push(s);
        }
        let l = self.sets.len();
        self.sets[l - 1].add(t);
    }

    fn apply_serial(&self, original_data: Data) -> Result<Data, Error> {
        let mut data = original_data;
        let filterables = self.get_filterables();
        for filterable in filterables {
            let new_data = filterable.apply(data)?;
            data = new_data.clone();
        }
        Ok(data)
    }

    fn apply_concat(&self, original_data: Data) -> Result<Data, Error> {
        let mut data: Vec<Data> = Vec::new();
        let filterables = self.get_filterables();
        for filterable in filterables {
            let new_data = filterable.apply(original_data.clone())?;
            data.push(new_data.clone());
        }
        Ok(Data::Array(data))
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

    fn apply(&self, original_data: Data) -> Result<Data, Error> {
        match self.strategy {
            DataStrategy::Serial => self.apply_serial(original_data),
            DataStrategy::Concat => self.apply_concat(original_data),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_is_blank() {
        let g: FilterGroup = Default::default();
        assert_eq!(g.sets.len(), 0);
    }

    #[test]
    fn adds_set() {
        let mut g: FilterGroup = Default::default();
        let s: FilterSet = Default::default();
        g.add_set(s);
        assert_eq!(g.sets.len(), 1);
    }

    #[test]
    fn adds_filter_type_auto_adds_set() {
        let mut g: FilterGroup = Default::default();
        g.add_filter(FilterType::Current);
        assert_eq!(g.sets.len(), 1);
    }
}
