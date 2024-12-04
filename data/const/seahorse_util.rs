use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::{self, AssociatedToken},
    token::{self, Mint, Token, TokenAccount},
};


use crate::dot::program::*;
use std::{cell::RefCell, rc::Rc};

use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Deref, Index, IndexMut},
};

// TODO maybe hide the names better? wouldn't want any namespace collisions
// Utility structs, functions, and macros to beautify the generated code a little.

pub struct Mutable<T>(Rc<RefCell<T>>);

impl<T> Mutable<T> {
    pub fn new(obj: T) -> Self {
        Self(Rc::new(RefCell::new(obj)))
    }
}

impl<T> Clone for Mutable<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Deref for Mutable<T> {
    type Target = Rc<RefCell<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Debug> Debug for Mutable<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<T: Default> Default for Mutable<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

pub trait IndexWrapped {
    type Output;

    fn index_wrapped(&self, index: i128) -> &Self::Output;
}

pub trait IndexWrappedMut: IndexWrapped {
    fn index_wrapped_mut(&mut self, index: i128) -> &mut <Self as IndexWrapped>::Output;
}

impl<T> IndexWrapped for Vec<T> {
    type Output = T;

    fn index_wrapped(&self, mut index: i128) -> &Self::Output {
        if index < 0 {
            index += self.len() as i128;
        }

        let index: usize = index.try_into().unwrap();

        self.index(index)
    }
}

impl<T> IndexWrappedMut for Vec<T> {
    fn index_wrapped_mut(&mut self, mut index: i128) -> &mut <Self as IndexWrapped>::Output {
        if index < 0 {
            index += self.len() as i128;
        }

        let index: usize = index.try_into().unwrap();

        self.index_mut(index)
    }
}

impl<T, const N: usize> IndexWrapped for [T; N] {
    type Output = T;

    fn index_wrapped(&self, mut index: i128) -> &Self::Output {
        if index < 0 {
            index += N as i128;
        }

        let index: usize = index.try_into().unwrap();

        self.index(index)
    }
}

impl<T, const N: usize> IndexWrappedMut for [T; N] {
    fn index_wrapped_mut(&mut self, mut index: i128) -> &mut <Self as IndexWrapped>::Output {
        if index < 0 {
            index += N as i128;
        }

        let index: usize = index.try_into().unwrap();

        self.index_mut(index)
    }
}

#[derive(Clone)]
pub struct Empty<T: Clone> {
    pub account: T,
    pub bump: Option<u8>,
}

#[derive(Clone, Debug)]
pub struct ProgramsMap<'info>(pub HashMap<&'static str, AccountInfo<'info>>);

impl<'info> ProgramsMap<'info> {
    pub fn get(&self, name: &'static str) -> AccountInfo<'info> {
        self.0.get(name).unwrap().clone()
    }
}

#[derive(Clone, Debug)]
pub struct WithPrograms<'info, 'entrypoint, A> {
    pub account: &'entrypoint A,
    pub programs: &'entrypoint ProgramsMap<'info>,
}

impl<'info, 'entrypoint, A> Deref for WithPrograms<'info, 'entrypoint, A> {
    type Target = A;

    fn deref(&self) -> &Self::Target {
        &self.account
    }
}

pub type SeahorseAccount<'info, 'entrypoint, A> =
    WithPrograms<'info, 'entrypoint, Box<Account<'info, A>>>;

pub type SeahorseSigner<'info, 'entrypoint> = WithPrograms<'info, 'entrypoint, Signer<'info>>;

#[derive(Clone, Debug)]
pub struct CpiAccount<'info> {
    /// CHECK: CpiAccounts temporarily store AccountInfos.
    pub account_info: AccountInfo<'info>,
    pub is_writable: bool,
    pub is_signer: bool,
    pub seeds: Option<Vec<Vec<u8>>>,
}

#[macro_export]
macro_rules! seahorse_const {($ name: ident, $ value: expr) => {
        macro_rules! $name {() => {
                $value
            };
        }

        pub(crate) use $name;
    };
}

pub trait Loadable {
    type Loaded;

    fn load(stored: Self) -> Self::Loaded;

    fn store(loaded: Self::Loaded) -> Self;
}

macro_rules! Loaded {($ name: ty) => {
        <$name as Loadable>::Loaded
    };
}

pub(crate) use Loaded;

#[macro_export]
macro_rules! assign {($ lval: expr, $ rval: expr) => {{
        let temp = $rval;

        $lval = temp;
    }};
}

#[macro_export]
macro_rules! index_assign {($ lval: expr, $ idx: expr, $ rval: expr) => {
        let temp_rval = $rval;
        let temp_idx = $idx;

        $lval[temp_idx] = temp_rval;
    };
}

pub(crate) use assign;

pub(crate) use index_assign;

pub(crate) use seahorse_const;
