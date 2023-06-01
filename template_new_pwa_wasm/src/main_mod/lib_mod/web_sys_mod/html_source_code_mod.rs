//! html_source_code_mod.rs

/// HtmlSourceCode - type to manipulate HTML source code safer than with string functions only  
/// WARNING for HTML INJECTION!   
/// HTML is the standard markup language for Web pages. HTML source code is just a text.  
/// It is easy to read, write, understand and parse.  
/// The syntax of HTML source code is similar to XML structured with elements, tags, nodes, texts, comments and attributes.  
/// The browser then transforms this HTML source code into the DOM tree and then renders that.  
/// It is very tempting to modify this source code in our application with string manipulation and then pass it to the browser.  
/// The html source code (it is just a string) that is provided by the programmer is always ok, he wants it to work properly.  
/// The BIG problem arises when we need to inject some user provided data into the HTML source code.  
/// The HTML syntax mixes instructions and data together and this creates a BIG problem.  
/// Never put user provided strings in a html source code directly, because it can contain an HTML injection attack.  
/// We need to encode all user data before putting it into the HTML source code.  
/// There are 2 types of encodings: one for attributes values and another for text nodes.  
/// We will create a new type that makes it safer and easier for the programmer to replace data in the HTML source code.  
///

pub struct HtmlSourceCode {
    html: String,
}

impl HtmlSourceCode {
    /// The programmer provides a &'static str to initiate HtmlSourceCode.  
    /// The html source code coming from the programmer is always ok, he wants it to work properly.  
    /// The data that will be replaced, have a recognizable and unique value.  
    pub fn new(html_code: &'static str) -> Self {
        HtmlSourceCode {
            html: html_code.to_string(),
        }
    }

    /// get the well formed html  
    /// We trust the programmer to carefully work with HtmlSourceCode to be always well formed and without HTML injection.  
    pub fn get_html(&self) -> String {
        self.html.clone()
    }

    /// This must be pure text, no html element are allowed for bold or italic...  
    /// We trust the programmer that it will replace only the anticipated placeholders.  
    pub fn replace_text_node(&mut self, placeholder: &'static str, text: &str) {
        self.html = self
            .html
            .replace(placeholder, &html_escape::encode_text(text));
    }

    /// The attribute value must be double_quoted.  
    /// We trust the programmer that it will replace only the anticipated placeholders.  
    pub fn replace_attribute_value(&mut self, placeholder: &'static str, value: &str) {
        self.html = self.html.replace(
            placeholder,
            &html_escape::encode_double_quoted_attribute(value),
        );
    }

    /// We expect the HtmlSourceCode to be well formed. For that we trust the programmer.  
    /// We trust the programmer that it will replace only the anticipated placeholders.  
    pub fn replace_html_source_code(
        &mut self,
        placeholder: &'static str,
        html_source_code: &HtmlSourceCode,
    ) {
        self.html = self.html.replace(placeholder, &html_source_code.get_html());
    }

    /// Injects the HTMLSourceCode into a DOM element.  
    /// We trust the programmer to carefully work with HtmlSourceCode to be always well formed and without HTML injection.  
    pub fn inject_into_dom_element(self, element_id: &str) {
        let html_element = super::get_element_by_id(element_id);
        html_element.set_inner_html(&self.html);
    }
}
