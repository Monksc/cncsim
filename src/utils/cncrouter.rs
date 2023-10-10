use std::convert::From;

pub struct CNCRouter {
    tools: Vec<Tool>,
    pos: Coordinate,
    tool_index: usize,
}

pub struct Tool {
    pub radius: f64,
    pub length: f64,
}

pub struct Coordinate {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct OptionCoordinate {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub z: Option<f64>,
}


// MARK: CNCRouter

impl From<Vec<Tool>> for CNCRouter {
    fn from(tools: Vec<Tool>) -> Self {
        Self {
            tools: tools,
            pos: Default::default(),
            tool_index: 0,
        }
    }
}

impl CNCRouter {
    pub fn set_tool(&mut self, index: usize) {
        self.tool_index = index;
    }
    pub fn set_pos(&mut self, pos: &OptionCoordinate) {
        if let Some(x) = pos.x {
            self.pos.x = x;
        }
        if let Some(y) = pos.y {
            self.pos.y = y;
        }
        if let Some(z) = pos.z {
            self.pos.z = z;
        }
    }
    pub fn get_pos(&self) -> &Coordinate {
        &self.pos
    }
    pub fn get_tool(&self) -> &Tool {
        &self.tools[self.tool_index]
    }
}

// MARK: Tool

impl Default for Tool {
    fn default() -> Self {
        Self {
            radius: 0.3,
            length: 0.5,
        }
    }
}

// MARK: Coordinate

impl Default for Coordinate {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Coordinate {
    pub fn distance_to(&self, point: &Coordinate) -> f64 {
        let dx = point.x - self.x;
        let dy = point.y - self.y;
        let dz = point.z - self.z;

        (dx*dx + dy*dy + dz*dz).sqrt()
    }
}

// MARK: OptionCoordinate

impl Default for OptionCoordinate {
    fn default() -> Self {
        Self {
            x: None,
            y: None,
            z: None,
        }
    }
}

