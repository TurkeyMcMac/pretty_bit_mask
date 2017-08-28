use std::ops::{
    BitOrAssign,
    BitXorAssign,
    BitAndAssign,
    Not,
    BitAnd,
};

/// The trait describing things which can have
/// bit masks applied to them. All integer types
/// (and `bool`) implement this trait.
pub trait BitMaskable
    where Self: Copy +
             BitOrAssign +
             BitXorAssign +
             BitAndAssign +
             Not<Output = Self> +
             BitAnd<Self, Output = Self> +
             Eq,
{
    /// Applies a bit mask.
    fn mask(&mut self, mask: Self);
    
    /// Flips the value of a bit mask.
    fn flip(&mut self, mask: Self);
    
    /// Removes a bit mask.
    fn unmask(&mut self, mask: Self);
    
    /// Checks whether a certain mask is currently applied.
    fn masked(self, mask: Self) -> bool;
}

impl <B> BitMaskable for B
    where B: Copy +
             BitOrAssign +
             BitXorAssign +
             BitAndAssign +
             Not<Output = B> +
             BitAnd<B, Output = B> +
             Eq,
{
    #[inline(always)]
    fn mask(&mut self, mask: B) {
        *self |= mask;
    }
    
    #[inline(always)]
    fn flip(&mut self, mask: B) {
        *self ^= mask;
    }
    
    #[inline(always)]
    fn unmask(&mut self, mask: B) {
        *self &= !mask;
    }
    
    #[inline(always)]
    fn masked(self, mask: B) -> bool {
        self & mask == mask
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn mask_works() {
        let mut m: u32 = 0;
        
        m.mask(128);
        
        assert!(m.masked(128));
        
        m.unmask(128);
        
        assert!(!m.masked(128));

    }
    
    #[test]
    fn flip_works() {
        let mut m: u32 = 0;
        
        m.flip(128);
        
        assert!(m.masked(128));
        
        m.flip(128);
        
        assert!(!m.masked(128));
    }
    
    #[test]
    fn multi_masks_work() {
        let mut m: u32 = 0;
        
        m.mask(1|2|4);
        
        assert!(m.masked(1|2|4));
        
        assert!(m.masked(2|4));
        
        assert!(!m.masked(1|2|4|8));
        
        m.unmask(2|4|8);
        
        assert!(m.masked(1));
    }
}
