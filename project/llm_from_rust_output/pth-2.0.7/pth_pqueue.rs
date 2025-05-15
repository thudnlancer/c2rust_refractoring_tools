use std::ptr;

#[derive(Debug, Clone, Copy)]
pub struct PthPqueue {
    q_head: Option<Box<PthSt>>,
    q_num: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct PthSt {
    q_prev: Option<Box<PthSt>>,
    q_next: Option<Box<PthSt>>,
    q_prio: i32,
    // Other fields omitted for brevity
}

impl PthPqueue {
    pub fn new() -> Self {
        PthPqueue {
            q_head: None,
            q_num: 0,
        }
    }

    pub fn init(&mut self) {
        self.q_head = None;
        self.q_num = 0;
    }

    pub fn insert(&mut self, prio: i32, mut t: Box<PthSt>) {
        if self.q_head.is_none() || self.q_num == 0 {
            t.q_prev = Some(t.clone());
            t.q_next = Some(t.clone());
            t.q_prio = prio;
            self.q_head = Some(t);
            self.q_num += 1;
            return;
        }

        let head = self.q_head.as_mut().unwrap();
        if head.q_prio < prio {
            t.q_prev = head.q_prev.clone();
            t.q_next = Some(head.clone());
            t.q_prev.as_mut().unwrap().q_next = Some(t.clone());
            head.q_prev = Some(t.clone());
            t.q_prio = prio;
            head.q_prio = prio - head.q_prio;
            self.q_head = Some(t);
        } else {
            let mut c = head.clone();
            let mut p = c.q_prio;
            while p - c.q_next.as_ref().unwrap().q_prio >= prio 
                && !ptr::eq(c.q_next.as_ref().unwrap().as_ref(), head.as_ref()) 
            {
                c = c.q_next.unwrap();
                p -= c.q_prio;
            }
            t.q_prev = Some(c.clone());
            t.q_next = c.q_next.clone();
            t.q_prev.as_mut().unwrap().q_next = Some(t.clone());
            t.q_next.as_mut().unwrap().q_prev = Some(t.clone());
            t.q_prio = p - prio;
            if !ptr::eq(t.q_next.as_ref().unwrap().as_ref(), head.as_ref()) {
                t.q_next.as_mut().unwrap().q_prio -= t.q_prio;
            }
        }
        self.q_num += 1;
    }

    pub fn delmax(&mut self) -> Option<Box<PthSt>> {
        if self.q_head.is_none() {
            return None;
        }

        let head = self.q_head.take().unwrap();
        if ptr::eq(head.q_next.as_ref().unwrap().as_ref(), head.as_ref()) {
            self.q_num = 0;
            Some(head)
        } else {
            let next = head.q_next.unwrap();
            head.q_prev.as_mut().unwrap().q_next = Some(next.clone());
            next.q_prev = head.q_prev.clone();
            next.q_prio = head.q_prio - next.q_prio;
            self.q_head = Some(next);
            self.q_num -= 1;
            Some(head)
        }
    }

    pub fn delete(&mut self, t: Box<PthSt>) {
        if self.q_head.is_none() {
            return;
        }

        if ptr::eq(self.q_head.as_ref().unwrap().as_ref(), t.as_ref()) {
            if ptr::eq(t.q_next.as_ref().unwrap().as_ref(), t.as_ref()) {
                self.q_head = None;
                self.q_num = 0;
            } else {
                let next = t.q_next.unwrap();
                t.q_prev.as_mut().unwrap().q_next = Some(next.clone());
                next.q_prev = t.q_prev.clone();
                next.q_prio = t.q_prio - next.q_prio;
                self.q_head = Some(next);
                self.q_num -= 1;
            }
        } else {
            t.q_prev.as_mut().unwrap().q_next = t.q_next.clone();
            t.q_next.as_mut().unwrap().q_prev = t.q_prev.clone();
            if !ptr::eq(t.q_next.as_ref().unwrap().as_ref(), self.q_head.as_ref().unwrap().as_ref()) {
                t.q_next.as_mut().unwrap().q_prio += t.q_prio;
            }
            self.q_num -= 1;
        }
    }

    pub fn favorite(&mut self, t: Box<PthSt>) -> bool {
        if self.q_head.is_none() || self.q_num == 0 {
            return false;
        }

        if self.q_num == 1 {
            return true;
        }

        self.delete(t.clone());
        let new_prio = self.q_head.as_ref().map_or(5, |h| h.q_prio + 1);
        self.insert(new_prio, t);
        true
    }

    pub fn increase(&mut self) {
        if let Some(head) = self.q_head.as_mut() {
            head.q_prio += 1;
        }
    }

    pub fn tail(&self) -> Option<Box<PthSt>> {
        self.q_head.as_ref().map(|h| h.q_prev.clone()).flatten()
    }

    pub fn walk(&self, t: &PthSt, direction: i32) -> Option<Box<PthSt>> {
        if direction == 1 << 2 {
            if !ptr::eq(t, self.q_head.as_ref().unwrap().as_ref()) {
                t.q_prev.clone()
            } else {
                None
            }
        } else if direction == 1 << 1 {
            let next = t.q_next.clone();
            if ptr::eq(next.as_ref().unwrap().as_ref(), self.q_head.as_ref().unwrap().as_ref()) {
                None
            } else {
                next
            }
        } else {
            None
        }
    }

    pub fn contains(&self, t: &PthSt) -> bool {
        let mut current = self.q_head.clone();
        while let Some(c) = current {
            if ptr::eq(c.as_ref(), t) {
                return true;
            }
            current = self.walk(c.as_ref(), 1 << 1);
        }
        false
    }
}