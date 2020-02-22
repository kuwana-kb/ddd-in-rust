/// 物流拠点
pub struct PhysicalDistributionBase {}

impl PhysicalDistributionBase {
    /// 出庫
    pub fn ship(&self, baggage: Baggage) -> Baggage {
        unimplemented!()
    }
    /// 入庫
    pub fn receive(&self, baggage: Baggage) {
        unimplemented!()
    }

    // 輸送(出庫 -> 入庫)
    // 物流拠点が輸送のふるまいを持つのは不自然
    // 入出庫の記録等も物流拠点が担うことになりそう
    // pub fn transport(&self, to: &mut PhysicalDistributionBase, baggage: Baggage) {
    //     let shipped_baggage = self.ship(baggage);
    //     to.receive(shipped_baggage);
    // }
}

pub struct TransportService {}

impl TransportService {
    pub fn transport(
        from: PhysicalDistributionBase,
        to: PhysicalDistributionBase,
        baggage: Baggage,
    ) {
        let shipped_baggage = from.ship(baggage);
        to.receive(shipped_baggage);

        // 配送の記録等を行う..
    }
}

pub struct Baggage {}
