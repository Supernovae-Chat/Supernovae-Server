use std::fmt::Display;

const BITMASK_21_BITS: u32 = 0b111111111111111111111;
const BITMASK_22_BITS: u32 = 0b1111111111111111111111;

#[derive(Debug, PartialEq, Eq)]
pub enum SemVerError {
    MajorTooLarge,
    MinorTooLarge,
    PatchTooLarge,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SemVer(u64);

impl SemVer {
    pub fn new(major: u32, minor: u32, patch: u32) -> Result<Self, SemVerError> {
        if major > BITMASK_21_BITS {
            return Err(SemVerError::MajorTooLarge);
        } else if minor > BITMASK_21_BITS {
            return Err(SemVerError::MinorTooLarge);
        } else if patch > BITMASK_22_BITS {
            return Err(SemVerError::PatchTooLarge);
        }

        let res: u64 = u64::from(major) << 43 | u64::from(minor) << 22 | u64::from(patch);

        Ok(SemVer(res))
    }

    pub fn major(&self) -> u32 {
        (self.0 >> 43) as u32
    }

    pub fn minor(&self) -> u32 {
        ((self.0 >> 22) & BITMASK_21_BITS as u64) as u32
    }

    pub fn patch(&self) -> u32 {
        (self.0 & BITMASK_22_BITS as u64) as u32
    }

    pub fn set_major(&mut self, major: u32) -> Result<(), SemVerError> {
        if major > BITMASK_21_BITS {
            return Err(SemVerError::MajorTooLarge);
        }
        self.0 &= !(BITMASK_21_BITS as u64) << 43;
        self.0 |= u64::from(major) << 43;
        Ok(())
    }

    pub fn set_minor(&mut self, minor: u32) -> Result<(), SemVerError> {
        if minor > BITMASK_21_BITS {
            return Err(SemVerError::MinorTooLarge);
        }
        self.0 &= !(BITMASK_21_BITS as u64) << 22;
        self.0 |= u64::from(minor) << 22;
        Ok(())
    }

    pub fn set_patch(&mut self, patch: u32) -> Result<(), SemVerError> {
        if patch > BITMASK_22_BITS {
            return Err(SemVerError::PatchTooLarge);
        }
        self.0 &= !BITMASK_22_BITS as u64;
        self.0 |= u64::from(patch);
        Ok(())
    }
}

impl Display for SemVer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major(), self.minor(), self.patch())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SEMVER_1_2_3: SemVer = SemVer(8796101410819);

    #[test]
    fn validate_semver_eq() {
        assert_eq!(SEMVER_1_2_3, SemVer::new(1, 2, 3).unwrap());
    }

    #[test]
    fn validate_semver_ne() {
        assert_ne!(SEMVER_1_2_3, SemVer::new(1, 2, 4).unwrap());
    }

    #[test]
    fn validate_semver_errors() {
        assert_eq!(SemVer::new(2097152, 2, 4).unwrap_err(), SemVerError::MajorTooLarge);
        assert_eq!(SemVer::new(1, 2097152, 4).unwrap_err(), SemVerError::MinorTooLarge);
        assert_eq!(SemVer::new(1, 2, 4194304).unwrap_err(), SemVerError::PatchTooLarge);
    }

    #[test]
    fn validate_semver_valid() {
        SemVer::new(2097151, 2, 4).unwrap();
        SemVer::new(1, 2097151, 4).unwrap();
        SemVer::new(1, 2, 4194303).unwrap();
    }

    #[test]
    fn validate_semver_values() {
        assert_eq!(SEMVER_1_2_3.major(), 1);
        assert_eq!(SEMVER_1_2_3.minor(), 2);
        assert_eq!(SEMVER_1_2_3.patch(), 3);
    }

    #[test]
    fn validate_semver_set_values_valid() {
        let mut semver = SEMVER_1_2_3;

        semver.set_major(3).unwrap();
        semver.set_minor(2).unwrap();
        semver.set_patch(1).unwrap();
    }

    #[test]
    fn validate_semver_set_values_errors() {
        let mut semver = SEMVER_1_2_3;

        assert_eq!(semver.set_major(2097152).unwrap_err(), SemVerError::MajorTooLarge);
        assert_eq!(semver.set_minor(2097152).unwrap_err(), SemVerError::MinorTooLarge);
        assert_eq!(semver.set_patch(4194304).unwrap_err(), SemVerError::PatchTooLarge);
    }
}