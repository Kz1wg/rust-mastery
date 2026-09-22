//! Lesson 02-4: フィールドの公開範囲とAPI
//!
//! `Temperature` と `Inventory` は、不変条件を守るために
//! フィールドを private にし、構築・変更を専用のメソッド経由に限定している。
//!
//! もしフィールドが `pub` だったら（本文 Bad Example のように）、
//! `Temperature { 0: -300.0 }` や `inventory.reserved = 999` のような
//! 直接代入で不変条件を破れてしまう。この演習では、その経路が
//! コンパイルできないこと（`compile_fail`）まで含めて確認する。

const ABSOLUTE_ZERO_CELSIUS: f64 = -273.15;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Temperature(f64);

#[derive(Debug, PartialEq)]
pub struct TemperatureError;

impl Temperature {
    /// 絶対零度（-273.15℃）未満なら `Err` を返す。
    ///
    /// フィールドが private なので、外部から直接構築することはできない:
    ///
    /// ```compile_fail
    /// use ex008_visibility::Temperature;
    ///
    /// let t = Temperature(-300.0); // private field
    /// ```
    pub fn new(celsius: f64) -> Result<Self, TemperatureError> {
        todo!("celsius が絶対零度未満なら Err、そうでなければ Ok(Temperature(celsius)) を返してください")
    }

    pub fn celsius(self) -> f64 {
        self.0
    }

    /// 絶対零度未満に変更しようとした場合、値は変更せず `Err` を返す。
    pub fn set_celsius(&mut self, celsius: f64) -> Result<(), TemperatureError> {
        todo!("celsius が絶対零度未満なら Err を返し、そうでなければ self.0 を更新してください")
    }
}

#[derive(Debug, PartialEq)]
pub struct InventoryError;

/// 在庫。常に `reserved <= quantity` を保つ。
///
/// フィールドが private なので、外部から直接構築・変更することはできない:
///
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
    /// `reserved` は 0 から始まる。
    pub fn new(quantity: u32) -> Self {
        todo!("quantity を受け取り、reserved は 0 で初期化してください")
    }

    pub fn quantity(&self) -> u32 {
        self.quantity
    }

    pub fn reserved(&self) -> u32 {
        self.reserved
    }

    /// 予約されていない在庫数。
    pub fn available(&self) -> u32 {
        todo!("quantity - reserved を返してください")
    }

    /// 在庫を予約する。`reserved` が `quantity` を超える場合は
    /// 何も変更せず `Err` を返す。
    pub fn reserve(&mut self, amount: u32) -> Result<(), InventoryError> {
        todo!("reserved + amount が quantity を超えるなら Err、そうでなければ reserved に加算してください")
    }

    /// 予約を解除する（出荷確定など）。`reserved` を下回る解除はできない。
    pub fn release(&mut self, amount: u32) -> Result<(), InventoryError> {
        todo!("amount が reserved を超えるなら Err、そうでなければ reserved から減算してください")
    }
}
