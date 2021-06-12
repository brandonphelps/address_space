#![allow(dead_code)]


#[derive(Debug, PartialEq, Eq)]
pub struct Section {
    start_addr: u32,
    data: Vec<u8>
}

impl Section {
    pub fn new(start_addr: u32, data: Vec<u8>) -> Self {
	Self {
	    start_addr: start_addr,
	    data: data,
	}
    }

    /// @brief addr is aboslute, and offset will be applied internally
    /// can only write data to within the section available.
    fn write_data(&mut self, addr: u32, value: u8) {
        let offset: usize = (addr - self.start_addr) as usize;
        self.data[offset] = value;
    }

    /// @breif appends data to the end of the segment
    fn push_data(&mut self, value: u8) {
        self.data.push(value);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn read_data(&self, addr: u32, size: usize) -> Option<Vec<u8>> {
        let mut result = Vec::new();
        // bounds checking.
        if size > self.data.len() {
            return None;
        }

        let mut offset = addr - self.start_addr;
        // todo: switch to some cpy things.
        for _i in 0..size {
            result.push(self.data[offset as usize]);
            offset += 1;
        }
        return Some(result);
    }

    pub fn start_addr(&self) -> u32 {
        self.start_addr
    }

    pub fn end_addr(&self) -> u32 {
        self.start_addr + self.data.len() as u32
    }

    /// @brief consumes the sec2 and updates the current record
    /// with the values of the second, must be adjacent segments.
    fn merge_with(&mut self, sec2: Self) -> bool {
        match merge_sections(&self, &sec2) {
            Ok(p) => {
                self.start_addr = p.start_addr;
                self.data = p.data;
                true
            }
            Err(_f) => false,
        }
    }
}


/// @brief performs a merge of two record sections.
/// by createing a third section of contiguious memory regions.
/// returns None if the two regions aren't neighbors or are overlapping. 
fn merge_sections(sec1: &Section, sec2: &Section) -> Result<Section, String> {
    // overlapping sections.
    if sec1.start_addr == sec2.start_addr {
        return Err("Overlapping addresses".into());
    }

    let mut start_addr = 0;
    let mut max_addr = 0;
    let mut min_length = 0;
    if sec1.start_addr < sec2.start_addr {
        start_addr = sec1.start_addr;
	min_length = sec1.data.len();
	max_addr = sec2.start_addr;
    } else if sec1.start_addr > sec2.start_addr {
        start_addr = sec2.start_addr;
	min_length = sec2.data.len();
	max_addr = sec1.start_addr;	
    }
    
    if (start_addr as usize + min_length) == max_addr as usize {
        let mut new_section = Section { start_addr: start_addr, data: Vec::new() };
        
        // need to order the data properly.
        if sec1.start_addr < sec2.start_addr { 
            for i in sec1.data.iter() {
                new_section.data.push(*i);
            }

            for i in sec2.data.iter() {
                new_section.data.push(*i)
            }
        } else {
            for i in sec2.data.iter() {
                new_section.data.push(*i)
            }
            for i in sec1.data.iter() {
                new_section.data.push(*i);
            }
        }
        Ok(new_section)
    } else {
	println!("non contigious sections");
	Err("Non contigous sections".into())
    }
}


pub struct AddressSpace {
    data: Vec<Section>,
}

impl AddressSpace {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }

    // todo: can we merge these and the second one?
    fn find_section(&self, addr: u32) -> Option<&Section> {
        for i in self.data.iter() {
            if i.start_addr < addr {
                if addr < i.end_addr() {
                    return Some(i);
                }
            } else if i.start_addr == addr {
                return Some(i);
            }
        }
        return None;
    }

    fn find_section_mut(&mut self, addr: u32) -> Option<&mut Section> {
        for i in self.data.iter_mut() {
            if i.start_addr < addr {
                if addr < i.end_addr() {
                    return Some(i);
                }
            } else if i.start_addr == addr {
                return Some(i);
            } else if i.start_addr > addr {
                return None;
            }
        }
        return None;
    }

    fn find_neighboring_section(&self, sec: &Section) -> Option<&Section> {
        for i in self.data.iter() {
            if sec.end_addr() == i.start_addr {
                return Some(i);
            }
        }
        return None;
    }

    /// @breif searches for neighbors and joins them together.
    /// will only perform a single neighbor search and join.
    fn consolidate(&mut self) {
        let mut has_neighbor_index = None;
        for (index, i) in self.data.iter().enumerate() {
            let p = self.find_neighboring_section(i);
            if p.is_some() {
                has_neighbor_index = Some(index + 1);
                break;
            }
        }

        match has_neighbor_index {
            Some(index) => {
                let neighbor = self.data.swap_remove(index);
                self.data[index - 1].merge_with(neighbor);
            }
            None => {}
        }
    }

    // maintains sorted order via insertion sort.
    fn insert_section(&mut self, section: Section) {
        // must already be sorted
        // sort on the insert.
        match self
            .data
            .binary_search_by(|x| x.start_addr.cmp(&section.start_addr))
        {
            Err(s) => self.data.insert(s, section),
            Ok(_f) => (),
        }
    }

    pub fn set_data(&mut self, addr: u32, data: u8) {
        let section = self.find_section_mut(addr);
        if section.is_some() {
            section.unwrap().write_data(addr, data);
        } else {
            // quick test of neighboring data.
            let mut skip_none = false;
            if addr > 1 {
                let sect2 = self.find_section_mut(addr - 1);
                if sect2.is_some() {
                    sect2.unwrap().push_data(data);
                    skip_none = true;
                }
            }
            if !skip_none {
                let new_section = Section {
                    start_addr: addr,
                    data: vec![data],
                };
                self.insert_section(new_section);
                // Two consolidates due to adding an entry into the middle of two sections.
                self.consolidate();
            }
        }
        self.consolidate();
    }

    pub fn write_data(&mut self, _addr: u32, _data: &Vec<u8>) {}

    pub fn get_bytes(&self, addr: u32, size: usize) -> Option<Vec<u8>> {
        match self.find_section(addr) {
            Some(sec) => sec.read_data(addr, size),
            None => None,
        }
    }
}


#[cfg(test)]
mod tests  {
    use super::*;
    #[test]
    fn test_section_merge() {
	let section_one = Section::new(0, vec![1,2,3,4,5]);
	let section_two = Section::new(5, vec![6,7,8,9]);
	let section_three = merge_sections(&section_one, &section_two);
	assert!(section_three.is_ok());
	let r = section_three.unwrap();
	assert_eq!(r.data, vec![1,2,3,4,5,6,7,8,9]);
	assert_eq!(r.start_addr, 0);
    }

    #[test]
    fn merge_order() {
	let section_one = Section::new(0, vec![1,2,3,4,5]);
	let section_two = Section::new(5, vec![6,7,8,9]);
	let section_three = merge_sections(&section_one, &section_two);
	let section_four = merge_sections(&section_two, &section_one);

	assert!(section_three.is_ok());
	assert!(section_four.is_ok());
	assert_eq!(section_three.unwrap().data, section_four.unwrap().data);
    }

    
}
