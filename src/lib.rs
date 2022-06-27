#![no_std]

pub mod concat {
    use seq_macro::seq;

    #[inline]
    pub const fn c14(status: u8, first: u32) -> [u8; 5] {
        let first = first.to_be_bytes();
        seq!(N in 0..=3 {
            [
                status,
                #(first[N],)*
            ]
        })
    }

    #[inline]
    pub const fn c41(first: u32, status: u8) -> [u8; 5] {
        let first = first.to_be_bytes();
        seq!(N in 0..=3 {
            [
                #(first[N],)*
                status
            ]
        })
    }

    #[inline]
    pub const fn c44(first: u32, second: u32) -> [u8; 8] {
        let first = first.to_be_bytes();
        let second = second.to_be_bytes();
        seq!(N in 0..=3 {
            [
                #(first[N],)*
                #(second[N],)*
            ]
        })
    }

    #[inline]
    pub const fn c144(status: u8, first: u32, second: u32) -> [u8; 9] {
        let first = first.to_be_bytes();
        let second = second.to_be_bytes();
        seq!(N in 0..=3 {
            [
                status,
                #(first[N],)*
                #(second[N],)*
            ]
        })
    }

    #[inline]
    pub const fn c414(first: u32, status: u8, second: u32) -> [u8; 9] {
        let first = first.to_be_bytes();
        let second = second.to_be_bytes();
        seq!(N in 0..=3 {
            [
                #(first[N],)*
                status,
                #(second[N],)*
            ]
        })
    }

    #[inline]
    pub const fn c441(first: u32, second: u32, status: u8) -> [u8; 9] {
        let first = first.to_be_bytes();
        let second = second.to_be_bytes();
        seq!(N in 0..=3 {
            [
                #(first[N],)*
                #(second[N],)*
                status,
            ]
        })
    }

    #[inline]
    pub const fn c444(first: u32, second: u32, a3: u32) -> [u8; 12] {
        let first = first.to_be_bytes();
        let second = second.to_be_bytes();
        let a3 = a3.to_be_bytes();
        seq!(N in 0..=3 {
            [
                #(first[N],)*
                #(second[N],)*
                #(a3[N],)*
            ]
        })
    }

    #[inline]
    pub const fn c4414(first: u32, second: u32, status: u8, a3: u32) -> [u8; 13] {
        let first = first.to_be_bytes();
        let second = second.to_be_bytes();
        let a3 = a3.to_be_bytes();
        seq!(N in 0..=3 {
            [
                #(first[N],)*
                #(second[N],)*
                status,
                #(a3[N],)*
            ]
        })
    }

    #[inline]
    pub const fn c4441(first: u32, second: u32, a3: u32, status: u8) -> [u8; 13] {
        let first = first.to_be_bytes();
        let second = second.to_be_bytes();
        let a3 = a3.to_be_bytes();
        seq!(N in 0..=3 {
            [
                #(first[N],)*
                #(second[N],)*
                #(a3[N],)*
                status,
            ]
        })
    }

    #[inline]
    pub const fn c4444(first: u32, second: u32, a3: u32, a4: u32) -> [u8; 16] {
        let first = first.to_be_bytes();
        let second = second.to_be_bytes();
        let a3 = a3.to_be_bytes();
        let a4 = a4.to_be_bytes();
        seq!(N in 0..=3 {
        [
            #(first[N],)*
            #(second[N],)*
            #(a3[N],)*
            #(a4[N],)*
        ]
        })
    }

    #[inline]
    pub const fn c44144(first: u32, second: u32, status: u8, a3: u32, a4: u32) -> [u8; 17] {
        let first = first.to_be_bytes();
        let second = second.to_be_bytes();
        let a3 = a3.to_be_bytes();
        let a4 = a4.to_be_bytes();
        seq!(N in 0..=3 {
            [
                #(first[N],)*
                #(second[N],)*
                status,
                #(a3[N],)*
                #(a4[N],)*
            ]
        })
    }

    #[inline]
    pub const fn c44414(first: u32, second: u32, a3: u32, status: u8, a4: u32) -> [u8; 17] {
        let first = first.to_be_bytes();
        let second = second.to_be_bytes();
        let a3 = a3.to_be_bytes();
        let a4 = a4.to_be_bytes();
        seq!(N in 0..=3 {
            [
                #(first[N],)*
                #(second[N],)*
                #(a3[N],)*
                status,
                #(a4[N],)*
            ]
        })
    }

    #[inline]
    pub const fn c444144(a1: u32, a2: u32, a3: u32, status: u8, a4: u32, a5: u32) -> [u8; 21] {
        let a1 = a1.to_be_bytes();
        let a2 = a2.to_be_bytes();
        let a3 = a3.to_be_bytes();
        let a4 = a4.to_be_bytes();
        let a5 = a5.to_be_bytes();
        seq!(N in 0..=3 {
            [
                #(a1[N],)*
                #(a2[N],)*
                #(a3[N],)*
                status,
                #(a4[N],)*
                #(a5[N],)*
            ]
        })
    }
}

pub mod from {
    use seq_macro::seq;

    #[inline]
    pub const fn f14(bytes: [u8; 5]) -> (u8, u32) {
        let status = bytes[0];
        seq!(N in 1..5 {
            let a1 = u32::from_be_bytes([ #(bytes[N],)* ]);
        });
        (status, a1)
    }

    #[inline]
    pub const fn f41(bytes: [u8; 5]) -> (u32, u8) {
        seq!(N in 0..4 {
            let a1 = u32::from_be_bytes([ #(bytes[N],)* ]);
        });
        let status = bytes[4];
        (a1, status)
    }

    #[inline]
    pub const fn f44(bytes: [u8; 8]) -> (u32, u32) {
        seq!(N in 0..4 {
            let a1 = u32::from_be_bytes([ #(bytes[N],)* ]);
        });
        seq!(N in 4..8 {
            let a2 = u32::from_be_bytes([ #(bytes[N],)* ]);
        });
        (a1, a2)
    }

    #[inline]
    pub const fn f144(bytes: [u8; 9]) -> (u8, u32, u32) {
        let status = bytes[0];
        seq!(N in 1..5 {
            let a1 = u32::from_be_bytes([ #(bytes[N],)* ]);
        });
        seq!(N in 5..9 {
            let a2 = u32::from_be_bytes([ #(bytes[N],)* ]);
        });
        (status, a1, a2)
    }

    #[inline]
    pub const fn f414(bytes: [u8; 9]) -> (u32, u8, u32) {
        seq!(N in 0..4 {
            let a1 = u32::from_be_bytes([ #(bytes[N],)* ]);
        });
        let status = bytes[4];
        seq!(N in 5..9 {
            let a2 = u32::from_be_bytes([ #(bytes[N],)* ]);
        });
        (a1, status, a2)
    }

    #[inline]
    pub const fn f441(bytes: [u8; 9]) -> (u32, u32, u8) {
        seq!(N in 0..4 {
            let a1 = u32::from_be_bytes([ #(bytes[N],)* ]);
        });
        seq!(N in 4..8 {
            let a2 = u32::from_be_bytes([ #(bytes[N],)* ]);
        });
        let status = bytes[8];
        (a1, a2, status)
    }

    #[inline]
    pub const fn f444(bytes: [u8; 12]) -> (u32, u32, u32) {
        seq!(N in 0..4 {
            let a1 = u32::from_be_bytes([ #(bytes[N],)* ]);
        });
        seq!(N in 4..8 {
            let a2 = u32::from_be_bytes([ #(bytes[N],)* ]);
        });
        seq!(N in 8..12 {
            let a3 = u32::from_be_bytes([ #(bytes[N],)* ]);
        });
        (a1, a2, a3)
    }

    #[inline]
    pub const fn f4441(bytes: [u8; 13]) -> (u32, u32, u32, u8) {
        seq!(N in 0..4 {
            let a1 = u32::from_be_bytes([ #(bytes[N],)* ]);
        });
        seq!(N in 4..8 {
            let a2 = u32::from_be_bytes([ #(bytes[N],)* ]);
        });
        seq!(N in 8..12 {
            let a3 = u32::from_be_bytes([ #(bytes[N],)* ]);
        });
        let status = bytes[12];
        (a1, a2, a3, status)
    }

    #[inline]
    pub const fn f4414(bytes: [u8; 13]) -> (u32, u32, u8, u32) {
        seq!(N in 0..4 {
            let a1 = u32::from_be_bytes([ #(bytes[N],)* ]);
        });
        seq!(N in 4..8 {
            let a2 = u32::from_be_bytes([ #(bytes[N],)* ]);
        });
        let status = bytes[8];
        seq!(N in 9..13 {
            let a3 = u32::from_be_bytes([ #(bytes[N],)* ]);
        });
        (a1, a2, status, a3)
    }

    #[inline]
    pub const fn f4444(bytes: [u8; 16]) -> (u32, u32, u32, u32) {
        seq!(N in 0..4 {
            let a1 = u32::from_be_bytes([ #(bytes[N],)* ]);
        });
        seq!(N in 4..8 {
            let a2 = u32::from_be_bytes([ #(bytes[N],)* ]);
        });
        seq!(N in 8..12 {
            let a3 = u32::from_be_bytes([ #(bytes[N],)* ]);
        });
        seq!(N in 12..16 {
            let a4 = u32::from_be_bytes([ #(bytes[N],)* ]);
        });
        (a1, a2, a3, a4)
    }

    #[inline]
    pub const fn f44144(bytes: [u8; 17]) -> (u32, u32, u8, u32, u32) {
        seq!(N in 0..4 {
            let a1 = u32::from_be_bytes([ #(bytes[N],)* ]);
        });
        seq!(N in 4..8 {
            let a2 = u32::from_be_bytes([ #(bytes[N],)* ]);
        });
        let status = bytes[8];
        seq!(N in 9..13 {
            let a3 = u32::from_be_bytes([ #(bytes[N],)* ]);
        });
        seq!(N in 13..17 {
            let a4 = u32::from_be_bytes([ #(bytes[N],)* ]);
        });
        (a1, a2, status, a3, a4)
    }

    #[inline]
    pub const fn f444144(bytes: [u8; 21]) -> (u32, u32, u32, u8, u32, u32) {
        seq!(N in 0..4 {
            let a1 = u32::from_be_bytes([ #(bytes[N],)* ]);
        });
        seq!(N in 4..8 {
            let a2 = u32::from_be_bytes([ #(bytes[N],)* ]);
        });
        seq!(N in 8..12 {
            let a3 = u32::from_be_bytes([ #(bytes[N],)* ]);
        });
        let status = bytes[12];
        seq!(N in 13..17 {
            let a4 = u32::from_be_bytes([ #(bytes[N],)* ]);
        });
        seq!(N in 17..21 {
            let a5 = u32::from_be_bytes([ #(bytes[N],)* ]);
        });
        (a1, a2, a3, status, a4, a5)
    }
}

pub use concat::*;
pub use from::*;
