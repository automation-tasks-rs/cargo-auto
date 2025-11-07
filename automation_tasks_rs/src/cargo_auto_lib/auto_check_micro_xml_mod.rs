// auto_check_micro_xml_mod.rs

//! Checks the correctness of micro XML files.

use crate::cargo_auto_lib::error_mod::{Error, Result};
use crate::cargo_auto_lib::public_api_mod::{RED, RESET, YELLOW};
use crate::generic_functions_mod::{pos, ResultLogError};
use glob::glob;
use reader_for_microxml::{ReaderForMicroXml, Token};

/// I want html pages to be correct microXML when I use them for single page application.
/// Before build or release this function will check for correctness.
pub fn auto_check_micro_xml(path_to_html_pages: &str) -> Result<()> {
    println!("  {YELLOW}Running auto_check_micro_xml {path_to_html_pages}{RESET}");
    let glob_str = format!("{}/*.html", path_to_html_pages.trim_end_matches('/'));
    // find html pages for single page application
    for filename_result in glob(&glob_str).log(pos!())? {
        let filename_pathbuff = filename_result?;
        let filename_pathbuff = camino::Utf8Path::from_path(&filename_pathbuff)
            .ok_or_else(|| Error::ErrorFromStr("filename is None"))
            .log(pos!())?;

        let file_name = filename_pathbuff
            .file_name()
            .ok_or_else(|| Error::ErrorFromStr("filename is None"))
            .log(pos!())?;
        let str_xml = std::fs::read_to_string(filename_pathbuff).log(pos!())?;

        // check if file have CRLF instead of LF and show error
        if str_xml.contains("\r\n") {
            return Err(Error::ErrorFromString(format!(
                "{RED}Error: {filename_pathbuff} has CRLF line endings instead of LF. Correct the file! {RESET}"
            )));
        }

        // check microxml correctness. Error on errors.
        check_micro_xml(&str_xml, file_name).log(pos!())?;
    }
    println!("  {YELLOW}Finished auto_check_micro_xml{RESET}");
    Ok(())
}

/// Errors if the microXML string is not correct
fn check_micro_xml(str_xml: &str, file_name: &str) -> Result<()> {
    println!("  {YELLOW}Check MicroXml: {file_name}{RESET}");
    // remove <!DOCTYPE html> because it is not microXML
    let str_xml = str_xml.replace("<!DOCTYPE html>", "");
    let reader_iterator = ReaderForMicroXml::new(&str_xml);
    // reader_iterator is iterator Option<Result<Token,&str>>
    // the first option is used for the iterator to know where is the end
    // then the Result can have an Token or an Error
    let mut vec_elem: Vec<&str> = vec![];
    for result_token in reader_iterator {
        match result_token {
            Ok(token) => match token {
                Token::StartElement(name) => vec_elem.push(name),
                Token::Attribute(_name, _value) => continue,
                Token::TextNode(_txt) => continue,
                Token::Comment(_txt) => continue,
                Token::EndElement(end_element_name) => {
                    let start_element_name = vec_elem
                        .pop()
                        .ok_or_else(|| Error::ErrorFromStr("vec_elem.pop() is None"))
                        .log(pos!())?;
                    if !end_element_name.is_empty() && end_element_name != start_element_name {
                        return Err(Error::ErrorFromString(format!(
                            "{RED}MicroXml {} start {} does not match end {}{RESET}",
                            file_name, start_element_name, end_element_name,
                        )));
                    }
                }
            },
            Err(err_msg) => {
                return Err(Error::ErrorFromString(format!(
                    "{RED}MicroXml {} incorrect : {}{RESET}",
                    file_name, err_msg,
                )));
            }
        }
    }
    Ok(())
}
