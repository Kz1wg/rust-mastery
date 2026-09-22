const ABSOLUTE_ZERO_CELSIUS: f64 = -273.15;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Temperature(f64);

#[derive(Debug, PartialEq)]
pub struct TemperatureError;

impl Temperature {
    /// ```compile_fail
    /// use ex008_visibility::Temperature;
    ///
    /// let t = Temperature(-300.0); // private field
    /// ```
    pub fn new(celsius: f64) -> Result<Self, TemperatureError> {
        if celsius < ABSOLUTE_ZERO_CELSIUS {
            Err(TemperatureError)
        } else {
            Ok(Temperature(celsius))
        }
    }

    pub fn celsius(self) -> f64 {
        self.0
    }

    pub fn set_celsius(&mut self, celsius: f64) -> Result<(), TemperatureError> {
        if celsius < ABSOLUTE_ZERO_CELSIUS {
            Err(TemperatureError)
        } else {
            self.0 = celsius;
            Ok(())
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct InventoryError;

/// ```compile_fail
/// use ex008_visibility::Inventory;
///
/// let inv = Inventory { quantity: 10, reserved: 20 }; // private field
/// ```
pub struct Inventory {
    quantity: u32,
    reserved: u32,
}

impl Inventory {
    pub fn new(quantity: u32) -> Self {
        Inventory {
            quantity,
            reserved: 0,
        }
    }

    pub fn quantity(&self) -> u32 {
        self.quantity
    }

    pub fn reserved(&self) -> u32 {
        self.reserved
    }

    pub fn available(&self) -> u32 {
        self.quantity - self.reserved
    }

    pub fn reserve(&mut self, amount: u32) -> Result<(), InventoryError> {
        if self.reserved + amount > self.quantity {
            Err(InventoryError)
        } else {
            self.reserved += amount;
            Ok(())
        }
    }

    pub fn release(&mut self, amount: u32) -> Result<(), InventoryError> {
        if amount > self.reserved {
            Err(InventoryError)
        } else {
            self.reserved -= amount;
            Ok(())
        }
    }
}
