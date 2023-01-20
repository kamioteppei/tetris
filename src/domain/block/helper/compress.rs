use crate::domain::{block::atom::Atom, contract::Config};

pub struct CompressHelper {}
impl CompressHelper {
    pub fn get_vec_is_fullfill_by_line(config: &Config, atoms: &Vec<Atom>) -> Vec<bool> {
        let mut vec_atom_counts_by_line = vec![0; config.height as usize];
        for atom in atoms {
            let &(_, y) = atom.ref_point();
            vec_atom_counts_by_line[y as usize] += 1;
        }
        let vec_is_fullfill_by_line: Vec<bool> = vec_atom_counts_by_line
            .iter()
            .map(|count| *count == config.width)
            .collect();
        vec_is_fullfill_by_line
    }

    pub fn get_vec_down_size_by_line(vec_is_fullfill_by_line: &Vec<bool>) -> Vec<i32> {
        let mut down_size_temp = 0;
        let vec_down_size_by_line = vec_is_fullfill_by_line
            .iter()
            .map(|is_delete| {
                if *is_delete {
                    {
                        down_size_temp += 1;
                        0
                    }
                } else {
                    down_size_temp
                }
            })
            .collect();
        vec_down_size_by_line
    }

    pub fn delete_atoms(atoms: &Vec<Atom>, vec_is_fullfill_by_line: &Vec<bool>) -> Vec<Atom> {
        let deleted_atoms = atoms
            .clone()
            .into_iter()
            .filter(|atom| {
                let &(_, y) = atom.ref_point();
                !vec_is_fullfill_by_line[y as usize]
            })
            .collect();
        deleted_atoms
    }

    pub fn compress_atoms(atoms: &Vec<Atom>, vec_down_size_by_line: &Vec<i32>) -> Vec<Atom> {
        let compressed_atoms = atoms
            .clone()
            .into_iter()
            .map(|atom| {
                let &(x, y) = atom.ref_point();
                Atom::new(
                    (x, y - vec_down_size_by_line[y as usize]),
                    atom.ref_bgcolor().clone(),
                )
            })
            .collect();
        compressed_atoms
    }

    pub fn get_delete_line_count(vec_is_fullfill_by_line: &Vec<bool>) -> i32 {
        let delete_line_count = vec_is_fullfill_by_line
            .iter()
            .filter(|is_delete| **is_delete)
            .count();
        delete_line_count as i32
    }
}
