use phf;
use phf::phf_map;
use regex::Regex;
use url::Url;

include!(concat!(env!("OUT_DIR"), "/book_map.rs"));

#[derive(Debug, PartialEq)]
pub struct Verse {
    verse_str: String,
}

pub trait ToUriString {
    fn to_uri_string(&self) -> String;
}

impl ToUriString for Verse {
    fn to_uri_string(&self) -> String {
        if self.verse_str.contains('-') {
            let parts: Vec<&str> = self.verse_str.split('-').collect();
            format!("p{}-p{}#p{}", parts[0], parts[1], parts[0])
        } else {
            format!("p{}#p{}", self.verse_str, self.verse_str)
        }
    }
}

pub trait BookAbbreviation {
    fn book(&self) -> Option<String>;
    fn check_for_starting_word(&self) -> Option<String>;
    fn reference(&self) -> Option<Reference>;
}

#[derive(Debug, PartialEq)]
pub struct Reference {
    book_abbr: String,
    chapter_opt: Option<String>,
    verse_opt: Option<Verse>,
    series: String,
}

impl Reference {
    pub fn return_uri(&self) -> String {
        // Create the base URL
        let base_url = format!(
            "https://www.churchofjesuschrist.org/study/scriptures/{}/{}",
            self.series, self.book_abbr
        );

        // Parse the base URL
        let mut url = Url::parse(&base_url).unwrap();

        // Add the chapter to the path if it exists
        if let Some(chapter) = &self.chapter_opt {
            url.path_segments_mut().unwrap().push(&chapter.to_string());
        }

        url.query_pairs_mut().append_pair("lang", "eng");

        // Add the verse as a query parameter if it exists
        if let Some(verse) = &self.verse_opt {
            let uri_string = verse.to_uri_string();
            let parts: Vec<&str> = uri_string.split('#').collect();
            if parts.len() == 2 {
                url.query_pairs_mut().append_pair("id", parts[0]);
                url.set_fragment(Some(parts[1]));
            } else {
                url.query_pairs_mut().append_pair("id", &uri_string);
            }
        }

        url.into()
    }
}

impl BookAbbreviation for str {
    fn reference(&self) -> Option<Reference> {
        // formats for references:
        // <<Book:String><space>[<Chapter:u8>[:<Verse:u8>]]
        // Book is of the form: ([u8<space>](<string><space>)+
        // e.g.:
        // 1 Nephi 3
        // 1 Nephi 3:4
        // Matthew 11:28
        // Words of Mormon 1:9
        //, it is almost better to parse from right to left, first look for a colon from the right, to know if there is a verse
        //, then look for a u8, to know if there is a chapter
        //, then look for a string, to find the name of the book

        let re = Regex::new(
            r"(?x)
            ^\s*
            (?P<book>[\w\s\-]+?)      # Book name (non-greedy)
            \s*
            (?:
                (?P<chapter>\d+(-\d+)?)    # Chapter number or range
                (?:
                    :(?P<verse>\d+(-\d+)?) # Verse number or range
                )?
            )?
            \s*$
        ",
        )
        .unwrap();

        if let Some(caps) = re.captures(self) {
            let book_name = caps.name("book")?.as_str().trim().to_string();
            let chapter = caps.name("chapter").map(|ch| ch.as_str().to_string());
            let verse = caps.name("verse").map(|v| Verse {
                verse_str: v.as_str().to_string(),
            });

            if let Some(book_abbr) = book_name.book() {
                if let Some(series) = SERIES_MAP.get(&book_abbr) {
                    return Some(Reference {
                        book_abbr,
                        chapter_opt: chapter,
                        verse_opt: verse,
                        series: series.to_string(),
                    });
                }
            }
        }
        None
    }

    fn book(&self) -> Option<String> {
        let result = BOOK_MAP.get(self).map(|&s| s.to_string());
        match result {
            Some(abbr) => Some(abbr),
            None => self.check_for_starting_word(),
        }
    }

    fn check_for_starting_word(&self) -> Option<String> {
        for (key, &value) in BOOK_MAP.entries() {
            if key.starts_with(self) {
                return Some(value.to_string());
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_books_should_expand_to_abbreviation() {
        assert_eq!("1 Nephi".book(), Some("1-ne".to_string()));
        assert_eq!("1-ne".book(), Some("1-ne".to_string()));
        assert_eq!("3 Nephi".book(), Some("3-ne".to_string()));
        assert_eq!("Matt".book(), Some("matt".to_string()));
        assert_eq!("Matthew".book(), Some("matt".to_string()));
        assert_eq!("Unknown".book(), None);
        assert_eq!("wom".book(), Some("w-of-m".to_string()));
        assert_eq!("Words of Mormon".book(), Some("w-of-m".to_string()));
    }

    #[test]
    fn test_reference_parsing() {
        assert_eq!(
            "1 Nephi 3:4".reference(),
            Some(Reference {
                book_abbr: "1-ne".to_string(),
                chapter_opt: Some("3".to_string()),
                verse_opt: Some(Verse {
                    verse_str: "4".to_string()
                }),
                series: "bofm".to_string(),
            })
        );
        assert_eq!(
            "1 Nephi 3".reference(),
            Some(Reference {
                book_abbr: "1-ne".to_string(),
                chapter_opt: Some("3".to_string()),
                verse_opt: None,
                series: "bofm".to_string(),
            })
        );
        assert_eq!(
            "Matthew 11:28-30".reference(),
            Some(Reference {
                book_abbr: "matt".to_string(),
                chapter_opt: Some("11".to_string()),
                verse_opt: Some(Verse {
                    verse_str: "28-30".to_string()
                }),
                series: "nt".to_string(),
            })
        );
        assert_eq!(
            "Words of Mormon 1:9".reference(),
            Some(Reference {
                book_abbr: "w-of-m".to_string(),
                chapter_opt: Some("1".to_string()),
                series: "bofm".to_string(),
                verse_opt: Some(Verse {
                    verse_str: "9".to_string()
                })
            })
        );
        assert_eq!("Unknown 1:1".reference(), None);
    }

    #[test]
    fn test_url_generation() {
        println!("1 Nephi 3:4.ref_url(): ");
        let reference = "1 Nephi 3:4".reference();
        let answer = "1 Nephi 3:4".reference();
        println!("answer: {:?}", answer);
        println!("{:?}", reference.unwrap());
        assert_eq!(
            "1 Nephi 3:4".reference(),
            Some(Reference {
                book_abbr: "1-ne".to_string(),
                chapter_opt: Some("3".to_string()),
                series: "bofm".to_string(),
                verse_opt: Some(Verse {
                    verse_str: "4".to_string()
                })
            })
        );
        assert_eq!(
            "1 Nephi 3:4".reference().unwrap().return_uri(),
            "https://www.churchofjesuschrist.org/study/scriptures/bofm/1-ne/3?lang=eng&id=p4#p4"
                .to_string()
        );
        assert_eq!(
            "Matthew 11:28-30".reference().unwrap().return_uri(),
            "https://www.churchofjesuschrist.org/study/scriptures/nt/matt/11?lang=eng&id=p28-p30#p28"
                .to_string()
        );

        let foo = "wom".reference();
        let foo2 = "w-of-m".reference();
        println!("{:?}", foo.unwrap());
        println!("{:?}", foo2.unwrap());
        let reference = "wom".reference();

        assert_eq!(
            "wom".reference().unwrap().return_uri(),
            "https://www.churchofjesuschrist.org/study/scriptures/bofm/w-of-m?lang=eng".to_string()
        );

        assert_eq!(
            "1 Nephi".reference().unwrap().return_uri(),
            "https://www.churchofjesuschrist.org/study/scriptures/bofm/1-ne?lang=eng".to_string()
        );
        // let book = "wom".book();
        // let reference = "wom".ref_url();
        assert_eq!("Unknown".reference(), None);
    }
}
