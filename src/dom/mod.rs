use std::collections::HashMap;

/// Represents a node in the DOM tree
#[derive(Debug, Clone)]
pub enum Node {
    Element(Element),
    Text(String),
    Comment(String),
    Doctype(String),
}

/// Represents an HTML element with a tag name, attributes, and child nodes
#[derive(Debug, Clone)]
pub struct Element {
    pub tag_name: String,
    pub attributes: HashMap<String, String>,
    pub children: Vec<Node>,
}

impl Node {
    /// Creates a new element node
    pub fn new_element(tag_name: &str) -> Self {
        Node::Element(Element {
            tag_name: tag_name.to_lowercase(),
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
    pub fn new_doctype(doctype: &str) -> Self {
        Node::Doctype(doctype.to_string())
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
                if element.tag_name == "html" {
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
        let node = Node::new_element("div");
        if let Node::Element(element) = node {
            assert_eq!(element.tag_name, "div");
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
        let node = Node::new_doctype("html");
        if let Node::Doctype(doctype) = node {
            assert_eq!(doctype, "html");
        } else {
            panic!("Expected Doctype node");
        }
    }

    #[test]
    fn test_document() {
        let mut doc = Document::new();
        assert!(doc.nodes.is_empty());

        // Add DOCTYPE
        doc.add_node(Node::new_doctype("html"));

        // Add HTML element
        let mut html = Element {
            tag_name: "html".to_string(),
            attributes: HashMap::new(),
            children: Vec::new(),
        };

        // Add HEAD element with TITLE
        let mut head = Element {
            tag_name: "head".to_string(),
            attributes: HashMap::new(),
            children: Vec::new(),
        };

        let title = Element {
            tag_name: "title".to_string(),
            attributes: HashMap::new(),
            children: vec![Node::new_text("Test Page")],
        };

        head.children.push(Node::Element(title));

        // Add BODY element with text
        let body = Element {
            tag_name: "body".to_string(),
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
        assert_eq!(html_element.tag_name, "html");
        assert_eq!(html_element.children.len(), 2);
    }
}
