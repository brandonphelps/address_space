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
            Some(p) => {
                self.start_addr = p.start_addr;
                self.data = p.data;
                true
            }
            None => false,
        }
    }
}


/// @brief performs a merge of two record sections.
/// by createing a third section of contiguious memory regions.
/// returns None if the two regions aren't neighbors or are overlapping. 
fn merge_sections(sec1: &Section, sec2: &Section) -> Option<Section> {

    let res = None;

    // overlapping sections.
    if sec1.start_addr == sec2.start_addr {
        return None;
    }

    let mut start_addr = 0;

    if sec1.start_addr < sec2.start_addr {
        start_addr = sec1.start_addr;
    } else if sec1.start_addr > sec2.start_addr {
        start_addr = sec2.start_addr;
    }

    
    if (sec1.start_addr as usize + sec1.data.len()) == sec2.start_addr as usize {

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
        Some(new_section)
    } else {
        res
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
	assert!(section_three.is_some());
	let r = section_three.unwrap();
	assert_eq!(r.data, vec![1,2,3,4,5,6,7,8,9]);
	assert_eq!(r.start_addr, 0);
    }


    
}
