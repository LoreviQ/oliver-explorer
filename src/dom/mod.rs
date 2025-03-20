use std::collections::HashMap;

/// HTML element types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ElementType {
    Html,
    Head,
    Title,
    Body,
    Div,
    Span,
    P,
    A,
    Img,
    Script,
    Style,
    // Add more HTML elements as needed
    Custom(String), // For custom or less common elements
}

impl ElementType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "html" => ElementType::Html,
            "head" => ElementType::Head,
            "title" => ElementType::Title,
            "body" => ElementType::Body,
            "div" => ElementType::Div,
            "span" => ElementType::Span,
            "p" => ElementType::P,
            "a" => ElementType::A,
            "img" => ElementType::Img,
            "script" => ElementType::Script,
            "style" => ElementType::Style,
            // Add more mappings as needed
            _ => ElementType::Custom(s.to_string()),
        }
    }
}

/// DOCTYPE declarations
#[derive(Debug, Clone, PartialEq)]
pub enum DoctypeType {
    Html5,               // <!DOCTYPE html>
    Html4Strict,         // HTML 4.01 Strict
    Html4Transitional,   // HTML 4.01 Transitional
    Html4Frameset,       // HTML 4.01 Frameset
    Xhtml10Strict,       // XHTML 1.0 Strict
    Xhtml10Transitional, // XHTML 1.0 Transitional
    Xhtml10Frameset,     // XHTML 1.0 Frameset
    Xhtml11,             // XHTML 1.1
    Custom(String),      // For custom doctypes
}

impl DoctypeType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "html" => DoctypeType::Html5,
            // Add more mappings for other DOCTYPE declarations
            _ => DoctypeType::Custom(s.to_string()),
        }
    }
}

/// Represents a node in the DOM tree
#[derive(Debug, Clone)]
pub enum Node {
    Element(Element),
    Text(String),
    Comment(String),
    Doctype(DoctypeType),
}

/// Represents an HTML element with a tag name, attributes, and child nodes
#[derive(Debug, Clone)]
pub struct Element {
    pub element_type: ElementType,
    pub attributes: HashMap<String, String>,
    pub children: Vec<Node>,
}

impl Node {
    /// Creates a new element node
    pub fn new_element(tag_name: ElementType) -> Self {
        Node::Element(Element {
            element_type: tag_name,
            attributes: HashMap::new(),
            children: Vec::new(),
        })
    }

    /// Creates a new text node
    pub fn new_text(content: &str) -> Self {
        Node::Text(content.to_string())
    }

    /// Creates a new comment node
    pub fn new_comment(content: &str) -> Self {
        Node::Comment(content.to_string())
    }

    /// Creates a new doctype node
    pub fn new_doctype(doctype: DoctypeType) -> Self {
        Node::Doctype(doctype)
    }
}

/// Represents a complete DOM document
#[derive(Debug, Clone)]
pub struct Document {
    pub nodes: Vec<Node>,
}

impl Document {
    pub fn new() -> Self {
        Document { nodes: Vec::new() }
    }

    /// Adds a node to the document
    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    /// Returns the HTML element if it exists in the document
    pub fn html_element(&self) -> Option<&Element> {
        for node in &self.nodes {
            if let Node::Element(element) = node {
                if element.element_type == ElementType::Html {
                    return Some(element);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_element() {
        let node = Node::new_element(ElementType::Div);
        if let Node::Element(element) = node {
            assert_eq!(element.element_type, ElementType::Div);
            assert!(element.attributes.is_empty());
            assert!(element.children.is_empty());
        } else {
            panic!("Expected Element node");
        }
    }

    #[test]
    fn test_new_text() {
        let node = Node::new_text("Hello World");
        if let Node::Text(text) = node {
            assert_eq!(text, "Hello World");
        } else {
            panic!("Expected Text node");
        }
    }

    #[test]
    fn test_new_comment() {
        let node = Node::new_comment("This is a comment");
        if let Node::Comment(comment) = node {
            assert_eq!(comment, "This is a comment");
        } else {
            panic!("Expected Comment node");
        }
    }

    #[test]
    fn test_new_doctype() {
        let node = Node::new_doctype(DoctypeType::Html5);
        if let Node::Doctype(doctype) = node {
            assert_eq!(doctype, DoctypeType::Html5);
        } else {
            panic!("Expected Doctype node");
        }
    }

    #[test]
    fn test_document() {
        let mut doc = Document::new();
        assert!(doc.nodes.is_empty());

        // Add DOCTYPE
        doc.add_node(Node::new_doctype(DoctypeType::Html5));

        // Add HTML element
        let mut html = Element {
            element_type: ElementType::Html,
            attributes: HashMap::new(),
            children: Vec::new(),
        };

        // Add HEAD element with TITLE
        let mut head = Element {
            element_type: ElementType::Head,
            attributes: HashMap::new(),
            children: Vec::new(),
        };

        let title = Element {
            element_type: ElementType::Title,
            attributes: HashMap::new(),
            children: vec![Node::new_text("Test Page")],
        };

        head.children.push(Node::Element(title));

        // Add BODY element with text
        let body = Element {
            element_type: ElementType::Body,
            attributes: HashMap::new(),
            children: vec![Node::new_text("Hello World")],
        };

        html.children.push(Node::Element(head));
        html.children.push(Node::Element(body));

        doc.add_node(Node::Element(html));

        // Test document structure
        assert_eq!(doc.nodes.len(), 2);

        // Test html_element helper
        let html_element = doc.html_element().unwrap();
        assert_eq!(html_element.element_type, ElementType::Html);
        assert_eq!(html_element.children.len(), 2);
    }

    #[test]
    fn test_custom_element() {
        let node = Node::new_element(ElementType::Custom("custom-element".to_string()));
        if let Node::Element(element) = node {
            if let ElementType::Custom(name) = &element.element_type {
                assert_eq!(name, "custom-element");
            } else {
                panic!("Expected Custom element type");
            }
        } else {
            panic!("Expected Element node");
        }
    }
}
