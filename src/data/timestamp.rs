#[repr(transparent)]
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug, serde::Serialize)]
pub struct Timestamp(u32);

impl From<u32> for Timestamp {
    fn from(n: u32) -> Self {
        Self(n)
    }
}

impl Timestamp {
    pub const UNIX_EPOCH: Timestamp = Timestamp(0);
    pub const ONE_DAY: Timestamp = Timestamp(60 * 60 * 24);

    pub fn now() -> Self {
        let secs = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .expect("Traveled back in time")
            .as_secs();
        // as u32 only causes problems after Sun 07 Feb 2106 07:28:15 AM CET
        // but I guess this won't be used after that
        Self(secs as u32)
    }

    pub fn bottoming_sub(self, rhs: Self) -> Self {
        Self(self.0.checked_sub(rhs.0).unwrap_or(0))
    }

    pub fn as_u32(self) -> u32 {
        self.0
    }

    pub fn checked_sub(self, duration: Duration) -> Option<Self> {
        self.0.checked_sub(duration.0).map(Self)
    }

    pub fn diff(self, other: Self) -> Option<Duration> {
        self.0.checked_sub(other.0).map(Duration)
    }

    pub fn native(self) -> i64 {
        i64::from(self.0)
    }
}

#[repr(transparent)]
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug, serde::Serialize, Default)]
pub struct Duration(u32);

impl Duration {
    pub fn seconds(n: u32) -> Self {
        Self(n)
    }

    pub fn minutes(n: u32) -> Option<Self> {
        n.checked_mul(60).map(Self)
    }

    pub fn hours(n: u32) -> Option<Self> {
        n.checked_mul(60).and_then(Self::minutes)
    }

    pub fn days(n: u32) -> Option<Self> {
        n.checked_mul(24).and_then(Self::hours)
    }
}
