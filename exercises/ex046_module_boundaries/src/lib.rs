//! Lesson 12-1: module分割の基準
//!
//! order モジュールが外に見せるのは Order::new / id / total_with_tax だけ。
//! フィールドと内部計算（calculate_tax）は隠す。

pub mod order {
    pub struct Order {
        id: u32,
        total: u64,
        tax: u64,
    }

    impl Order {
        /// 構築時に税を計算する。以降 tax は常に total の10%になる。
        ///
        /// フィールドは外から触れない:
        ///
        /// ```compile_fail
        /// use ex046_module_boundaries::order::Order;
        ///
        /// let o = Order::new(1, 1000);
        /// let _ = o.tax; // private field
        /// ```
        ///
        /// 内部の計算関数も見えない:
        ///
        /// ```compile_fail
        /// let _ = ex046_module_boundaries::order::calculate_tax(1000); // private fn
        /// ```
        pub fn new(id: u32, total: u64) -> Self {
            todo!("tax を calculate_tax(total) で求めて Order を作ってください")
        }

        pub fn id(&self) -> u32 {
            todo!("id を返してください")
        }

        pub fn total_with_tax(&self) -> u64 {
            todo!("total と tax の合計を返してください")
        }
    }

    /// 内部計算。モジュールの外からは見えない。
    fn calculate_tax(total: u64) -> u64 {
        total / 10
    }
}
