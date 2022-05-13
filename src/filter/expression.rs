use super::*;

#[derive(Debug)]
pub struct FilterExpression {
    groups: Vec<FilterGroup>,
}

impl Default for FilterExpression {
    fn default() -> Self {
        let g: FilterGroup = Default::default();
        Self { groups: vec![g] }
    }
}

impl std::fmt::Display for FilterExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for (idx, group) in self.groups.iter().enumerate() {
            if idx > 0 {
                write!(f, "{}", &group.strategy)?;
            }
            write!(f, "{}", &group)?;
        }
        Ok(())
    }
}

impl FilterExpression {
    pub fn change_strategy(&mut self, s: DataStrategy) {
        if let Some(last) = self.groups.last_mut() {
            last.strategy = s;
        }
    }

    pub fn add_group(&mut self, g: FilterGroup) {
        self.groups.push(g);
    }

    pub fn add_set(&mut self, s: FilterSet) {
        let l = self.groups.len();
        self.groups[l - 1].add_set(s);
    }

    pub fn add_filter(&mut self, t: FilterType) {
        if self.groups.is_empty() {
            let g: FilterGroup = Default::default();
            self.groups.push(g);
        }
        let l = self.groups.len();
        self.groups[l - 1].add_filter(t);
    }
}

impl Filterable for FilterExpression {
    fn get_filterables(&self) -> Vec<Box<dyn Filterable>> {
        let mut map: Vec<Box<dyn Filterable>> = Vec::new();
        for set in &self.groups {
            map.push(Box::new(set.clone()));
        }
        map
    }
}
