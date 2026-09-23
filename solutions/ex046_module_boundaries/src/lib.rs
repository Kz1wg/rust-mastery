pub mod order {
    pub struct Order {
        id: u32,
        total: u64,
        tax: u64,
    }

    impl Order {
        /// ```compile_fail
        /// use ex046_module_boundaries::order::Order;
        ///
        /// let o = Order::new(1, 1000);
        /// let _ = o.tax; // private field
        /// ```
        ///
        /// ```compile_fail
        /// let _ = ex046_module_boundaries::order::calculate_tax(1000); // private fn
        /// ```
        pub fn new(id: u32, total: u64) -> Self {
            Order {
                id,
                total,
                tax: calculate_tax(total),
            }
        }

        pub fn id(&self) -> u32 {
            self.id
        }

        pub fn total_with_tax(&self) -> u64 {
            self.total + self.tax
        }
    }

    fn calculate_tax(total: u64) -> u64 {
        total / 10
    }
}
