
#[derive(Debug, Clone)]
pub struct F32struct {
    pub value: f32
}

#[derive(Debug, Clone)]
pub struct F64struct {
    pub value: f64
}   

fn compare_f32(a: &f32, b: &f32) -> std::cmp::Ordering {
    if a.is_nan() && b.is_nan() {
        std::cmp::Ordering::Equal
    } else if a.is_nan() {
        std::cmp::Ordering::Less
    } else if b.is_nan() {
        std::cmp::Ordering::Greater
    } else {
        a.partial_cmp(b).unwrap()
    }
}

fn compare_f64(a: &f64, b: &f64) -> std::cmp::Ordering {
    if a.is_nan() && b.is_nan() {
        std::cmp::Ordering::Equal
    } else if a.is_nan() {
        std::cmp::Ordering::Less
    } else if b.is_nan() {
        std::cmp::Ordering::Greater
    } else {
        a.partial_cmp(b).unwrap()
    }
}

impl Ord for F32struct {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        compare_f32(&self.value, &other.value)
    }
}   

impl PartialOrd for F32struct {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(compare_f32(&self.value, &other.value))
    }
}

impl Ord for F64struct {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        compare_f64(&self.value, &other.value)
    }
}

impl PartialOrd for F64struct {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(compare_f64(&self.value, &other.value))
    }
}

impl PartialEq for F32struct {
    fn eq(&self, other: &Self) -> bool {
        compare_f32(&self.value, &other.value) == std::cmp::Ordering::Equal
    }
}

impl PartialEq for F64struct {
    fn eq(&self, other: &Self) -> bool {
        compare_f64(&self.value, &other.value) == std::cmp::Ordering::Equal
    }
}

impl Eq for F32struct {}
impl Eq for F64struct {}

impl F32struct {
    pub fn new(value: f32) -> Self {
        Self {
            value
        }
    }
}

impl F64struct {
    pub fn new(value: f64) -> Self {
        Self {
            value
        }
    }
}
