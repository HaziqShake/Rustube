#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Page {
    Home,
    Search(String),
    Watch(String),
    Subscriptions,
    History,
    Downloads,
    Settings,
}

pub struct Router {
    pub current: Page,
    history: Vec<Page>,
    forward_stack: Vec<Page>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            current: Page::Home,
            history: Vec::new(),
            forward_stack: Vec::new(),
        }
    }

    pub fn navigate_to(&mut self, page: Page) {
        if self.current != page {
            self.history.push(self.current.clone());
            self.forward_stack.clear();
            self.current = page;
        }
    }

    pub fn can_go_back(&self) -> bool {
        !self.history.is_empty()
    }

    pub fn can_go_forward(&self) -> bool {
        !self.forward_stack.is_empty()
    }

    pub fn go_back(&mut self) -> Option<Page> {
        if let Some(prev) = self.history.pop() {
            self.forward_stack.push(self.current.clone());
            self.current = prev.clone();
            Some(prev)
        } else {
            None
        }
    }

    pub fn go_forward(&mut self) -> Option<Page> {
        if let Some(next) = self.forward_stack.pop() {
            self.history.push(self.current.clone());
            self.current = next.clone();
            Some(next)
        } else {
            None
        }
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}
