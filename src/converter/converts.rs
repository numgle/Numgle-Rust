use std::{cmp::min, slice::Iter, vec, sync::{Arc, RwLock}};

use rocket::Data;

use super::{
    analyzer::{
        get_coda_index, get_lower_engish_index, get_not_complete_hangul_index, get_nucleus_index,
        get_number_index, get_onset_index, get_upper_engish_index, is_char_in_array,
        is_char_in_range,
    },
    data::DataSet,
};

pub struct Converter<'a> {
    creators: Vec<NumgleCreator<'a>>,
}

impl<'a> Converter<'a> {
    pub fn new(dataset: DataSet) -> Converter<'a> {
        let dataset = Arc::new(dataset);
        let complete_hangul = NumgleCreator {
            range_checker: Box::new(is_char_in_range(dataset.range.completeHangul.to_tuple())),
            create: Box::new(convert_completehangul(Arc::clone(&dataset))),
        };
        let notcomplete_hangul = NumgleCreator {
            range_checker: Box::new(is_char_in_range(dataset.range.notCompleteHangul.to_tuple())),
            create: Box::new(convert_alphanumeric(
                Arc::clone(&dataset.han),
                get_not_complete_hangul_index,
            )),
        };
        let upper_english = NumgleCreator {
            range_checker: Box::new(is_char_in_range(dataset.range.uppercase.to_tuple())),
            create: Box::new(convert_alphanumeric(
                Arc::clone(&dataset.englishUpper),
                get_upper_engish_index,
            )),
        };
        let lower_english = NumgleCreator {
            range_checker: Box::new(is_char_in_range(dataset.range.lowercase.to_tuple())),
            create: Box::new(convert_alphanumeric(
                Arc::clone(&dataset.englishLower),
                get_lower_engish_index,
            )),
        };
        let number = NumgleCreator {
            range_checker: Box::new(is_char_in_range(dataset.range.number.to_tuple())),
            create: Box::new(convert_alphanumeric(Arc::clone(&dataset.number), get_number_index)),
        };
        let special = NumgleCreator {
            range_checker: Box::new(is_char_in_array((*dataset.range.special).clone())),
            create: Box::new(convert_special(Arc::clone(&dataset.range.special), Arc::clone(&dataset.special))),
        };
        
        Converter{creators:vec![complete_hangul,notcomplete_hangul,upper_english,lower_english,number,special]}
    }

    pub fn convert_char(&self, char: char) -> String {
        let mut iter = self.creators.iter();
        fn f(iter: &mut Iter<NumgleCreator>, char: char) -> String {
            if let Some(creator) = iter.next() {
                if let Some(result) = creator.try_create(char) {
                    return result;
                } else {
                    return f(iter, char);
                }
            } else {
                "".to_string()
            }
        }
        f(&mut iter, char)
    }

    pub fn convert_str(&self, str: &str) -> String {
        let v: Vec<String> = str.chars().map(|c| self.convert_char(c)).collect();
        v.join("\n")
    }
}

pub struct NumgleCreator<'a> {
    range_checker: Box<dyn Fn(char) -> bool + 'a + Send + Sync>,
    create: Box<dyn Fn(char) -> String + 'a + Send + Sync>,
}

impl<'a> NumgleCreator<'a> {
    fn try_create(&self, char: char) -> Option<String> {
        if (self.range_checker)(char) {
            Some((self.create)(char))
        } else {
            None
        }
    }
}

fn convert_completehangul<'a>(data: Arc<DataSet>) -> impl Fn(char) -> String + 'a {
    move |char| {
        let onset = get_onset_index(char) as usize;
        let nuclues = get_nucleus_index(char) as usize;
        let coda = get_coda_index(char) as usize;

        if nuclues >= 8 && nuclues != 20 {
            data.jong[coda].clone() + &data.jung[nuclues - 8] + &data.cho[onset].clone()
        } else {
            data.jong[coda].clone() + &data.cj[min(8, nuclues)][onset].clone()
        }
    }
}

fn convert_alphanumeric<'a, F: 'a>(
    data: Arc<Vec<String>>,
    get_index: F,
) -> impl Fn(char) -> String + 'a
where
    F: Fn(char) -> u32,
{
    move |char| {
        let index = get_index(char) as usize;
        data[index].clone()
    }
}

fn convert_special<'a>(
    index_data: Arc<Vec<u32>>,
    data: Arc<Vec<String>>,
) -> impl Fn(char) -> String + 'a {
    move |char| {
        let char_num = char as u32;
        if let Some(index) = index_data.iter().position(|&n| n == char_num) {
            let index = index as usize;
            data[index].clone()
        } else {
            "".to_string()
        }
    }
}
